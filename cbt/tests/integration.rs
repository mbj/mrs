use indoc::indoc;

macro_rules! backend_setup {
    () => {{
        if cbt::testing::platform_not_supported() {
            return;
        }
        cbt::backend::autodetect::run().expect("No container backend detected")
    }};
}

#[test]
fn test_hello_world() {
    let backend = backend_setup!();

    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint("echo".to_string(), vec!["Hello, World!".to_string()])
        .remove();

    let output = definition.run_capture_only_stdout();

    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "Hello, World!");
}

#[test]
fn test_backend_autodetect() {
    let _backend = backend_setup!();
}

#[test]
fn test_container_with_env_vars() {
    let backend = backend_setup!();

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
    let backend = backend_setup!();

    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest")).entrypoint(
        "sh".to_string(),
        vec![
            "-c".to_string(),
            "trap 'exit 0' TERM; sleep 30 & wait".to_string(),
        ],
    );

    definition.with_container(|container| {
        let output = container.exec_capture_only_stdout([], "echo", ["Container is running!"]);
        let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

        assert_eq!(stdout.trim(), "Container is running!");
    });
}

#[test]
fn test_read_host_tcp_port() {
    let backend = backend_setup!();

    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint(
            "sh".to_string(),
            vec![
                "-c".to_string(),
                "trap 'exit 0' TERM; sleep 30 & wait".to_string(),
            ],
        )
        .publish(cbt::Publish::from("127.0.0.1::8080/tcp"));

    definition.with_container(|container| {
        let host_port = container
            .read_host_tcp_port(8080)
            .expect("port 8080 should be published");

        assert!(host_port > 0);
    });
}

#[test]
fn test_read_host_tcp_port_not_published() {
    let backend = backend_setup!();

    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest")).entrypoint(
        "sh".to_string(),
        vec![
            "-c".to_string(),
            "trap 'exit 0' TERM; sleep 30 & wait".to_string(),
        ],
    );

    definition.with_container(|container| {
        let host_port = container.read_host_tcp_port(8080);

        assert_eq!(host_port, None);
    });
}

#[test]
fn test_command_with_stdin() {
    if cbt::testing::platform_not_supported() {
        return;
    }

    let input = b"Hello from stdin!";
    let output = cbt::Command::new("cat")
        .stdin_bytes(input.to_vec())
        .capture_only_stdout();

    assert_eq!(output, input);
}

#[test]
fn test_image_build_from_instructions() {
    let backend = backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN echo 'test build from instructions'
    "};
    let image = cbt::Image::from("cbt-test-instructions:latest");

    let definition = cbt::BuildDefinition::from_instructions(image.clone(), dockerfile);

    cbt::image::build(backend, &definition).expect("Build should succeed");

    assert!(
        cbt::image::is_present(backend, &image),
        "Image should exist after build"
    );

    backend.remove_image(&image);
}

#[test]
fn test_image_build_from_directory() {
    let backend = backend_setup!();

    let image = cbt::Image::from("cbt-test-directory:latest");
    let definition =
        cbt::BuildDefinition::from_directory(image.clone(), "tests/fixtures/test-build");

    cbt::image::build(backend, &definition).expect("Build should succeed");

    assert!(
        cbt::image::is_present(backend, &image),
        "Image should exist after build"
    );

    backend.remove_image(&image);
}

#[test]
fn test_image_build_if_absent() {
    let backend = backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN echo 'test build if absent'
    "};
    let image = cbt::Image::from(format!("cbt-test-if-absent-{}:latest", std::process::id()));

    let definition = cbt::BuildDefinition::from_instructions(image.clone(), dockerfile);

    cbt::image::build_if_absent(backend, &definition).expect("First build should succeed");
    assert!(cbt::image::is_present(backend, &image));

    cbt::image::build_if_absent(backend, &definition).expect("Second build should succeed");
    assert!(cbt::image::is_present(backend, &image));

    backend.remove_image(&image);
}

#[test]
fn test_image_tag() {
    let backend = backend_setup!();

    let source = cbt::Image::from("alpine:latest");
    cbt::image::pull_if_absent(backend, &source);

    let target = cbt::Image::from("cbt-test-tagged:latest");
    cbt::image::tag(backend, &source, &target);

    assert!(cbt::image::is_present(backend, &source));
    assert!(cbt::image::is_present(backend, &target));

    backend.remove_image(&target);
}

#[test]
fn test_image_pull_if_absent() {
    let backend = backend_setup!();

    let image = cbt::Image::from("alpine:latest");

    cbt::image::pull_if_absent(backend, &image);
    assert!(cbt::image::is_present(backend, &image));
}

#[test]
fn test_image_build_from_instructions_hash() {
    let backend = backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN echo 'test hash'
    "};

    let definition = cbt::BuildDefinition::from_instructions_hash("cbt-test-hash", dockerfile);

    cbt::image::build(backend, &definition).expect("Build should succeed");
    assert!(cbt::image::is_present(backend, &definition.image));

    let definition2 = cbt::BuildDefinition::from_instructions_hash("cbt-test-hash", dockerfile);
    assert_eq!(definition.image, definition2.image);

    backend.remove_image(&definition.image);
}

#[test]
fn test_image_build_from_directory_hash() {
    let backend = backend_setup!();

    let definition =
        cbt::BuildDefinition::from_directory_hash("cbt-test-dir-hash", "tests/fixtures/test-build");

    cbt::image::build(backend, &definition).expect("Build should succeed");
    assert!(cbt::image::is_present(backend, &definition.image));

    let definition2 =
        cbt::BuildDefinition::from_directory_hash("cbt-test-dir-hash", "tests/fixtures/test-build");
    assert_eq!(definition.image, definition2.image);

    backend.remove_image(&definition.image);
}
