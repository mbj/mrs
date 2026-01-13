use crate::stack::{load_stack, try_load_stack};
use crate::types::*;
use std::collections::BTreeSet;

pub(crate) struct RemoteOperation {
    pub(crate) client_request_token: ClientRequestToken,
    pub(crate) stack_id: StackId,
}

#[derive(Clone, Debug, Eq, PartialEq, clap::ValueEnum)]
pub enum ReviewChangeSet {
    Interactive,
    NoReview,
}

pub type Capabilities = std::collections::BTreeSet<aws_sdk_cloudformation::types::Capability>;

pub struct TemplateUploader<'a> {
    pub cloudformation: &'a aws_sdk_cloudformation::Client,
    pub s3: &'a aws_sdk_s3::Client,
    pub s3_bucket_name_output_key: &'a OutputKey,
    pub stack_name: &'a StackName,
}

impl TemplateUploader<'_> {
    async fn upload(&self, template_rendered: &TemplateRendered) -> TemplateUrl {
        let hex =
            hex::encode(<sha2::Sha256 as sha2::Digest>::digest(&template_rendered.body).as_slice());

        let s3_bucket_name = crate::stack::read_stack_output(
            self.cloudformation,
            self.stack_name,
            self.s3_bucket_name_output_key,
        )
        .await;

        let s3_object_key = format!("{hex}.{}", template_rendered.format.file_ext());

        log::info!("Uploading template to: {s3_bucket_name}/{s3_object_key}");

        self.s3
            .put_object()
            .bucket(&s3_bucket_name)
            .key(&s3_object_key)
            .body((&template_rendered.body).into())
            .send()
            .await
            .unwrap();

        format!("https://s3.amazonaws.com/{s3_bucket_name}/{s3_object_key}").into()
    }
}

#[derive(Debug)]
pub struct InstanceSpec {
    pub capabilities: Capabilities,
    pub stack_name: StackName,
    pub parameter_map: ParameterMap,
    pub template: Template,
}

impl InstanceSpec {
    #[must_use]
    pub fn context<'a>(
        &'a self,
        cloudformation: &'a aws_sdk_cloudformation::Client,
        template_uploader: Option<&'a TemplateUploader<'a>>,
    ) -> Context<'a> {
        Context {
            cloudformation,
            instance_spec: self,
            template_uploader,
        }
    }

    pub(crate) async fn watch(cloudformation: &aws_sdk_cloudformation::Client, stack_id: StackId) {
        crate::events::Poll::default(stack_id)
            .run(cloudformation, |stack_event| {
                crate::events::print_event(stack_event)
            })
            .await;
    }
}

pub struct Context<'a> {
    cloudformation: &'a aws_sdk_cloudformation::Client,
    instance_spec: &'a InstanceSpec,
    template_uploader: Option<&'a TemplateUploader<'a>>,
}

impl Context<'_> {
    pub async fn create_change_set(
        &self,
        change_set_name: &ChangeSetName,
        user_parameter_map: &ParameterMap,
    ) {
        let existing_stack = load_stack(self.cloudformation, &self.instance_spec.stack_name).await;

        let change_set_arn = self
            .start_create_change_set(&existing_stack, change_set_name, user_parameter_map)
            .await;

        println!("ChangeSetArn: {}", change_set_arn.as_str());
    }

    pub async fn delete_change_set(&self, change_set_name: &ChangeSetName) {
        self.cloudformation
            .delete_change_set()
            .change_set_name(change_set_name)
            .stack_name(self.instance_spec.stack_name.as_str())
            .send()
            .await
            .unwrap();
    }

    pub async fn describe_change_set(&self, change_set_name: &ChangeSetName) {
        let output = self
            .cloudformation
            .describe_change_set()
            .change_set_name(change_set_name)
            .stack_name(self.instance_spec.stack_name.as_str())
            .send()
            .await
            .unwrap();

        crate::change_set::print_change_set_output(&output);
    }

    pub async fn list_change_sets(&self) {
        let mut paginator = self
            .cloudformation
            .list_change_sets()
            .stack_name(self.instance_spec.stack_name.as_str())
            .into_paginator()
            .send();

        while let Some(result) = paginator.next().await {
            for summary in result.unwrap().summaries.unwrap_or_default() {
                eprintln!("{summary:#?}")
            }
        }
    }

    pub async fn delete(&self) {
        let client_request_token = ClientRequestToken::generate();
        let stack_id =
            crate::stack::load_stack_id(self.cloudformation, &self.instance_spec.stack_name).await;

        self.cloudformation
            .delete_stack()
            .client_request_token(&client_request_token)
            .stack_name(self.instance_spec.stack_name.as_str())
            .send()
            .await
            .unwrap();

        Self::wait(
            self.cloudformation,
            RemoteOperation {
                client_request_token,
                stack_id,
            },
        )
        .await
    }

    pub async fn create(&self, user_parameter_map: &ParameterMap) {
        self.wait_for_final(self.start_create(user_parameter_map).await)
            .await
    }

    pub async fn sync(
        &self,
        review_change_set: &ReviewChangeSet,
        user_parameter_map: &ParameterMap,
    ) {
        match try_load_stack(self.cloudformation, &self.instance_spec.stack_name).await {
            Some(existing_stack) => {
                self.update_existing_stack(review_change_set, &existing_stack, user_parameter_map)
                    .await
            }
            None => self.create(user_parameter_map).await,
        }
    }

    pub async fn update(
        &self,
        review_change_set: &ReviewChangeSet,
        user_parameter_map: &ParameterMap,
    ) {
        let existing_stack = load_stack(self.cloudformation, &self.instance_spec.stack_name).await;
        self.update_existing_stack(review_change_set, &existing_stack, user_parameter_map)
            .await
    }

    async fn update_existing_stack(
        &self,
        review_change_set: &ReviewChangeSet,
        existing_stack: &aws_sdk_cloudformation::types::Stack,
        user_parameter_map: &ParameterMap,
    ) {
        match review_change_set {
            ReviewChangeSet::Interactive => {
                self.update_interactive(existing_stack, user_parameter_map)
                    .await
            }
            ReviewChangeSet::NoReview => {
                self.wait_for_final_update(
                    self.start_template_update(existing_stack, user_parameter_map)
                        .await,
                )
                .await
            }
        }
    }

    async fn update_interactive(
        &self,
        existing_stack: &aws_sdk_cloudformation::types::Stack,
        user_parameter_map: &ParameterMap,
    ) {
        use std::io::BufRead;

        let change_set_name =
            format!("interactive-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")).into();

        log::info!("Creating change set: {change_set_name}");

        let change_set_arn = self
            .start_create_change_set(existing_stack, &change_set_name, user_parameter_map)
            .await;

        log::info!("Created change set: {change_set_arn}");

        self.wait_for_final_change_set_status(&change_set_arn).await;

        let output = self
            .cloudformation
            .describe_change_set()
            .change_set_name(&change_set_arn)
            .include_property_values(true)
            .send()
            .await
            .unwrap();

        let stack_id: StackId = StackId(output.stack_id.clone().unwrap());

        crate::change_set::print_change_set_output(&output);

        println!("Apply? Type YES to proceed or send SIGERM");

        for line in std::io::stdin().lock().lines() {
            if line.unwrap() == "YES" {
                break;
            } else {
                println!("Only YES please ;)")
            }
        }

        let client_request_token = ClientRequestToken::generate();

        self.cloudformation
            .execute_change_set()
            .change_set_name(change_set_arn)
            .client_request_token(&client_request_token)
            .send()
            .await
            .unwrap();

        self.wait_for_final(RemoteOperation {
            stack_id,
            client_request_token,
        })
        .await
    }

    async fn wait_for_final_update(&self, result: Option<RemoteOperation>) {
        match result {
            None => log::info!("Stack is already up to date"),
            Some(remote_operation) => self.wait_for_final(remote_operation).await,
        }
    }

    async fn wait_for_final(&self, remote_operation: RemoteOperation) {
        Self::wait(self.cloudformation, remote_operation).await
    }

    pub async fn parameter_update(&self, user_parameter_map: &ParameterMap) {
        let result = self
            .start_parameter_update(
                self.cloudformation,
                &load_stack(self.cloudformation, &self.instance_spec.stack_name).await,
                user_parameter_map,
            )
            .await;

        match result {
            None => log::info!("Stack is already up to date"),
            Some(remote_operation) => Self::wait(self.cloudformation, remote_operation).await,
        }
    }

    async fn wait(
        cloudformation: &aws_sdk_cloudformation::Client,
        remote_operation: RemoteOperation,
    ) {
        crate::events::Poll::wait_for_remote_operation(remote_operation)
            .run(cloudformation, crate::events::print_event)
            .await
    }

    async fn wait_for_final_change_set_status(&self, change_set_arn: &ChangeSetArn) {
        use aws_sdk_cloudformation::types::ChangeSetStatus;

        enum Iteration {
            Continue,
            Stop,
        }

        loop {
            let output = self
                .cloudformation
                .describe_change_set()
                .change_set_name(change_set_arn)
                .include_property_values(true)
                .send()
                .await
                .unwrap();

            let status = output.status.unwrap();

            let (log_level, panic_message, iteration) = match status {
                ChangeSetStatus::CreateComplete | ChangeSetStatus::DeleteComplete => {
                    (log::Level::Info, None, Iteration::Stop)
                }
                ChangeSetStatus::CreateInProgress
                | ChangeSetStatus::CreatePending
                | ChangeSetStatus::DeleteInProgress
                | ChangeSetStatus::DeletePending => (log::Level::Info, None, Iteration::Continue),
                ChangeSetStatus::DeleteFailed => (
                    log::Level::Error,
                    Some("Failed to delete change set"),
                    Iteration::Stop,
                ),
                ChangeSetStatus::Failed => (
                    log::Level::Error,
                    Some("Failed to create change set"),
                    Iteration::Stop,
                ),
                unknown => panic!("Unknown change set status: {unknown:#?}"),
            };

            log::log!(
                log_level,
                "{change_set_arn}: {status}, {}",
                match output.status_reason.as_ref() {
                    Some(reason) => reason.as_str(),
                    None => "",
                }
            );

            if let Some(panic_message) = panic_message {
                panic!("{panic_message}");
            }

            match iteration {
                Iteration::Continue => {}
                Iteration::Stop => break,
            }

            tokio::time::sleep(std::time::Duration::new(1, 0)).await
        }
    }

    async fn start_create(&self, user_parameter_map: &ParameterMap) -> RemoteOperation {
        let client_request_token = ClientRequestToken::generate();

        let request = self
            .cloudformation
            .create_stack()
            .stack_name(self.instance_spec.stack_name.as_str())
            .set_parameters(Some(
                self.instance_spec
                    .parameter_map
                    .merge(user_parameter_map)
                    .to_create_parameters(),
            ))
            .set_capabilities(Some(self.capabilities()))
            .client_request_token(&client_request_token);

        let stack_id = StackId(
            self.set_create_template(request)
                .await
                .send()
                .await
                .unwrap()
                .stack_id
                .unwrap(),
        );

        RemoteOperation {
            client_request_token,
            stack_id,
        }
    }

    pub async fn start_create_change_set(
        &self,
        existing_stack: &aws_sdk_cloudformation::types::Stack,
        change_set_name: &ChangeSetName,
        user_parameter_map: &ParameterMap,
    ) -> ChangeSetArn {
        let client_request_token = ClientRequestToken::generate();

        let existing_stack_parameter_keys = existing_stack_parameter_keys(existing_stack);

        let request = self
            .cloudformation
            .create_change_set()
            .change_set_name(change_set_name)
            .stack_name(existing_stack.stack_id.as_ref().unwrap())
            .set_parameters(Some(
                self.instance_spec
                    .parameter_map
                    .merge(user_parameter_map)
                    .to_template_update_parameters(
                        &self.instance_spec.template.parameter_keys(),
                        &existing_stack_parameter_keys,
                    ),
            ))
            .set_capabilities(Some(self.capabilities()))
            .client_token(&client_request_token);

        let output = self
            .set_create_change_set_template(request)
            .await
            .send()
            .await
            .unwrap();

        ChangeSetArn(output.id.unwrap())
    }

    async fn start_template_update(
        &self,
        existing_stack: &aws_sdk_cloudformation::types::Stack,
        user_parameter_map: &ParameterMap,
    ) -> Option<RemoteOperation> {
        let client_request_token = ClientRequestToken::generate();
        let existing_stack_parameter_keys = existing_stack_parameter_keys(existing_stack);

        let request = self
            .cloudformation
            .update_stack()
            .stack_name(existing_stack.stack_id.as_ref().unwrap())
            .set_parameters(Some(
                self.instance_spec
                    .parameter_map
                    .merge(user_parameter_map)
                    .to_template_update_parameters(
                        &self.instance_spec.template.parameter_keys(),
                        &existing_stack_parameter_keys,
                    ),
            ))
            .set_capabilities(Some(self.capabilities()))
            .client_request_token(&client_request_token);

        let response = self.set_update_template(request).await.send().await;

        Self::process_update_response(client_request_token, response)
    }

    async fn start_parameter_update(
        &self,
        cloudformation: &aws_sdk_cloudformation::Client,
        existing_stack: &aws_sdk_cloudformation::types::Stack,
        user_parameter_map: &ParameterMap,
    ) -> Option<RemoteOperation> {
        let client_request_token = ClientRequestToken::generate();
        let response = cloudformation
            .update_stack()
            .stack_name(existing_stack.stack_id.as_ref().unwrap())
            .set_parameters(Some(
                user_parameter_map.to_parameter_update_parameters(existing_stack),
            ))
            .set_capabilities(Some(self.capabilities()))
            .client_request_token(&client_request_token)
            .use_previous_template(true)
            .send()
            .await;

        Self::process_update_response(client_request_token, response)
    }

    fn process_update_response(
        client_request_token: ClientRequestToken,
        result: Result<
            aws_sdk_cloudformation::operation::update_stack::UpdateStackOutput,
            aws_sdk_cloudformation::error::SdkError<
                aws_sdk_cloudformation::operation::update_stack::UpdateStackError,
            >,
        >,
    ) -> Option<RemoteOperation> {
        match result {
            Ok(output) => Some(RemoteOperation {
                client_request_token,
                stack_id: StackId(output.stack_id.unwrap()),
            }),
            Err(error) => {
                let service_error = error.into_service_error();
                let meta = service_error.meta();

                match meta.code() {
                    // CF API has no more direct signal that an update is a noop.
                    Some("ValidationError")
                        if meta.message() == Some("No updates are to be performed.") =>
                    {
                        None
                    }
                    _ => panic!("unexpected service error: {service_error:#?}"),
                }
            }
        }
    }

    fn capabilities(&self) -> Vec<aws_sdk_cloudformation::types::Capability> {
        self.instance_spec.capabilities.iter().cloned().collect()
    }

    fn template_rendered(&self) -> TemplateRendered {
        self.instance_spec.template.rendered()
    }

    async fn set_create_template(
        &self,
        request: aws_sdk_cloudformation::operation::create_stack::builders::CreateStackFluentBuilder,
    ) -> aws_sdk_cloudformation::operation::create_stack::builders::CreateStackFluentBuilder {
        match self.upload_template().await {
            Ok(template_url) => request.template_url(template_url),
            Err(template_body) => request.template_body(template_body),
        }
    }

    async fn set_create_change_set_template(
        &self,
        request: aws_sdk_cloudformation::operation::create_change_set::builders::CreateChangeSetFluentBuilder,
    ) -> aws_sdk_cloudformation::operation::create_change_set::builders::CreateChangeSetFluentBuilder
    {
        match self.upload_template().await {
            Ok(template_url) => request.template_url(template_url),
            Err(template_body) => request.template_body(template_body),
        }
    }

    async fn set_update_template(
        &self,
        request: aws_sdk_cloudformation::operation::update_stack::builders::UpdateStackFluentBuilder,
    ) -> aws_sdk_cloudformation::operation::update_stack::builders::UpdateStackFluentBuilder {
        match self.upload_template().await {
            Ok(template_url) => request.template_url(template_url),
            Err(template_body) => request.template_body(template_body),
        }
    }

    async fn upload_template(&self) -> Result<TemplateUrl, TemplateBody> {
        let template_rendered = self.template_rendered();

        if template_rendered.body.needs_upload() {
            log::debug!("Template is to big for inline, uploading it");

            let template_uploader = self
                .template_uploader
                .as_ref()
                .expect("Template needs upload but template uploader not configured!");

            Ok(template_uploader.upload(&template_rendered).await)
        } else {
            Err(template_rendered.body)
        }
    }
}

pub struct Registry(pub Vec<InstanceSpec>);

impl Registry {
    #[must_use]
    pub fn find(&self, instance_name: &StackName) -> Option<&InstanceSpec> {
        self.0
            .iter()
            .find(|&instance_spec| instance_spec.stack_name == *instance_name)
    }

    #[must_use]
    pub fn templates(&self) -> std::collections::BTreeMap<TemplateName, &Template> {
        let mut map = std::collections::BTreeMap::new();

        for instance_spec in &self.0 {
            let template = &instance_spec.template;
            let name = template.name();

            if let Some(other) = map.insert(name.clone(), template)
                && other != template
            {
                panic!("Template name clash for unequal templates: {name:#?}")
            }
        }

        map
    }

    pub fn golden_tests(&self, path: &std::path::Path) {
        let mut base: std::path::PathBuf = path.into();
        base.push("templates");
        std::fs::create_dir_all(&base).unwrap();

        for (_name, template) in self.templates() {
            verify_template(&base, template)
        }
    }
}

fn verify_template(base: &std::path::Path, template: &Template) {
    let new = template.rendered_pretty();

    let mut template_path: std::path::PathBuf = base.into();

    template_path.push(format!(
        "{}.{}",
        template.name().as_str(),
        new.format.file_ext()
    ));

    if std::env::var("UPDATE_GOLDEN_TESTS").is_ok() {
        if let Ok(existing) = std::fs::read_to_string(&template_path)
            && new.body == existing.into()
        {
            return;
        }
        eprintln!("Updating: {}", template_path.display());
        std::fs::write(template_path, new.body).unwrap();
    } else {
        let expected = std::fs::read_to_string(&template_path)
            .unwrap_or_else(|error| panic!("Could not open: {}: {error}", template_path.display()));

        assert_eq!(&expected, new.body.as_str())
    }
}

fn existing_stack_parameter_keys(
    existing_stack: &aws_sdk_cloudformation::types::Stack,
) -> BTreeSet<ParameterKey> {
    BTreeSet::from_iter(match &existing_stack.parameters {
        None => vec![],
        Some(parameters) => parameters
            .iter()
            .map(|parameter| ParameterKey(parameter.parameter_key.clone().unwrap()))
            .collect(),
    })
}
