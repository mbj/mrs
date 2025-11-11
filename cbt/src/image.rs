use crate::{Backend, Image};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

/// Source for building an image
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BuildSource {
    /// Build from a directory containing a Dockerfile
    Directory(PathBuf),
    /// Build from Dockerfile instructions provided as a string
    Instructions(String),
}

/// Definition for building a container image
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildDefinition {
    image: Image,
    pub source: BuildSource,
}

impl BuildDefinition {
    /// Create a new build definition from a directory containing a Dockerfile
    pub fn from_directory(image: Image, path: impl Into<PathBuf>) -> Self {
        Self {
            image,
            source: BuildSource::Directory(path.into()),
        }
    }

    /// Create a new build definition from Dockerfile instructions as a string
    pub fn from_instructions(image: Image, instructions: impl Into<String>) -> Self {
        Self {
            image,
            source: BuildSource::Instructions(instructions.into()),
        }
    }

    /// Create a build definition from a directory with content-based hash tag
    pub fn from_directory_hash(image_name: &str, path: impl Into<PathBuf>) -> Self {
        let path = path.into();
        let hash = compute_directory_hash(&path);
        let image = Image::from(format!("{}:{}", image_name, hash));
        Self {
            image,
            source: BuildSource::Directory(path),
        }
    }

    /// Create a build definition from Dockerfile instructions with content-based hash tag
    pub fn from_instructions_hash(image_name: &str, instructions: impl Into<String>) -> Self {
        let content = instructions.into();
        let hash = compute_content_hash(&content);
        let image = Image::from(format!("{}:{}", image_name, hash));
        Self {
            image,
            source: BuildSource::Instructions(content),
        }
    }

    /// Build the image using the specified backend and return the built image
    pub fn build(&self, backend: Backend) -> Result<Image, crate::command::CaptureError> {
        let mut arguments = vec![
            "build".to_string(),
            "--tag".to_string(),
            self.image.as_str().to_string(),
        ];

        // Add source
        match &self.source {
            BuildSource::Directory(path) => {
                arguments.push(path.to_string_lossy().to_string());
            }
            BuildSource::Instructions(content) => {
                arguments.push("-".to_string());
                backend
                    .command()
                    .arguments(arguments)
                    .stdin_bytes(content.as_bytes().to_vec())
                    .capture_only_stdout_result()
                    .map(|_| self.image.clone())?;
                return Ok(self.image.clone());
            }
        }

        backend
            .command()
            .arguments(arguments)
            .capture_only_stdout_result()
            .map(|_| self.image.clone())
    }

    /// Build the image only if it's not already present, and return the image
    pub fn build_if_absent(&self, backend: Backend) -> Result<Image, crate::command::CaptureError> {
        if !is_present(backend, &self.image) {
            self.build(backend)
        } else {
            Ok(self.image.clone())
        }
    }

    /// Get the image that will be built
    pub fn image(&self) -> &Image {
        &self.image
    }
}

/// Check if an image is present in the local registry
pub fn is_present(backend: Backend, image: &Image) -> bool {
    match backend {
        Backend::Docker => backend
            .command()
            .arguments(["inspect", "--type", "image", image.as_str()])
            .capture_only_stdout_result()
            .is_ok(),
        Backend::Podman => {
            // For Podman, image exists returns 0 if present, 1 if not
            // We use status() instead of capture because we don't need output
            let status = backend
                .command()
                .arguments(["image", "exists", image.as_str()])
                .status();
            status.success()
        }
    }
}

/// Tag an image with a new name
pub fn tag(backend: Backend, source: &Image, target: &Image) {
    backend
        .command()
        .arguments(["tag", source.as_str(), target.as_str()])
        .capture_only_stdout();
}

/// Pull an image from a registry
pub fn pull(backend: Backend, image: &Image) {
    backend
        .command()
        .arguments(["pull", image.as_str()])
        .capture_only_stdout();
}

/// Pull an image only if it's not already present
pub fn pull_if_absent(backend: Backend, image: &Image) {
    if !is_present(backend, image) {
        pull(backend, image);
    }
}

/// Push an image to a registry
pub fn push(backend: Backend, image: &Image) {
    backend
        .command()
        .arguments(["push", image.as_str()])
        .capture_only_stdout();
}

fn compute_content_hash(content: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn compute_directory_hash(path: &PathBuf) -> String {
    use walkdir::WalkDir;

    let mut hasher = Sha256::new();

    for entry in WalkDir::new(path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            let relative_path = entry.path().strip_prefix(path).unwrap();
            hasher.update(relative_path.to_string_lossy().as_bytes());

            let content = std::fs::read(entry.path()).expect("Failed to read file");
            hasher.update(&content);
        }
    }

    format!("{:x}", hasher.finalize())
}
