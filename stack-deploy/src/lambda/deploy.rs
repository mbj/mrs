use crate::instance_spec::InstanceSpec;
use crate::types::{ParameterKey, ParameterMap, ParameterValue};
use sha2::Digest;

pub struct BinaryName(pub String);
pub struct BuildTarget(pub String);
pub struct S3ObjectKey(pub String);

impl From<&S3ObjectKey> for String {
    fn from(value: &S3ObjectKey) -> String {
        value.0.clone()
    }
}

#[derive(Clone)]
pub struct S3BucketName(pub String);

impl From<&S3BucketName> for String {
    fn from(value: &S3BucketName) -> String {
        value.0.clone()
    }
}

pub enum BuildType {
    Debug,
    Release,
}

impl BuildType {
    fn path(&self) -> &str {
        match self {
            Self::Debug => "debug",
            Self::Release => "release",
        }
    }

    fn args(&self) -> &[&str] {
        match self {
            Self::Debug => &[],
            Self::Release => &["--release"],
        }
    }
}

pub enum S3BucketSource {
    Static(S3BucketName),
    StackOutput {
        stack_name: crate::types::StackName,
        output_key: crate::types::OutputKey,
    },
}

pub struct ZipFile {
    body: aws_sdk_s3::primitives::ByteStream,
    s3_object_key: S3ObjectKey,
}

impl ZipFile {
    pub async fn upload(
        self,
        s3: &aws_sdk_s3::client::Client,
        s3_bucket_name: &S3BucketName,
    ) -> ParameterValue {
        if !self.object_exists(s3, s3_bucket_name).await {
            s3.put_object()
                .bucket(s3_bucket_name)
                .key(&self.s3_object_key)
                .body(self.body)
                .send()
                .await
                .unwrap();
        }

        ParameterValue(self.s3_object_key.0.clone())
    }

    async fn object_exists(
        &self,
        s3: &aws_sdk_s3::client::Client,
        s3_bucket_name: &S3BucketName,
    ) -> bool {
        let result = s3
            .head_object()
            .bucket(s3_bucket_name)
            .key(&self.s3_object_key.0)
            .send()
            .await;

        match result {
            Err(error) => match error.into_service_error() {
                aws_sdk_s3::operation::head_object::HeadObjectError::NotFound { .. } => false,
                other => panic!("Unexpected head object error: {other:#?}"),
            },
            Ok(_) => true,
        }
    }
}

pub struct Target {
    pub binary_name: BinaryName,
    pub build_target: BuildTarget,
    pub build_type: BuildType,
}

impl Target {
    /// Path to target
    ///
    /// ### Examples
    ///
    /// ```
    /// # use crate::stack_deploy::lambda::deploy::*;
    ///
    /// assert_eq!(
    ///     std::path::PathBuf::from("./target/example-target/debug/example-binary"),
    ///     Target {
    ///         binary_name: BinaryName(String::from("example-binary")),
    ///         build_target: BuildTarget(String::from("example-target")),
    ///         build_type: BuildType::Debug
    ///     }
    ///     .path()
    /// )
    /// ```
    pub fn path(&self) -> std::path::PathBuf {
        std::path::Path::new("./target")
            .join(&self.build_target.0)
            .join(self.build_type.path())
            .join(&self.binary_name.0)
    }

    /// Build the lambda target via cargo
    pub fn build(&self) {
        eprintln!("Building lambda target");
        assert!(
            std::process::Command::new("cargo")
                .args(["build", "--target", &self.build_target.0])
                .args(self.build_type.args())
                .status()
                .unwrap()
                .success()
        );
    }

    /// Build the lambda target and generate zip file
    pub fn build_zip(&self) -> ZipFile {
        self.build();
        self.generate_zip()
    }

    /// Generate the zip file from target read from ./target
    fn generate_zip(&self) -> ZipFile {
        let path = self.path();
        eprintln!("Reading binary from: {}", path.display());

        let binary = std::fs::read(path).unwrap();

        eprintln!("Compressing binary into zip");

        let mut cursor: std::io::Cursor<Vec<u8>> = std::io::Cursor::new(vec![]);
        let mut zip = zip::write::ZipWriter::new(&mut cursor);
        zip.start_file::<&str, ()>(
            "bootstrap",
            zip::write::FileOptions::default()
                .unix_permissions(0o555)
                // We want byte wise deterministic zip files
                // The default is to encode the current time which
                // would not be stable cross runs on unmodified binary.
                .last_modified_time(zip::DateTime::default()),
        )
        .unwrap();
        std::io::Write::write_all(&mut zip, binary.as_ref()).unwrap();
        zip.finish().unwrap();

        eprintln!("Computing zip hash");
        let body = cursor.into_inner();
        let hash = hex::encode(sha2::Sha256::digest(&body).as_slice());
        eprintln!("Content hash: {hash}");

        ZipFile {
            body: aws_sdk_s3::primitives::ByteStream::from(body),
            s3_object_key: S3ObjectKey(format!("{hash}.zip")),
        }
    }

    pub async fn deploy_parameter_update(
        s3: &aws_sdk_s3::client::Client,
        cloudformation: &aws_sdk_cloudformation::Client,
        s3_bucket_name: &S3BucketName,
        instance_spec: &InstanceSpec,
        parameter_key: &ParameterKey,
        zip_file: ZipFile,
    ) {
        let parameter_value = zip_file.upload(s3, s3_bucket_name).await;

        instance_spec
            .parameter_update(
                cloudformation,
                &ParameterMap(std::collections::BTreeMap::from([(
                    parameter_key.clone(),
                    parameter_value,
                )])),
            )
            .await;
    }

    pub async fn deploy_template_update(
        s3: &aws_sdk_s3::client::Client,
        cloudformation: &aws_sdk_cloudformation::Client,
        s3_bucket_name: &S3BucketName,
        instance_spec: &InstanceSpec,
        parameter_key: &ParameterKey,
        zip_file: ZipFile,
    ) {
        let parameter_value = zip_file.upload(s3, s3_bucket_name).await;

        instance_spec
            .update(
                cloudformation,
                &ParameterMap(std::collections::BTreeMap::from([(
                    parameter_key.clone(),
                    parameter_value,
                )])),
            )
            .await;
    }
}

pub mod cli {
    use crate::instance_spec::Registry;
    use crate::lambda::deploy::S3BucketSource;
    use crate::types::{ParameterKey, ParameterMap, ParameterValue, StackName};

    #[derive(Clone, Debug, clap::Parser)]
    pub struct App {
        #[clap(subcommand)]
        command: Command,
    }

    impl App {
        pub async fn run(&self, config: &'_ Config<'_>) {
            self.command.run(config).await
        }
    }

    pub struct Config<'a> {
        pub cloudformation: &'a aws_sdk_cloudformation::client::Client,
        pub parameter_key: ParameterKey,
        pub registry: Registry,
        pub s3: &'a aws_sdk_s3::client::Client,
        pub s3_bucket_source: S3BucketSource,
        pub target: crate::lambda::deploy::Target,
    }

    impl Config<'_> {
        pub(crate) fn build(&self) {
            self.target.build()
        }

        pub(crate) async fn upload(&self) -> ParameterValue {
            let s3_bucket_name = self.load_s3_bucket_name().await;

            let parameter_value = self
                .target
                .build_zip()
                .upload(self.s3, &s3_bucket_name)
                .await;

            eprintln!("Lambda object key: {}", parameter_value.0);

            parameter_value
        }

        pub(crate) async fn deploy_template(&self, stack_name: &StackName) {
            let instance_spec = self
                .registry
                .find(stack_name)
                .expect("instance spec not registered");

            let parameter_value = self.upload().await;

            instance_spec
                .sync(
                    self.cloudformation,
                    &ParameterMap(std::collections::BTreeMap::from([(
                        self.parameter_key.clone(),
                        parameter_value,
                    )])),
                )
                .await
        }

        pub(crate) async fn deploy_parameter(&self, stack_name: &StackName) {
            let instance_spec = self
                .registry
                .find(stack_name)
                .expect("instance spec not registered");

            let parameter_value = self.upload().await;

            instance_spec
                .parameter_update(
                    self.cloudformation,
                    &ParameterMap(std::collections::BTreeMap::from([(
                        self.parameter_key.clone(),
                        parameter_value,
                    )])),
                )
                .await
        }

        async fn load_s3_bucket_name(&self) -> crate::lambda::deploy::S3BucketName {
            match &self.s3_bucket_source {
                S3BucketSource::StackOutput {
                    stack_name,
                    output_key,
                } => crate::lambda::deploy::S3BucketName(
                    crate::stack::read_stack_output(self.cloudformation, stack_name, output_key)
                        .await,
                ),
                S3BucketSource::Static(s3_bucket_name) => s3_bucket_name.clone(),
            }
        }
    }

    #[derive(Clone, Debug, clap::Parser)]
    pub enum Command {
        /// Deploy lambda function with template update
        DeployTemplate {
            /// Instance spec name to deploy to
            #[arg(long = "stack-name")]
            name: StackName,
        },
        /// Deploy lambda function with parameter update
        DeployParameter {
            /// Instance spec name to deploy to
            #[arg(long = "stack-name")]
            name: StackName,
        },
        /// Build lambda function
        Build,
        /// Upload lambda function
        Upload,
    }

    impl Command {
        pub async fn run(&self, config: &'_ Config<'_>) {
            match self {
                Self::Build => config.build(),
                Self::Upload => {
                    config.upload().await;
                }
                Self::DeployTemplate { name } => config.deploy_template(name).await,
                Self::DeployParameter { name } => config.deploy_parameter(name).await,
            }
        }
    }
}
