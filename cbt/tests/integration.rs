use indoc::indoc;

#[test]
fn test_hello_world() {
    let backend = cbt::test_backend_setup!();

    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint("echo".to_string())
        .argument("Hello, World!".to_string())
        .remove();

    let output = definition.run_capture_only_stdout();

    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "Hello, World!");
}

#[test]
fn test_backend_autodetect() {
    let _backend = cbt::test_backend_setup!();
}

#[test]
fn test_container_with_env_vars() {
    let backend = cbt::test_backend_setup!();

    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint("sh".to_string())
        .arguments(vec!["-c".to_string(), "echo $TEST_VAR".to_string()])
        .env("TEST_VAR", "test_value")
        .remove();

    let output = definition.run_capture_only_stdout();
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "test_value");
}

#[test]
fn test_container_detached_and_exec() {
    let backend = cbt::test_backend_setup!();

    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint("sh".to_string())
        .arguments(vec![
            "-c".to_string(),
            "trap 'exit 0' TERM; sleep 30 & wait".to_string(),
        ]);

    definition.with_container(|container| {
        let output = container.exec_capture_only_stdout([], "echo", ["Container is running!"]);
        let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

        assert_eq!(stdout.trim(), "Container is running!");
    });
}

#[test]
fn test_read_host_tcp_port() {
    let backend = cbt::test_backend_setup!();

    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint("sh".to_string())
        .arguments(vec![
            "-c".to_string(),
            "trap 'exit 0' TERM; sleep 30 & wait".to_string(),
        ])
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
    let backend = cbt::test_backend_setup!();

    let definition = cbt::Definition::new(backend, cbt::Image::from("alpine:latest"))
        .entrypoint("sh".to_string())
        .arguments(vec![
            "-c".to_string(),
            "trap 'exit 0' TERM; sleep 30 & wait".to_string(),
        ]);

    definition.with_container(|container| {
        let host_port = container.read_host_tcp_port(8080);

        assert_eq!(host_port, None);
    });
}

#[test]
fn test_command_with_stdin() {
    let input = b"Hello from stdin!";
    let output = cbt::Command::new("cat")
        .stdin_bytes(input.to_vec())
        .capture_only_stdout();

    assert_eq!(output, input);
}

#[test]
fn test_image_build_from_instructions() {
    let backend = cbt::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN touch dirty && echo 'test build from instructions'
    "};

    let definition = cbt::BuildDefinition::from_instructions(
        cbt::Image::from("cbt-test-instructions:latest"),
        dockerfile,
    );

    let built_image = definition.build(backend).expect("Build should succeed");

    assert!(
        cbt::image::is_present(backend, &built_image),
        "Image should exist after build"
    );

    backend.remove_image(&built_image);
}

#[test]
fn test_image_build_from_directory() {
    let backend = cbt::test_backend_setup!();

    let definition = cbt::BuildDefinition::from_directory(
        cbt::Image::from("cbt-test-directory:latest"),
        "tests/fixtures/test-build",
    );

    let built_image = definition.build(backend).expect("Build should succeed");

    assert!(
        cbt::image::is_present(backend, &built_image),
        "Image should exist after build"
    );

    backend.remove_image(&built_image);
}

#[test]
fn test_image_build_if_absent() {
    let backend = cbt::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN touch dirty echo 'test build if absent'
    "};

    let definition = cbt::BuildDefinition::from_instructions(
        cbt::Image::from("cbt-test-if-absent:latest"),
        dockerfile,
    );

    let built_image = definition
        .build_if_absent(backend)
        .expect("First build should succeed");
    assert!(cbt::image::is_present(backend, &built_image));

    let built_image2 = definition
        .build_if_absent(backend)
        .expect("Second build should succeed");
    assert!(cbt::image::is_present(backend, &built_image2));

    backend.remove_image(&built_image);
}

#[test]
fn test_image_tag() {
    let backend = cbt::test_backend_setup!();

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
    let backend = cbt::test_backend_setup!();

    let image = cbt::Image::from("alpine:latest");

    cbt::image::pull_if_absent(backend, &image);
    assert!(cbt::image::is_present(backend, &image));
}

#[test]
fn test_image_build_from_instructions_hash() {
    let backend = cbt::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN touch dirty && echo 'test hash'
    "};

    let definition = cbt::BuildDefinition::from_instructions_hash("cbt-test-hash", dockerfile);

    let built_image = definition.build(backend).expect("Build should succeed");
    assert!(cbt::image::is_present(backend, &built_image));

    let definition2 = cbt::BuildDefinition::from_instructions_hash("cbt-test-hash", dockerfile);
    assert_eq!(definition.image(), definition2.image());

    backend.remove_image(&built_image);
}

#[test]
fn test_image_build_from_directory_hash() {
    let backend = cbt::test_backend_setup!();

    let definition =
        cbt::BuildDefinition::from_directory_hash("cbt-test-dir-hash", "tests/fixtures/test-build");

    let built_image = definition.build(backend).expect("Build should succeed");
    assert!(cbt::image::is_present(backend, &built_image));

    let definition2 =
        cbt::BuildDefinition::from_directory_hash("cbt-test-dir-hash", "tests/fixtures/test-build");
    assert_eq!(definition.image(), definition2.image());

    backend.remove_image(&built_image);
}
