use indoc::indoc;
use std::time::Duration;
use tokio::time::sleep;

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

#[tokio::test]
async fn test_hello_world() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("echo")
        .argument("Hello, World!");

    let output = definition.run_capture_only_stdout().await;

    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "Hello, World!");
}

#[tokio::test]
async fn test_backend_autodetect() {
    let _backend = ociman::test_backend_setup!();
}

#[tokio::test]
async fn test_container_with_env_vars() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "echo $TEST_VAR"])
        .environment_variable(ENV_TEST_VAR, "test_value");

    let output = definition.run_capture_only_stdout().await;
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "test_value");
}

#[tokio::test]
async fn test_container_exec() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition
        .with_container(async |container| {
            let output = container
                .exec("echo")
                .argument("Container is running!")
                .build()
                .capture_stdout()
                .string()
                .await
                .unwrap();

            assert_eq!(output.trim(), "Container is running!");
        })
        .await;
}

#[tokio::test]
async fn test_container_exec_status_success() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition
        .with_container(async |container| {
            assert!(container.exec("true").status().await.is_ok());
        })
        .await;
}

#[tokio::test]
async fn test_container_exec_status_failure() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition
        .with_container(async |container| {
            let error = container.exec("false").status().await.unwrap_err();
            assert_eq!(error.exit_status.map(|status| status.code()), Some(Some(1)));
        })
        .await;
}

#[tokio::test]
async fn test_container_exec_with_stdin() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition
        .with_container(async |container| {
            let output = container
                .exec("cat")
                .stdin(b"hello from stdin")
                .build()
                .capture_stdout()
                .bytes()
                .await
                .unwrap();

            assert_eq!(output, b"hello from stdin");
        })
        .await;
}

#[tokio::test]
async fn test_read_host_tcp_port() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh".to_string())
        .arguments(vec![
            "-c".to_string(),
            "trap 'exit 0' TERM; sleep 30 & wait".to_string(),
        ])
        .publish(ociman::Publish::tcp(8080).host_ip(std::net::Ipv4Addr::LOCALHOST.into()));

    definition
        .with_container(async |container| {
            let host_port = container
                .read_host_tcp_port(8080)
                .await
                .expect("port 8080 should be published");

            assert!(host_port > 0);
        })
        .await;
}

#[tokio::test]
async fn test_read_host_tcp_port_not_published() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition
        .with_container(async |container| {
            let host_port = container.read_host_tcp_port(8080).await;

            assert_eq!(host_port, None);
        })
        .await;
}

#[tokio::test]
async fn test_command_with_stdin() {
    let input = b"Hello from stdin!";
    let output = cmd_proc::Command::new("cat")
        .stdin_bytes(input.to_vec())
        .capture_stdout()
        .bytes()
        .await
        .unwrap();

    assert_eq!(output, input);
}

#[tokio::test]
async fn test_image_build_from_instructions() {
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

    let reference = definition.build().await;

    assert!(
        backend.is_image_present(&reference).await,
        "Image should exist after build"
    );

    backend.remove_image(&reference).await;
}

#[tokio::test]
async fn test_image_build_from_directory() {
    let backend = ociman::test_backend_setup!();

    let definition = ociman::BuildDefinition::from_directory(
        &backend,
        ociman::testing::test_reference("ociman-test-directory:latest"),
        "tests/fixtures/test-build",
    );

    let reference = definition.build().await;

    assert!(
        backend.is_image_present(&reference).await,
        "Image should exist after build"
    );

    backend.remove_image(&reference).await;
}

#[tokio::test]
async fn test_image_build_if_absent() {
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

    let reference1 = definition.build_if_absent().await;
    assert!(backend.is_image_present(&reference1).await);

    let reference2 = definition.build_if_absent().await;
    assert!(backend.is_image_present(&reference2).await);

    assert_eq!(reference1, reference2);

    backend.remove_image(&reference1).await;
}

#[tokio::test]
async fn test_image_tag() {
    let backend = ociman::test_backend_setup!();

    let source: ociman::image::Reference = "alpine:latest".parse().unwrap();
    let target: ociman::image::Reference =
        ociman::testing::test_reference("ociman-test-tagged:latest");

    backend.pull_image_if_absent(&source).await;

    assert!(!backend.is_image_present(&target).await);

    backend.tag_image(&source, &target).await;

    assert!(backend.is_image_present(&source).await);
    assert!(backend.is_image_present(&target).await);

    backend.remove_image(&target).await;
}

#[tokio::test]
async fn test_image_pull_if_absent() {
    let backend = ociman::test_backend_setup!();

    let reference: ociman::image::Reference = "alpine:latest".parse().unwrap();

    backend.pull_image_if_absent(&reference).await;
    assert!(backend.is_image_present(&reference).await);
}

#[tokio::test]
async fn test_image_build_from_instructions_hash() {
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

    let reference = definition.build().await;
    sleep(Duration::from_secs(2)).await;
    assert!(backend.is_image_present(&reference).await);
    sleep(Duration::from_secs(2)).await;

    let definition2 = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-hash"),
        dockerfile,
    );
    let reference2 = definition2.build().await;
    sleep(Duration::from_secs(2)).await;
    assert_eq!(reference, reference2);

    backend.remove_image(&reference).await;
    sleep(Duration::from_secs(2)).await;
}

#[tokio::test]
async fn test_image_build_from_directory_hash() {
    let backend = ociman::test_backend_setup!();

    let definition = ociman::BuildDefinition::from_directory_hash(
        &backend,
        ociman::testing::test_name("ociman-test-dir-hash"),
        "tests/fixtures/test-build-hash",
    );

    let reference1 = definition.build().await;
    sleep(Duration::from_secs(2)).await;
    assert!(backend.is_image_present(&reference1).await);
    sleep(Duration::from_secs(2)).await;

    let definition2 = ociman::BuildDefinition::from_directory_hash(
        &backend,
        ociman::testing::test_name("ociman-test-dir-hash"),
        "tests/fixtures/test-build-hash",
    );
    let reference2 = definition2.build().await;
    sleep(Duration::from_secs(2)).await;
    assert_eq!(reference1, reference2);

    backend.remove_image(&reference1).await;
    sleep(Duration::from_secs(2)).await;
}

#[tokio::test]
async fn test_image_build_with_build_args() {
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

    let reference = definition.build().await;
    assert!(backend.is_image_present(&reference).await);

    // Verify the build arg was used by checking the file created during build
    let def = test_definition(&backend, reference.clone())
        .entrypoint("cat")
        .arguments(["/test-output"]);

    let output = def.run_capture_only_stdout().await;
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert!(stdout.contains("test_value"));

    backend.remove_image(&reference).await;
}

#[tokio::test]
async fn test_image_build_args_affect_hash() {
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
    let reference1 = definition1.build().await;

    let definition2 = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-args-hash"),
        dockerfile,
    )
    .build_argument("VERSION".parse().unwrap(), "2.0");
    let reference2 = definition2.build().await;

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
    let reference3 = definition3.build().await;

    assert_eq!(
        reference1, reference3,
        "Same build arguments should produce same image tag"
    );

    backend.remove_image(&reference1).await;
    backend.remove_image(&reference2).await;
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

#[tokio::test]
async fn test_run_with_successful_exit() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap()).entrypoint("true");

    assert!(definition.run().await.is_ok());
}

#[tokio::test]
async fn test_run_with_nonzero_exit() {
    let backend = ociman::test_backend_setup!();

    let definition =
        test_definition(&backend, "alpine:latest".parse().unwrap()).entrypoint("false");

    let error = definition.run().await.unwrap_err();
    assert_eq!(error.exit_status.map(|status| status.code()), Some(Some(1)));
}

#[tokio::test]
async fn test_container_with_workdir() {
    let backend = ociman::test_backend_setup!();

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("pwd")
        .workdir("/tmp");

    let output = definition.run_capture_only_stdout().await;
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "/tmp");
}

#[tokio::test]
async fn test_container_commit() {
    let backend = ociman::test_backend_setup!();

    let target_reference: ociman::image::Reference =
        ociman::testing::test_reference("ociman-test-commit:latest");

    // Ensure target image doesn't exist before test
    if backend.is_image_present(&target_reference).await {
        backend.remove_image(&target_reference).await;
    }

    let definition = test_definition(&backend, "alpine:latest".parse().unwrap())
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    let commit_target = target_reference.clone();
    definition
        .with_container(async |container| {
            container
                .exec("touch")
                .argument("/committed-file")
                .status()
                .await
                .unwrap();
            container.commit(&commit_target, true).await.unwrap();
        })
        .await;

    assert!(
        backend.is_image_present(&target_reference).await,
        "Committed image should exist"
    );

    // Verify the committed file exists in the new image
    let verify_definition = test_definition(&backend, target_reference.clone())
        .entrypoint("ls")
        .arguments(["/committed-file"]);

    let output = verify_definition.run_capture_only_stdout().await;
    let stdout = std::str::from_utf8(&output).expect("invalid utf8 in output");

    assert_eq!(stdout.trim(), "/committed-file");

    backend.remove_image(&target_reference).await;
}
