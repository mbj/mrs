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

fn alpine_test_definition(backend: &ociman::Backend) -> ociman::Definition {
    test_definition(backend, ociman::testing::ALPINE_LATEST_IMAGE.clone())
}

fn alpine_dockerfile(body: &str) -> String {
    format!("FROM {}\n{body}", *ociman::testing::ALPINE_LATEST_IMAGE)
}

#[tokio::test]
async fn test_hello_world() {
    let backend = ociman::test_backend_setup!();

    let definition = alpine_test_definition(&backend)
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

    let definition = alpine_test_definition(&backend)
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

    let definition = alpine_test_definition(&backend)
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition
        .with_container(async |container| {
            let output = container
                .exec("echo")
                .argument("Container is running!")
                .build()
                .stdout_capture()
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

    let definition = alpine_test_definition(&backend)
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

    let definition = alpine_test_definition(&backend)
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition
        .with_container(async |container| {
            let error = container.exec("false").status().await.unwrap_err();
            let cmd_proc::CommandError::ExitStatus(status) = error else {
                panic!("expected ExitStatus, got {error:?}");
            };
            assert_eq!(status.code(), Some(1));
        })
        .await;
}

#[tokio::test]
async fn test_container_exec_with_stdin() {
    let backend = ociman::test_backend_setup!();

    let definition = alpine_test_definition(&backend)
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition
        .with_container(async |container| {
            let output = container
                .exec("cat")
                .stdin(b"hello from stdin")
                .build()
                .stdout_capture()
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

    let definition = alpine_test_definition(&backend)
        .entrypoint("sh".to_string())
        .arguments(vec![
            "-c".to_string(),
            "trap 'exit 0' TERM; sleep 30 & wait".to_string(),
        ])
        .publish(ociman::Publish::tcp(8080).host_ip(std::net::Ipv4Addr::LOCALHOST.into()));

    definition
        .with_container(async |container| {
            let host_port = container.read_host_tcp_port(8080).await.unwrap();

            assert!(host_port > 0);
        })
        .await;
}

#[tokio::test]
async fn test_read_host_tcp_port_not_published() {
    let backend = ociman::test_backend_setup!();

    let definition = alpine_test_definition(&backend)
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"]);

    definition
        .with_container(async |container| {
            let error = container.read_host_tcp_port(8080).await.unwrap_err();

            assert!(matches!(
                error,
                ociman::ReadHostTcpPortError::NotPublished {
                    container_port: 8080
                }
            ));
        })
        .await;
}

#[tokio::test]
async fn test_container_labels_roundtrip() {
    use ociman::label;

    let backend = ociman::test_backend_setup!();

    const MANAGED_KEY: label::Key = label::Key::from_static_or_panic("ociman-test.managed");
    const MANAGED_VALUE: label::Value = label::Value::from_static_or_panic("1");
    const SESSION_KEY: label::Key = label::Key::from_static_or_panic("ociman-test.session");
    const SESSION_VALUE: label::Value = label::Value::from_static_or_panic("integration");

    let definition = alpine_test_definition(&backend)
        .entrypoint("sh")
        .arguments(["-c", "trap 'exit 0' TERM; sleep 30 & wait"])
        .label(&SESSION_KEY, &SESSION_VALUE)
        .label(&MANAGED_KEY, &MANAGED_VALUE);

    definition
        .with_container(async |container| {
            let labels = container.labels().await.unwrap();

            let mut iter = labels.iter();

            let (key, value) = iter.next().unwrap();
            assert_eq!(key, &MANAGED_KEY);
            assert_eq!(value, &MANAGED_VALUE);

            let (key, value) = iter.next().unwrap();
            assert_eq!(key, &SESSION_KEY);
            assert_eq!(value, &SESSION_VALUE);

            assert!(iter.next().is_none());
        })
        .await;
}

#[tokio::test]
async fn test_list_containers_by_label() {
    use ociman::label;

    let backend = ociman::test_backend_setup!();

    const MARKER: label::Key =
        label::Key::from_static_or_panic("ociman-test.list-containers-marker");
    const SESSION: label::Key =
        label::Key::from_static_or_panic("ociman-test.list-containers-session");

    let marker_value = label::Value::from_static_or_panic("present");
    let alpha = label::Value::from_static_or_panic("alpha");
    let beta = label::Value::from_static_or_panic("beta");
    let unknown = label::Value::from_static_or_panic("nonexistent");

    // Sweep any leftover containers from a prior failed run sharing the marker.
    for mut container in backend
        .list_containers_by_label(&MARKER, None)
        .await
        .unwrap()
    {
        container.stop().await;
        container.remove().await;
    }

    let sleep_args = ["-c", "trap 'exit 0' TERM; sleep 30 & wait"];

    let mut a = ociman::Definition::new(
        backend.clone(),
        ociman::testing::ALPINE_LATEST_IMAGE.clone(),
    )
    .entrypoint("sh")
    .arguments(sleep_args)
    .label(&MARKER, &marker_value)
    .label(&SESSION, &alpha)
    .run_detached()
    .await;
    let mut b = ociman::Definition::new(
        backend.clone(),
        ociman::testing::ALPINE_LATEST_IMAGE.clone(),
    )
    .entrypoint("sh")
    .arguments(sleep_args)
    .label(&MARKER, &marker_value)
    .label(&SESSION, &beta)
    .run_detached()
    .await;
    let mut c = ociman::Definition::new(
        backend.clone(),
        ociman::testing::ALPINE_LATEST_IMAGE.clone(),
    )
    .entrypoint("sh")
    .arguments(sleep_args)
    .label(&MARKER, &marker_value)
    .run_detached()
    .await;

    // (1) Key only — all three.
    let by_marker = backend
        .list_containers_by_label(&MARKER, None)
        .await
        .unwrap();
    assert_eq!(by_marker.len(), 3);

    // (2) Key + value — exactly A.
    let by_alpha = backend
        .list_containers_by_label(&SESSION, Some(&alpha))
        .await
        .unwrap();
    assert_eq!(by_alpha.len(), 1);
    let labels = by_alpha[0].labels().await.unwrap();
    assert_eq!(labels.get(&SESSION).unwrap(), &alpha);
    assert_eq!(labels.get(&MARKER).unwrap(), &marker_value);

    // (3) Nonexistent value — empty.
    let none = backend
        .list_containers_by_label(&SESSION, Some(&unknown))
        .await
        .unwrap();
    assert!(none.is_empty());

    // (4) Stopped container is still listed (--all).
    a.stop().await;
    let after_stop = backend
        .list_containers_by_label(&MARKER, None)
        .await
        .unwrap();
    assert_eq!(after_stop.len(), 3);

    // Cleanup.
    a.remove().await;
    b.stop().await;
    b.remove().await;
    c.stop().await;
    c.remove().await;
}

#[tokio::test]
async fn test_command_with_stdin() {
    let input = b"Hello from stdin!";
    let output = cmd_proc::Command::new("cat")
        .stdin_bytes(input.to_vec())
        .stdout_capture()
        .bytes()
        .await
        .unwrap();

    assert_eq!(output, input);
}

#[tokio::test]
async fn test_image_build_from_instructions() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = alpine_dockerfile(indoc! {"
        RUN echo 'test_image_build_from_instructions' > /test-id
        RUN touch dirty && echo 'test build from instructions'
    "});

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

    let dockerfile = alpine_dockerfile(indoc! {"
        RUN echo 'test_image_build_if_absent' > /test-id
        RUN touch dirty && echo 'test build if absent'
    "});

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
async fn test_image_labels_roundtrip() {
    use ociman::label;

    let backend = ociman::test_backend_setup!();

    const ROLE_KEY: label::Key = label::Key::from_static_or_panic("ociman-test.role");
    const ROLE_VALUE: label::Value = label::Value::from_static_or_panic("integration");
    const BUILD_KEY: label::Key = label::Key::from_static_or_panic("ociman-test.build");
    const BUILD_VALUE: label::Value = label::Value::from_static_or_panic("1");

    let reference: ociman::image::Reference =
        ociman::testing::test_reference("ociman-test-image-labels:latest");

    if backend.is_image_present(&reference).await {
        backend.remove_image_force(&reference).await;
    }

    let dockerfile = alpine_dockerfile("RUN touch /label-test\n");

    let definition =
        ociman::BuildDefinition::from_instructions(&backend, reference.clone(), dockerfile)
            .label(&ROLE_KEY, &ROLE_VALUE)
            .label(&BUILD_KEY, &BUILD_VALUE);

    let built = definition.build().await;
    assert!(backend.is_image_present(&built).await);

    let labels = backend.image_labels(&built).await.unwrap();

    // Image labels returned by inspect may include image-level defaults set
    // by the base image or the builder. Check ours are present and correct
    // without asserting the full iteration sequence.
    assert_eq!(labels.get(&ROLE_KEY).unwrap(), &ROLE_VALUE);
    assert_eq!(labels.get(&BUILD_KEY).unwrap(), &BUILD_VALUE);
    assert!(labels.contains_key(&ROLE_KEY));
    assert!(labels.contains_key(&BUILD_KEY));

    let unknown = label::Key::from_static_or_panic("ociman-test.unknown");
    assert!(labels.get(&unknown).is_none());

    backend.remove_image_force(&built).await;
}

#[tokio::test]
async fn test_image_labels_affect_content_hash() {
    use ociman::label;

    let backend = ociman::test_backend_setup!();

    const ROLE_KEY: label::Key = label::Key::from_static_or_panic("ociman-test.role");

    let dockerfile = alpine_dockerfile("RUN touch /label-hash-test\n");

    let without_label = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-label-hash"),
        &*dockerfile,
    );

    let with_label_a = without_label
        .clone()
        .label(&ROLE_KEY, &label::Value::from_static_or_panic("a"));

    let with_label_b = without_label
        .clone()
        .label(&ROLE_KEY, &label::Value::from_static_or_panic("b"));

    let reference_none = without_label.target_reference();
    let reference_a = with_label_a.target_reference();
    let reference_b = with_label_b.target_reference();

    assert_ne!(
        reference_none, reference_a,
        "adding a label must change the content-addressed tag",
    );
    assert_ne!(
        reference_a, reference_b,
        "changing a label value must change the content-addressed tag",
    );
    assert_ne!(reference_none, reference_b);

    // Reconstructing with identical inputs must reproduce the same tag.
    let with_label_a_again = without_label
        .clone()
        .label(&ROLE_KEY, &label::Value::from_static_or_panic("a"));
    assert_eq!(reference_a, with_label_a_again.target_reference());
}

#[tokio::test]
async fn test_image_tag() {
    let backend = ociman::test_backend_setup!();

    let source = ociman::testing::ALPINE_LATEST_IMAGE.clone();
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

    let reference = ociman::testing::ALPINE_LATEST_IMAGE.clone();

    backend.pull_image_if_absent(&reference).await;
    assert!(backend.is_image_present(&reference).await);
}

#[tokio::test]
async fn test_image_build_from_instructions_hash() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = alpine_dockerfile(indoc! {"
        RUN echo 'test_image_build_from_instructions_hash' > /test-id
        RUN touch dirty && echo 'test hash'
    "});

    let definition = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-hash"),
        &*dockerfile,
    );

    let reference = definition.build().await;
    assert!(backend.is_image_present(&reference).await);

    let definition2 = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-hash"),
        &*dockerfile,
    );
    let reference2 = definition2.build().await;
    assert_eq!(reference, reference2);

    backend.remove_image(&reference).await;
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
    assert!(backend.is_image_present(&reference1).await);

    let definition2 = ociman::BuildDefinition::from_directory_hash(
        &backend,
        ociman::testing::test_name("ociman-test-dir-hash"),
        "tests/fixtures/test-build-hash",
    );
    let reference2 = definition2.build().await;
    assert_eq!(reference1, reference2);

    backend.remove_image(&reference1).await;
}

#[tokio::test]
async fn test_image_build_with_build_args() {
    let backend = ociman::test_backend_setup!();

    let dockerfile = alpine_dockerfile(indoc! {"
        RUN echo 'test_image_build_with_build_args' > /test-id
        ARG TEST_ARG
        RUN touch dirty && echo \"Build arg value: $TEST_ARG\" > /test-output
    "});

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

    let dockerfile = alpine_dockerfile(indoc! {"
        RUN echo 'test_image_build_args_affect_hash' > /test-id
        ARG VERSION
        RUN touch dirty && echo \"Version: $VERSION\"
    "});

    let definition1 = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-args-hash"),
        &*dockerfile,
    )
    .build_argument("VERSION".parse().unwrap(), "1.0");
    let reference1 = definition1.build().await;

    let definition2 = ociman::BuildDefinition::from_instructions_hash(
        &backend,
        ociman::testing::test_name("ociman-test-args-hash"),
        &*dockerfile,
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
        &*dockerfile,
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

    let definition = alpine_test_definition(&backend).entrypoint("true");

    assert!(definition.run().await.is_ok());
}

#[tokio::test]
async fn test_run_with_nonzero_exit() {
    let backend = ociman::test_backend_setup!();

    let definition = alpine_test_definition(&backend).entrypoint("false");

    let error = definition.run().await.unwrap_err();
    let cmd_proc::CommandError::ExitStatus(status) = error else {
        panic!("expected ExitStatus, got {error:?}");
    };
    assert_eq!(status.code(), Some(1));
}

#[tokio::test]
async fn test_container_with_workdir() {
    let backend = ociman::test_backend_setup!();

    let definition = alpine_test_definition(&backend)
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

    let definition = alpine_test_definition(&backend)
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

#[tokio::test]
async fn test_bridge_subnets() {
    let backend = ociman::test_backend_setup!();

    let subnets = backend.bridge_subnets().await.unwrap();

    assert!(!subnets.is_empty(), "Expected at least one bridge subnet");

    for subnet in &subnets {
        assert!(
            subnet.prefix_len() < 32,
            "Expected a network subnet, got host: {subnet}"
        );
    }
}
