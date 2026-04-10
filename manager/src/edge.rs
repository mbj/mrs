use git_proc::Build;

pub(crate) struct EdgeRelease {
    pub(crate) sha: String,
    pub(crate) tag: String,
}

pub(crate) async fn resolve() -> EdgeRelease {
    let sha = git_proc::rev_parse::new()
        .rev("HEAD")
        .build()
        .stdout_capture()
        .string()
        .await
        .unwrap_or_else(|error| panic!("Failed to get git SHA: {error}"))
        .trim()
        .to_string();

    let tag = format!("edge-{sha}");
    log::info!("Edge release: {tag}");

    EdgeRelease { sha, tag }
}
