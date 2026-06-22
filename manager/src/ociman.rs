//! ociman backend management commands.
//!
//! Within this module the `ociman` crate is referenced as `::ociman` to avoid
//! shadowing by this module's own name.

#[derive(Debug, clap::Parser)]
pub(crate) enum Command {
    /// Pre-pull the container images the test suite expects to be present.
    ///
    /// Wired as a nextest setup script so the pull happens once, serially,
    /// before the parallel test run — rather than tests pulling on demand and
    /// racing concurrent builds on the shared layer store.
    PullTestImages,
}

impl Command {
    pub(crate) async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::PullTestImages => pull_test_images().await,
        }
    }
}

async fn pull_test_images() -> Result<(), Box<dyn std::error::Error>> {
    if ::ociman::testing::platform_not_supported() {
        log::info!("pull-test-images: platform not supported, skipping");
        return Ok(());
    }

    let backend = ::ociman::backend::resolve::auto().await?;

    // ociman's shared/generic test images, plus pg-ephemeral's default Postgres
    // image — computed here so it tracks whatever pg-ephemeral actually boots,
    // rather than hardcoding its registry convention into the ociman layer.
    let mut images = ::ociman::testing::test_images();
    images.push((&pg_ephemeral::Image::default()).into());

    for image in &images {
        log::info!("pull-test-images: pulling {image}");
        backend.pull_image(image).await?;
    }

    Ok(())
}
