pub fn platform_not_supported() -> bool {
    std::env::consts::OS == "macos" && std::env::var("GITHUB_ACTIONS").is_ok()
}
