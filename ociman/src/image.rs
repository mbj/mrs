//! OCI image reference and build utilities.
//!
//! This module provides types for working with OCI image references and building images.
//!
//! # Usage
//!
//! It's recommended to import this module as `image`:
//!
//! ```
//! use ociman::image;
//!
//! let reference: image::Reference = "alpine:latest".parse().unwrap();
//! ```

pub use crate::reference::Reference;

use crate::Backend;
use sha2::Sha256;
use std::path::PathBuf;
use std::str::FromStr;

/// Build argument key
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BuildArgumentKey(String);

impl BuildArgumentKey {
    #[must_use]
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

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Id([u8; 32]);

impl FromStr for Id {
    type Err = IdError;

    // Podman prints the bare hex, Docker prints the digest form via image.ID.String():
    // https://github.com/moby/moby/blob/master/daemon/internal/image/image.go
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let hex = input.strip_prefix("sha256:").unwrap_or(input);

        hex::FromHex::from_hex(hex).map(Id).map_err(IdError)
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", hex::encode(self.0))
    }
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
#[error("image ID must be 64 hex characters, optionally prefixed with sha256:")]
pub struct IdError(#[source] hex::FromHexError);

#[derive(Debug, thiserror::Error)]
pub enum BuildError {
    #[error("image build failed")]
    Command(#[source] cmd_proc::CommandError),
}

#[derive(Debug, thiserror::Error)]
pub enum BuildIfAbsentError {
    #[error("image presence check failed")]
    ImagePresent(#[source] crate::backend::ImagePresentError),
    #[error(transparent)]
    Build(BuildError),
}

/// Build argument value
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BuildArgumentValue(String);

impl BuildArgumentValue {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for BuildArgumentValue {
    type Err = !;

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

/// Target image specification for the build
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BuildTarget {
    /// Use a fixed image reference
    Fixed(Reference),
    /// Generate tag from content hash, using the given name
    ContentAddressed(crate::reference::Name),
}

/// Definition for building a container image
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildDefinition {
    backend: Backend,
    target: BuildTarget,
    source: BuildSource,
    build_arguments: std::collections::BTreeMap<BuildArgumentKey, BuildArgumentValue>,
    labels: crate::label::Map,
}

impl BuildDefinition {
    /// Create a new build definition from a directory containing a Dockerfile
    pub fn from_directory(
        backend: &Backend,
        reference: Reference,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            backend: backend.clone(),
            target: BuildTarget::Fixed(reference),
            source: BuildSource::Directory(path.into()),
            build_arguments: std::collections::BTreeMap::new(),
            labels: crate::label::Map::new(),
        }
    }

    /// Create a new build definition from Dockerfile instructions as a string
    pub fn from_instructions(
        backend: &Backend,
        reference: Reference,
        instructions: impl Into<String>,
    ) -> Self {
        Self {
            backend: backend.clone(),
            target: BuildTarget::Fixed(reference),
            source: BuildSource::Instructions(instructions.into()),
            build_arguments: std::collections::BTreeMap::new(),
            labels: crate::label::Map::new(),
        }
    }

    /// Create a build definition from a directory with content-based hash tag
    pub fn from_directory_hash(
        backend: &Backend,
        name: crate::reference::Name,
        path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            backend: backend.clone(),
            target: BuildTarget::ContentAddressed(name),
            source: BuildSource::Directory(path.into()),
            build_arguments: std::collections::BTreeMap::new(),
            labels: crate::label::Map::new(),
        }
    }

    /// Create a build definition from Dockerfile instructions with content-based hash tag
    pub fn from_instructions_hash(
        backend: &Backend,
        name: crate::reference::Name,
        instructions: impl Into<String>,
    ) -> Self {
        Self {
            backend: backend.clone(),
            target: BuildTarget::ContentAddressed(name),
            source: BuildSource::Instructions(instructions.into()),
            build_arguments: std::collections::BTreeMap::new(),
            labels: crate::label::Map::new(),
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

    /// Add an image label. For content-addressed builds, the label is folded
    /// into the content hash so changing a label produces a distinct tag.
    #[must_use]
    pub fn label(mut self, key: &crate::label::Key, value: &crate::label::Value) -> Self {
        self.labels.insert(key.clone(), value.clone());
        self
    }

    /// Add multiple image labels.
    #[must_use]
    pub fn labels<'a>(
        mut self,
        pairs: impl IntoIterator<Item = (&'a crate::label::Key, &'a crate::label::Value)>,
    ) -> Self {
        for (key, value) in pairs {
            self.labels.insert(key.clone(), value.clone());
        }
        self
    }

    /// Build the image using the specified backend and return the built image reference
    pub async fn build(&self) -> Result<Reference, BuildError> {
        self.build_image(self.compute_final_reference()).await
    }

    /// Build the image only if it's not already present, and return the image reference.
    pub async fn build_if_absent(&self) -> Result<Reference, BuildIfAbsentError> {
        let target_reference = self.compute_final_reference();

        if self
            .backend
            .is_image_present(&target_reference)
            .await
            .map_err(BuildIfAbsentError::ImagePresent)?
        {
            Ok(target_reference)
        } else {
            self.build_image(target_reference)
                .await
                .map_err(BuildIfAbsentError::Build)
        }
    }

    async fn build_image(&self, target_reference: Reference) -> Result<Reference, BuildError> {
        let mut arguments = vec!["build".into(), "--tag".into(), target_reference.to_string()];

        for (key, value) in &self.build_arguments {
            arguments.push("--build-arg".into());
            arguments.push(format!("{}={}", key.as_str(), value.as_str()));
        }

        for (key, value) in self.labels.iter() {
            arguments.push("--label".into());
            arguments.push(format!("{key}={value}"));
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

        command
            .stdout_to_stderr()
            .run()
            .await
            .map_err(BuildError::Command)?;

        Ok(target_reference)
    }

    /// Resolve the target [`Reference`] this definition would produce, computing
    /// the content-addressed tag without actually building the image. Useful for
    /// tests and tools that want to reason about the identity a build would
    /// produce.
    #[must_use]
    pub fn target_reference(&self) -> Reference {
        self.compute_final_reference()
    }

    /// Compute the final image reference with hash if this is a hash-based definition
    fn compute_final_reference(&self) -> Reference {
        match &self.target {
            BuildTarget::Fixed(reference) => reference.clone(),
            BuildTarget::ContentAddressed(name) => {
                let hash = match &self.source {
                    BuildSource::Directory(path) => {
                        compute_directory_hash(path, &self.build_arguments, &self.labels)
                    }
                    BuildSource::Instructions(content) => {
                        compute_content_hash(content, &self.build_arguments, &self.labels)
                    }
                };
                Reference {
                    name: name.clone(),
                    tag: Some(hash.into()),
                    digest: None,
                }
            }
        }
    }
}

fn push_build_arguments(
    hasher: &mut crate::hasher::SegmentHasher,
    build_arguments: &std::collections::BTreeMap<BuildArgumentKey, BuildArgumentValue>,
) {
    hasher.push(b"build-arguments");
    for (key, value) in build_arguments {
        hasher.push(key.as_str());
        hasher.push(value.as_str());
    }
}

fn push_labels(hasher: &mut crate::hasher::SegmentHasher, labels: &crate::label::Map) {
    hasher.push(b"labels");
    for (key, value) in labels.iter() {
        hasher.push(key.as_str());
        hasher.push(value.as_str());
    }
}

fn compute_content_hash(
    content: &str,
    build_arguments: &std::collections::BTreeMap<BuildArgumentKey, BuildArgumentValue>,
    labels: &crate::label::Map,
) -> sha2::digest::Output<Sha256> {
    let mut hasher = crate::hasher::SegmentHasher::new();

    hasher.push(b"content");
    hasher.push(content);
    push_build_arguments(&mut hasher, build_arguments);
    push_labels(&mut hasher, labels);

    hasher.finalize()
}

fn compute_directory_hash(
    path: &PathBuf,
    build_arguments: &std::collections::BTreeMap<BuildArgumentKey, BuildArgumentValue>,
    labels: &crate::label::Map,
) -> sha2::digest::Output<Sha256> {
    use walkdir::WalkDir;

    let mut hasher = crate::hasher::SegmentHasher::new();

    hasher.push(b"directory");
    for entry in WalkDir::new(path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|result| result.ok())
    {
        if entry.file_type().is_file() {
            let relative_path = entry.path().strip_prefix(path).unwrap();
            hasher.push(b"file.path");
            hasher.push(relative_path.to_string_lossy().as_bytes());

            let content = std::fs::read(entry.path()).expect("Failed to read file");
            hasher.push(b"file.content");
            hasher.push(content);
        }
    }
    push_build_arguments(&mut hasher, build_arguments);
    push_labels(&mut hasher, labels);

    hasher.finalize()
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEX: &str = "8ebabf0d4f1bb5e6dbd2d0f0a3c1d2e3f4a5b6c7d8e9f0011223344556677889";

    #[test]
    fn id_accepts_bare_hex() {
        assert_eq!(HEX.parse::<Id>().unwrap().to_string(), HEX);
    }

    #[test]
    fn id_accepts_sha256_prefixed_hex() {
        assert_eq!(
            format!("sha256:{HEX}").parse::<Id>().unwrap().to_string(),
            HEX
        );
    }

    #[test]
    fn id_rejects_other_algorithm() {
        assert_eq!(
            format!("sha512:{HEX}").parse::<Id>(),
            Err(IdError(hex::FromHexError::OddLength))
        );
    }

    #[test]
    fn id_rejects_short_hex() {
        assert_eq!(
            HEX[..62].parse::<Id>(),
            Err(IdError(hex::FromHexError::InvalidStringLength))
        );
    }

    #[test]
    fn id_rejects_long_hex() {
        assert_eq!(
            format!("{HEX}00").parse::<Id>(),
            Err(IdError(hex::FromHexError::InvalidStringLength))
        );
    }
}
