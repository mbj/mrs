use indoc::indoc;

/// Helper function to create a Definition with .remove() automatically set
/// to prevent container leaks in tests.
fn test_definition(backend: ociman::Backend, image: ociman::Image) -> ociman::Definition {
    ociman::Definition::new(backend, image).remove()
}

#[test]
fn test_hello_world() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(backend, ociman::Image::from("alpine:latest"))
        .entrypoint("echo")
        .argument("Hello, World!");

    let output = definition.run_capture_only_stdout();

    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "Hello, World!");
}

#[test]
fn test_backend_autodetect() {
    let _backend = ociman::test_backend_setup!();
}

#[test]
fn test_container_with_env_vars() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(backend, ociman::Image::from("alpine:latest"))
        .entrypoint("sh")
        .arguments(["-c", "echo $TEST_VAR"])
        .environment_variable("TEST_VAR", "test_value");

    let output = definition.run_capture_only_stdout();
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "test_value");
}

#[test]
fn test_container_exec() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(backend, ociman::Image::from("alpine:latest"))
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition.with_container(|container| {
        let output = container.exec_capture_only_stdout([], "echo", ["Container is running!"]);
        let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

        assert_eq!(stdout.trim(), "Container is running!");
    });
}

#[test]
fn test_read_host_tcp_port() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(backend, ociman::Image::from("alpine:latest"))
        .entrypoint("sh".to_string())
        .arguments(vec![
            "-c".to_string(),
            "trap 'exit 0' TERM; sleep 30 & wait".to_string(),
        ])
        .publish(ociman::Publish::tcp(8080).host_ip(std::net::Ipv4Addr::LOCALHOST.into()));

    definition.with_container(|container| {
        let host_port = container
            .read_host_tcp_port(8080)
            .expect("port 8080 should be published");

        assert!(host_port > 0);
    });
}

#[test]
fn test_read_host_tcp_port_not_published() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(backend, ociman::Image::from("alpine:latest"))
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition.with_container(|container| {
        let host_port = container.read_host_tcp_port(8080);

        assert_eq!(host_port, None);
    });
}

#[test]
fn test_command_with_stdin() {
    let input = b"Hello from stdin!";
    let output = ociman::Command::new("cat")
        .stdin_bytes(input.to_vec())
        .capture_only_stdout();

    assert_eq!(output, input);
}

#[test]
fn test_image_build_from_instructions() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN touch dirty && echo 'test build from instructions'
    "};

    let definition = ociman::BuildDefinition::from_instructions(
        backend,
        ociman::Image::from("ociman-test-instructions:latest"),
        dockerfile,
    );

    let image = definition.build();

    assert!(
        backend.is_image_present(&image),
        "Image should exist after build"
    );

    backend.remove_image(&image);
}

#[test]
fn test_image_build_from_directory() {
    let backend = ociman::test_backend_setup!();

    let definition = ociman::BuildDefinition::from_directory(
        backend,
        ociman::Image::from("ociman-test-directory:latest"),
        "tests/fixtures/test-build",
    );

    let image = definition.build();

    assert!(
        backend.is_image_present(&image),
        "Image should exist after build"
    );

    backend.remove_image(&image);
}

#[test]
fn test_image_build_if_absent() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN touch dirty && echo 'test build if absent'
    "};

    let definition = ociman::BuildDefinition::from_instructions(
        backend,
        ociman::Image::from("ociman-test-if-absent:latest"),
        dockerfile,
    );

    let image1 = definition.build_if_absent();
    assert!(backend.is_image_present(&image1));

    let image2 = definition.build_if_absent();
    assert!(backend.is_image_present(&image2));

    assert_eq!(image1, image2);

    backend.remove_image(&image1);
}

#[test]
fn test_image_tag() {
    let backend = ociman::test_backend_setup!();

    let source = ociman::Image::from("alpine:latest");
    let target = ociman::Image::from("ociman-test-tagged:latest");

    backend.pull_image_if_absent(&source);

    assert!(!backend.is_image_present(&target));

    backend.tag_image(&source, &target);

    assert!(backend.is_image_present(&source));
    assert!(backend.is_image_present(&target));

    backend.remove_image(&target);
}

#[test]
fn test_image_pull_if_absent() {
    let backend = ociman::test_backend_setup!();

    let image = ociman::Image::from("alpine:latest");

    backend.pull_image_if_absent(&image);
    assert!(backend.is_image_present(&image));
}

#[test]
fn test_image_build_from_instructions_hash() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN touch dirty && echo 'test hash'
    "};

    let definition =
        ociman::BuildDefinition::from_instructions_hash(backend, "ociman-test-hash", dockerfile);

    let image = definition.build();
    assert!(backend.is_image_present(&image));

    let definition2 =
        ociman::BuildDefinition::from_instructions_hash(backend, "ociman-test-hash", dockerfile);
    let image2 = definition2.build();
    assert_eq!(image, image2);

    backend.remove_image(&image);
}

#[test]
fn test_image_build_from_directory_hash() {
    let backend = ociman::test_backend_setup!();

    let definition = ociman::BuildDefinition::from_directory_hash(
        backend,
        "ociman-test-dir-hash",
        "tests/fixtures/test-build",
    );

    let image1 = definition.build();
    assert!(backend.is_image_present(&image1));

    let definition2 = ociman::BuildDefinition::from_directory_hash(
        backend,
        "ociman-test-dir-hash",
        "tests/fixtures/test-build",
    );
    let image2 = definition2.build();
    assert_eq!(image1, image2);

    backend.remove_image(&image1);
}

#[test]
fn test_image_build_with_build_args() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        ARG TEST_ARG
        RUN touch dirty && echo \"Build arg value: $TEST_ARG\" > /test-output
    "};

    let definition = ociman::BuildDefinition::from_instructions(
        backend,
        ociman::Image::from("ociman-test-build-args:latest"),
        dockerfile,
    )
    .build_argument("TEST_ARG".parse().unwrap(), "test_value");

    let image = definition.build();
    assert!(backend.is_image_present(&image));

    // Verify the build arg was used by checking the file created during build
    let def = test_definition(backend, image.clone())
        .entrypoint("cat")
        .arguments(["/test-output"]);

    let output = def.run_capture_only_stdout();
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert!(stdout.contains("test_value"));

    backend.remove_image(&image);
}

#[test]
fn test_image_build_args_affect_hash() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        ARG VERSION
        RUN touch dirty && echo \"Version: $VERSION\"
    "};

    let definition1 = ociman::BuildDefinition::from_instructions_hash(
        backend,
        "ociman-test-args-hash",
        dockerfile,
    )
    .build_argument("VERSION".parse().unwrap(), "1.0");
    let image1 = definition1.build();

    let definition2 = ociman::BuildDefinition::from_instructions_hash(
        backend,
        "ociman-test-args-hash",
        dockerfile,
    )
    .build_argument("VERSION".parse().unwrap(), "2.0");
    let image2 = definition2.build();

    assert_ne!(
        image1, image2,
        "Different build arguments should produce different image tags"
    );

    let definition3 = ociman::BuildDefinition::from_instructions_hash(
        backend,
        "ociman-test-args-hash",
        dockerfile,
    )
    .build_argument("VERSION".parse().unwrap(), "1.0");
    let image3 = definition3.build();

    assert_eq!(
        image1, image3,
        "Same build arguments should produce same image tag"
    );

    backend.remove_image(&image1);
    backend.remove_image(&image2);
}

#[test]
fn test_build_argument_key_cannot_be_empty() {
    let result: Result<ociman::BuildArgumentKey, _> = "".parse();
    assert!(matches!(result, Err(ociman::BuildArgumentKeyError::Empty)));
}

#[test]
fn test_build_argument_key_cannot_contain_equals() {
    let result: Result<ociman::BuildArgumentKey, _> = "KEY=VALUE".parse();
    assert!(matches!(
        result,
        Err(ociman::BuildArgumentKeyError::ContainsEquals)
    ));
}

#[test]
fn test_run_status_with_successful_exit() {
    let backend = ociman::test_backend_setup!();

    let definition =
        test_definition(backend, ociman::Image::from("alpine:latest")).entrypoint("true");

    let status = definition.run_status();
    assert!(status.success());
}

#[test]
fn test_run_status_with_nonzero_exit() {
    let backend = ociman::test_backend_setup!();

    let definition =
        test_definition(backend, ociman::Image::from("alpine:latest")).entrypoint("false");

    let status = definition.run_status();
    assert!(!status.success());
    assert_eq!(status.code(), Some(1));
}

#[test]
fn test_run_status_success_with_successful_exit() {
    let backend = ociman::test_backend_setup!();

    test_definition(backend, ociman::Image::from("alpine:latest"))
        .entrypoint("true")
        .run_status_success();
}

#[test]
#[should_panic(expected = "Container execution failed with status: exit status: 1")]
// we are not executing on OSX as on GH there is no docker support so cannot run into the panic
// there.
#[cfg(not(target_os = "macos"))]
fn test_run_status_success_with_nonzero_exit() {
    let backend = ociman::test_backend_setup!();

    test_definition(backend, ociman::Image::from("alpine:latest"))
        .entrypoint("false")
        .run_status_success();
}

#[test]
fn test_container_with_workdir() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(backend, ociman::Image::from("alpine:latest"))
        .entrypoint("pwd")
        .workdir("/tmp");

    let output = definition.run_capture_only_stdout();
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "/tmp");
}
