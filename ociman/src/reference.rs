use crate::{Backend, Reference};
use sha2::{Digest, Sha256};
use std::path::PathBuf;
use std::str::FromStr;

/// Build argument key
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BuildArgumentKey(String);

impl BuildArgumentKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for BuildArgumentKey {
    type Err = BuildArgumentKeyError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input.is_empty() {
            return Err(BuildArgumentKeyError::Empty);
        }
        if input.contains('=') {
            return Err(BuildArgumentKeyError::ContainsEquals);
        }
        Ok(BuildArgumentKey(input.to_string()))
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum BuildArgumentKeyError {
    #[error("Build argument key cannot be empty")]
    Empty,
    #[error("Build argument key cannot contain '=' character")]
    ContainsEquals,
}

/// Build argument value
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BuildArgumentValue(String);

impl BuildArgumentValue {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for BuildArgumentValue {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(BuildArgumentValue(input.to_string()))
    }
}

impl From<String> for BuildArgumentValue {
    fn from(string: String) -> Self {
        BuildArgumentValue(string)
    }
}

impl From<&str> for BuildArgumentValue {
    fn from(string: &str) -> Self {
        BuildArgumentValue(string.to_string())
    }
}

/// Source for building an image
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BuildSource {
    /// Build from a directory containing a Dockerfile
    Directory(PathBuf),
    /// Build from Dockerfile instructions provided as a string
    Instructions(String),
}

/// Image naming strategy for the build
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ImageName {
    /// Use a static image reference
    Static(Reference),
    /// Generate image name with content-based hash
    Hashed { name: String },
}

/// Definition for building a container image
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildDefinition {
    backend: Backend,
    image_name: ImageName,
    source: BuildSource,
    build_arguments: std::collections::BTreeMap<BuildArgumentKey, BuildArgumentValue>,
}

impl BuildDefinition {
    /// Create a new build definition from a directory containing a Dockerfile
    pub fn from_directory(backend: Backend, image: Reference, path: impl Into<PathBuf>) -> Self {
        Self {
            backend,
            image_name: ImageName::Static(image),
            source: BuildSource::Directory(path.into()),
            build_arguments: std::collections::BTreeMap::new(),
        }
    }

    /// Create a new build definition from Dockerfile instructions as a string
    pub fn from_instructions(
        backend: Backend,
        image: Reference,
        instructions: impl Into<String>,
    ) -> Self {
        Self {
            backend,
            image_name: ImageName::Static(image),
            source: BuildSource::Instructions(instructions.into()),
            build_arguments: std::collections::BTreeMap::new(),
        }
    }

    /// Create a build definition from a directory with content-based hash tag
    pub fn from_directory_hash(
        backend: Backend,
        image_name: &str,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            backend,
            image_name: ImageName::Hashed {
                name: image_name.to_string(),
            },
            source: BuildSource::Directory(path.into()),
            build_arguments: std::collections::BTreeMap::new(),
        }
    }

    /// Create a build definition from Dockerfile instructions with content-based hash tag
    pub fn from_instructions_hash(
        backend: Backend,
        image_name: &str,
        instructions: impl Into<String>,
    ) -> Self {
        Self {
            backend,
            image_name: ImageName::Hashed {
                name: image_name.to_string(),
            },
            source: BuildSource::Instructions(instructions.into()),
            build_arguments: std::collections::BTreeMap::new(),
        }
    }

    /// Add a build argument
    pub fn build_argument(
        mut self,
        key: BuildArgumentKey,
        value: impl Into<BuildArgumentValue>,
    ) -> Self {
        self.build_arguments.insert(key, value.into());
        self
    }

    /// Add multiple build arguments
    pub fn build_arguments<V: Into<BuildArgumentValue>>(
        mut self,
        arguments: impl IntoIterator<Item = (BuildArgumentKey, V)>,
    ) -> Self {
        self.build_arguments.extend(
            arguments
                .into_iter()
                .map(|(key, value)| (key, value.into())),
        );
        self
    }

    /// Build the image using the specified backend and return the built reference
    pub fn build(&self) -> Reference {
        self.build_image(self.compute_final_image())
    }

    /// Build the image only if it's not already present, and return the reference
    pub fn build_if_absent(&self) -> Reference {
        let target_image = self.compute_final_image();

        if self.backend.is_image_present(&target_image) {
            target_image
        } else {
            self.build_image(target_image)
        }
    }

    fn build_image(&self, target_image: Reference) -> Reference {
        let mut arguments = vec!["build".into(), "--tag".into(), target_image.as_str().into()];

        for (key, value) in &self.build_arguments {
            arguments.push("--build-arg".into());
            arguments.push(format!("{}={}", key.as_str(), value.as_str()));
        }

        let command = match &self.source {
            BuildSource::Directory(path) => {
                arguments.push(path.to_string_lossy().into());
                self.backend.command().arguments(arguments)
            }
            BuildSource::Instructions(content) => {
                arguments.push("-".into());
                self.backend
                    .command()
                    .arguments(arguments)
                    .stdin_bytes(content.as_bytes().to_vec())
            }
        };

        command.capture_only_stdout();

        target_image
    }

    /// Compute the final image name with hash if this is a hash-based definition
    fn compute_final_image(&self) -> Reference {
        match &self.image_name {
            ImageName::Static(image) => image.clone(),
            ImageName::Hashed { name } => {
                let hash = match &self.source {
                    BuildSource::Directory(path) => {
                        compute_directory_hash(path, &self.build_arguments)
                    }
                    BuildSource::Instructions(content) => {
                        compute_content_hash(content, &self.build_arguments)
                    }
                };
                Reference::from(format!("{}:{}", name, hash))
            }
        }
    }
}

fn compute_content_hash(
    content: &str,
    build_arguments: &std::collections::BTreeMap<BuildArgumentKey, BuildArgumentValue>,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());

    for (key, value) in build_arguments {
        hasher.update(key.as_str().as_bytes());
        hasher.update(b"=");
        hasher.update(value.as_str().as_bytes());
    }

    format!("{:x}", hasher.finalize())
}

fn compute_directory_hash(
    path: &PathBuf,
    build_arguments: &std::collections::BTreeMap<BuildArgumentKey, BuildArgumentValue>,
) -> String {
    use walkdir::WalkDir;

    let mut hasher = Sha256::new();

    for entry in WalkDir::new(path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|result| result.ok())
    {
        if entry.file_type().is_file() {
            let relative_path = entry.path().strip_prefix(path).unwrap();
            hasher.update(relative_path.to_string_lossy().as_bytes());

            let content = std::fs::read(entry.path()).expect("Failed to read file");
            hasher.update(&content);
        }
    }

    for (key, value) in build_arguments {
        hasher.update(key.as_str().as_bytes());
        hasher.update(b"=");
        hasher.update(value.as_str().as_bytes());
    }

    format!("{:x}", hasher.finalize())
}
