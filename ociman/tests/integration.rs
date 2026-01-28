use indoc::indoc;

const ENV_TEST_VAR: cmd_proc::EnvVariableName<'static> =
    cmd_proc::EnvVariableName::from_static_or_panic("TEST_VAR");

/// Helper function to create a Definition with .remove() automatically set
/// to prevent container leaks in tests.
fn test_definition(
    backend: &ociman::Backend,
    reference: ociman::image::Reference,
) -> ociman::Definition {
    ociman::Definition::new(backend.clone(), reference).remove()
}

#[test]
fn test_hello_world() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
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

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "echo $TEST_VAR"])
        .environment_variable(ENV_TEST_VAR, "test_value");

    let output = definition.run_capture_only_stdout();
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "test_value");
}

#[test]
fn test_container_exec() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition.with_container(|container| {
        let output = container
            .exec("echo")
            .argument("Container is running!")
            .stdout()
            .string()
            .unwrap();

        assert_eq!(output.trim(), "Container is running!");
    });
}

#[test]
fn test_container_exec_status_success() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition.with_container(|container| {
        assert!(container.exec("true").status().is_ok());
    });
}

#[test]
fn test_container_exec_status_failure() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition.with_container(|container| {
        let error = container.exec("false").status().unwrap_err();
        assert_eq!(error.exit_status.map(|status| status.code()), Some(Some(1)));
    });
}

#[test]
fn test_container_exec_with_stdin() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition.with_container(|container| {
        let output = container
            .exec("cat")
            .stdin(b"hello from stdin")
            .stdout()
            .bytes()
            .unwrap();

        assert_eq!(output, b"hello from stdin");
    });
}

#[test]
fn test_read_host_tcp_port() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
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

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
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
    let output = cmd_proc::Command::new("cat")
        .stdin_bytes(input.to_vec())
        .stdout()
        .bytes()
        .unwrap();

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
        &backend,
        ociman::testing::test_reference("ociman-test-instructions:latest"),
        dockerfile,
    );

    let reference = definition.build();

    assert!(
        backend.is_image_present(&reference),
        "Image should exist after build"
    );

    backend.remove_image(&reference);
}

#[test]
fn test_image_build_from_directory() {
    let backend = ociman::test_backend_setup!();

    let definition = ociman::BuildDefinition::from_directory(
        &backend,
        ociman::testing::test_reference("ociman-test-directory:latest"),
        "tests/fixtures/test-build",
    );

    let reference = definition.build();

    assert!(
        backend.is_image_present(&reference),
        "Image should exist after build"
    );

    backend.remove_image(&reference);
}

#[test]
fn test_image_build_if_absent() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN touch dirty && echo 'test build if absent'
    "};

    let definition = ociman::BuildDefinition::from_instructions(
        &backend,
        ociman::testing::test_reference("ociman-test-if-absent:latest"),
        dockerfile,
    );

    let reference1 = definition.build_if_absent();
    assert!(backend.is_image_present(&reference1));

    let reference2 = definition.build_if_absent();
    assert!(backend.is_image_present(&reference2));

    assert_eq!(reference1, reference2);

    backend.remove_image(&reference1);
}

#[test]
fn test_image_tag() {
    let backend = ociman::test_backend_setup!();

    let source: ociman::image::Reference = "alpine:latest".parse().unwrap();
    let target: ociman::image::Reference =
        ociman::testing::test_reference("ociman-test-tagged:latest");

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

    let reference: ociman::image::Reference = "alpine:latest".parse().unwrap();

    backend.pull_image_if_absent(&reference);
    assert!(backend.is_image_present(&reference));
}

#[test]
fn test_image_build_from_instructions_hash() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = indoc! {"
        FROM alpine:latest
        RUN touch dirty && echo 'test hash'
    "};

    let definition = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-hash"),
        dockerfile,
    );

    let reference = definition.build();
    assert!(backend.is_image_present(&reference));

    let definition2 = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-hash"),
        dockerfile,
    );
    let reference2 = definition2.build();
    assert_eq!(reference, reference2);

    backend.remove_image(&reference);
}

#[test]
fn test_image_build_from_directory_hash() {
    let backend = ociman::test_backend_setup!();

    let definition = ociman::BuildDefinition::from_directory_hash(
        &backend,
        ociman::testing::test_name("ociman-test-dir-hash"),
        "tests/fixtures/test-build",
    );

    let reference1 = definition.build();
    assert!(backend.is_image_present(&reference1));

    let definition2 = ociman::BuildDefinition::from_directory_hash(
        &backend,
        ociman::testing::test_name("ociman-test-dir-hash"),
        "tests/fixtures/test-build",
    );
    let reference2 = definition2.build();
    assert_eq!(reference1, reference2);

    backend.remove_image(&reference1);
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
        &backend,
        ociman::testing::test_reference("ociman-test-build-args:latest"),
        dockerfile,
    )
    .build_argument("TEST_ARG".parse().unwrap(), "test_value");

    let reference = definition.build();
    assert!(backend.is_image_present(&reference));

    // Verify the build arg was used by checking the file created during build
    let def = test_definition(&backend, reference.clone())
        .entrypoint("cat")
        .arguments(["/test-output"]);

    let output = def.run_capture_only_stdout();
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert!(stdout.contains("test_value"));

    backend.remove_image(&reference);
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
        &backend,
        ociman::testing::test_name("ociman-test-args-hash"),
        dockerfile,
    )
    .build_argument("VERSION".parse().unwrap(), "1.0");
    let reference1 = definition1.build();

    let definition2 = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-args-hash"),
        dockerfile,
    )
    .build_argument("VERSION".parse().unwrap(), "2.0");
    let reference2 = definition2.build();

    assert_ne!(
        reference1, reference2,
        "Different build arguments should produce different image tags"
    );

    let definition3 = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-args-hash"),
        dockerfile,
    )
    .build_argument("VERSION".parse().unwrap(), "1.0");
    let reference3 = definition3.build();

    assert_eq!(
        reference1, reference3,
        "Same build arguments should produce same image tag"
    );

    backend.remove_image(&reference1);
    backend.remove_image(&reference2);
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
fn test_run_with_successful_exit() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap()).entrypoint("true");

    assert!(definition.run().is_ok());
}

#[test]
fn test_run_with_nonzero_exit() {
    let backend = ociman::test_backend_setup!();

    let definition =
        test_definition(&backend, "alpine:latest".parse().unwrap()).entrypoint("false");

    let error = definition.run().unwrap_err();
    assert_eq!(error.exit_status.map(|status| status.code()), Some(Some(1)));
}

#[test]
fn test_container_with_workdir() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("pwd")
        .workdir("/tmp");

    let output = definition.run_capture_only_stdout();
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "/tmp");
}

#[test]
fn test_container_commit() {
    let backend = ociman::test_backend_setup!();

    let target_reference: ociman::image::Reference =
        ociman::testing::test_reference("ociman-test-commit:latest");

    // Ensure target image doesn't exist before test
    if backend.is_image_present(&target_reference) {
        backend.remove_image(&target_reference);
    }

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition.with_container(|container| {
        container
            .exec("touch")
            .argument("/committed-file")
            .status()
            .unwrap();
        container.commit(&target_reference, true).unwrap();
    });

    assert!(
        backend.is_image_present(&target_reference),
        "Committed image should exist"
    );

    // Verify the committed file exists in the new image
    let verify_definition = test_definition(&backend, target_reference.clone())
        .entrypoint("ls")
        .arguments(["/committed-file"]);

    let output = verify_definition.run_capture_only_stdout();
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "/committed-file");

    backend.remove_image(&target_reference);
}

#[test]
fn test_container_remove_on_drop() {
    let backend = ociman::test_backend_setup!();

    // Create a container with stop_on_drop and remove_on_drop but WITHOUT --rm flag
    let definition = ociman::Definition::new(backend.clone(), "alpine:latest".parse().unwrap())
        .stop_on_drop()
        .remove_on_drop()
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    let container_id: String;

    {
        let container = definition.run_detached();
        container_id = container.inspect_format("{{.Id}}");

        // Container should exist while we have the handle
        assert!(
            backend
                .command()
                .arguments(["container", "inspect", &container_id])
                .status()
                .is_ok(),
            "Container should exist during scope"
        );

        // Container is dropped here, which should stop AND remove it
    }

    // Container should be removed after drop
    assert!(
        backend
            .command()
            .arguments(["container", "inspect", &container_id])
            .status()
            .is_err(),
        "Container should be removed after drop with remove_on_drop"
    );
}

#[test]
fn test_container_stop_on_drop() {
    let backend = ociman::test_backend_setup!();

    let definition = ociman::Definition::new(backend.clone(), "alpine:latest".parse().unwrap())
        .stop_on_drop()
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    let container_id: String;

    {
        let container = definition.run_detached();
        container_id = container.inspect_format("{{.Id}}");

        // Container should be running
        let status = container.inspect_format("{{.State.Running}}");
        assert_eq!(status, "true", "Container should be running");

        // Container is dropped here, which should stop it
    }

    // Container should still exist but be stopped
    let status = backend
        .command()
        .arguments([
            "container",
            "inspect",
            "--format",
            "{{.State.Running}}",
            &container_id,
        ])
        .stdout()
        .string()
        .unwrap();
    assert_eq!(
        status.trim(),
        "false",
        "Container should be stopped after drop with stop_on_drop"
    );

    // Manually clean up the container
    backend
        .command()
        .arguments(["container", "rm", &container_id])
        .status()
        .unwrap();
}

#[test]
fn test_container_without_stop_on_drop() {
    let backend = ociman::test_backend_setup!();

    // Create a container WITHOUT stop_on_drop (default)
    let definition = ociman::Definition::new(backend.clone(), "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    let container_id: String;

    {
        let container = definition.run_detached();
        container_id = container.inspect_format("{{.Id}}");
        // Container is dropped here, but should NOT be stopped
    }

    // Container should still be running after drop
    let status = backend
        .command()
        .arguments([
            "container",
            "inspect",
            "--format",
            "{{.State.Running}}",
            &container_id,
        ])
        .stdout()
        .string()
        .unwrap();
    assert_eq!(
        status.trim(),
        "true",
        "Container should still be running after drop without stop_on_drop"
    );

    // Manually stop and clean up the container
    backend
        .command()
        .arguments(["container", "stop", &container_id])
        .status()
        .unwrap();
    backend
        .command()
        .arguments(["container", "rm", &container_id])
        .status()
        .unwrap();
}
