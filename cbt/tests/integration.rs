#[test]
fn test_hello_world() {
    if cbt::testing::platform_not_supported() {
        return;
    }

    let backend = cbt::backend::autodetect::run().expect("No container backend detected");

    // Run a simple hello world container using alpine
    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint("echo".to_string(), vec!["Hello, World!".to_string()])
        .remove();

    let output = definition.run_capture_only_stdout();

    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "Hello, World!");
}

#[test]
fn test_backend_autodetect() {
    if cbt::testing::platform_not_supported() {
        return;
    }

    // Test that autodetect can find a backend
    let result = cbt::backend::autodetect::run();
    assert!(
        result.is_ok(),
        "Expected to find a container backend, but got: {:?}",
        result
    );
}

#[test]
fn test_container_with_env_vars() {
    if cbt::testing::platform_not_supported() {
        return;
    }

    let backend = cbt::backend::autodetect::run().expect("No container backend detected");

    // Run container with environment variables
    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint(
            "sh".to_string(),
            vec!["-c".to_string(), "echo $TEST_VAR".to_string()],
        )
        .env("TEST_VAR", "test_value")
        .remove();

    let output = definition.run_capture_only_stdout();
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "test_value");
}

#[test]
fn test_container_detached_and_exec() {
    if cbt::testing::platform_not_supported() {
        return;
    }

    let backend = cbt::backend::autodetect::run().expect("No container backend detected");

    // Run a long-running container
    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint("sleep".to_string(), vec!["30".to_string()]);

    let mut container = definition.run_detached();

    // Execute a command in the running container
    let output = container.exec_capture_only_stdout([], "echo", ["Container is running!"]);
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "Container is running!");

    // Stop the container
    container.stop();
}
