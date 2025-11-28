mod common;

use common::{TestDir, TestGitRepo};

#[test]
fn test_cache_status() {
    let repo = TestGitRepo::new("cache-test");

    repo.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");
    repo.write_file("data.sql", "INSERT INTO users (id) VALUES (1);");
    let commit_hash = repo.commit("Initial");

    let config_content = indoc::formatdoc! {r#"
        image = "17.1"

        [instances.main.seeds.a-schema]
        type = "sql-file"
        path = "schema.sql"

        [instances.main.seeds.b-data-from-git]
        type = "sql-file"
        path = "data.sql"
        git_revision = "{commit_hash}"

        [instances.main.seeds.c-run-command]
        type = "command"
        command = "echo"
        arguments = ["hello"]
        cache.type = "command-hash"

        [instances.main.seeds.d-run-script]
        type = "script"
        script = "echo 'hello world'"
    "#};
    repo.write_file("database.toml", &config_content);

    let expected = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "a-schema"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
        status = "miss"

        [[seeds]]
        name = "b-data-from-git"
        type = "sql-file-git-revision"
        cache_image = "pg-ephemeral/main:9d1fe3c033a5353478c44008c8c34a1223ab208664d43a1c63b22172c9d9c645"
        status = "miss"

        [[seeds]]
        name = "c-run-command"
        type = "command"
        cache_image = "pg-ephemeral/main:9c76bdb2e278bff90fa66ea86693deaeab85d2b84ff009d8c31d5d8b0c857e88"
        status = "miss"

        [[seeds]]
        name = "d-run-script"
        type = "script"
        cache_image = "pg-ephemeral/main:2204e587eb3ffecb4d3c372f127cbaf26a3c4df88a63b1fa86ec26036d36ecdc"
        status = "miss"
    "#};

    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&repo.path)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
}

#[test]
fn test_cache_status_deterministic() {
    let dir = TestDir::new("cache-deterministic-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let expected = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "schema"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
        status = "miss"
    "#};

    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output.status.success());
    assert_eq!(String::from_utf8(output.stdout).unwrap(), expected);
}

#[test]
fn test_cache_status_change_with_content() {
    let dir = TestDir::new("cache-changes-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let expected_before = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "schema"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
        status = "miss"
    "#};

    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output1 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output1.status.success());
    assert_eq!(String::from_utf8(output1.stdout).unwrap(), expected_before);

    dir.write_file(
        "schema.sql",
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT);",
    );

    let output2 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output2.status.success());
    // Cache image should change when content changes - just verify it's different
    assert_ne!(String::from_utf8(output2.stdout).unwrap(), expected_before);
}

#[test]
fn test_cache_status_change_with_image() {
    let dir = TestDir::new("cache-image-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let expected_before = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "schema"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:6ca66287ad925179b556edbe98c6e813ffd02e1ed129cc4bea99e10f610f656a"
        status = "miss"
    "#};

    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output1 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output1.status.success());
    assert_eq!(String::from_utf8(output1.stdout).unwrap(), expected_before);

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.2"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let output2 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output2.status.success());
    // Cache image should change when image changes - just verify it's different
    assert_ne!(String::from_utf8(output2.stdout).unwrap(), expected_before);
}

#[test]
fn test_cache_status_chain_propagates() {
    let dir = TestDir::new("cache-chain-test");

    dir.write_file("first.sql", "CREATE TABLE first (id INTEGER);");
    dir.write_file("second.sql", "CREATE TABLE second (id INTEGER);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.a-first]
            type = "sql-file"
            path = "first.sql"

            [instances.main.seeds.b-second]
            type = "sql-file"
            path = "second.sql"
        "#},
    );

    let expected_before = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "a-first"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:f8b223da0e4cf8d8db2add4450845e7563bf9800e27773f3301cdf7740cf6176"
        status = "miss"

        [[seeds]]
        name = "b-second"
        type = "sql-file"
        cache_image = "pg-ephemeral/main:96482a6449ac57c34b002ba936158de8ab89819238bd18863c37450d6915cd8e"
        status = "miss"
    "#};

    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output1 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output1.status.success());
    assert_eq!(String::from_utf8(output1.stdout).unwrap(), expected_before);

    dir.write_file("first.sql", "CREATE TABLE first (id INTEGER, name TEXT);");

    let output2 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output2.status.success());
    // Cache image should change when first seed changes, and propagate to second seed
    assert_ne!(String::from_utf8(output2.stdout).unwrap(), expected_before);
}

#[test]
fn test_cache_status_key_command() {
    let dir = TestDir::new("cache-key-command-test");

    dir.write_file("version.txt", "1.0.0");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.run-migrations]
            type = "command"
            command = "migrate"
            arguments = ["up"]

            [instances.main.seeds.run-migrations.cache]
            type = "key-command"
            command = "cat"
            arguments = ["version.txt"]
        "#},
    );

    let expected_before = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "run-migrations"
        type = "command"
        cache_image = "pg-ephemeral/main:7f881e9f75f7767a96ff07d2ee9649c0754a228cd69517e31928107abb11b256"
        status = "miss"
        cache_key_output = "1.0.0"
    "#};

    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output1 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output1.status.success());
    assert_eq!(String::from_utf8(output1.stdout).unwrap(), expected_before);

    // Change the version file - cache image should change
    dir.write_file("version.txt", "2.0.0");

    let output2 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output2.status.success());
    // Cache image should change when key command output changes
    assert_ne!(String::from_utf8(output2.stdout).unwrap(), expected_before);
}

#[test]
fn test_cache_status_output_truncation_and_verbose() {
    let dir = TestDir::new("cache-truncation-test");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.run-migrations]
            type = "command"
            command = "migrate"
            arguments = ["up"]

            [instances.main.seeds.run-migrations.cache]
            type = "key-script"
            script = "echo 'line1'; echo 'line2'; echo 'line3'"
        "#},
    );

    let expected_truncated = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "run-migrations"
        type = "command"
        cache_image = "pg-ephemeral/main:c0bb68b81534ae662400ea38803ee43940c9a96b20c625bbd517dac8018511e8"
        status = "miss"
        cache_key_output = "line1 [...2 more lines]"
    "#};

    let expected_verbose = indoc::indoc! {r#"
        version = "0.0.1-pre2"
        image = "17.1"

        [[seeds]]
        name = "run-migrations"
        type = "command"
        cache_image = "pg-ephemeral/main:c0bb68b81534ae662400ea38803ee43940c9a96b20c625bbd517dac8018511e8"
        status = "miss"
        cache_key_output = """
        line1
        line2
        line3
        """
    "#};

    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    // Without --verbose: truncated output
    let output1 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output1.status.success());
    assert_eq!(
        String::from_utf8(output1.stdout).unwrap(),
        expected_truncated
    );

    // With --verbose: full output
    let output2 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status", "--verbose"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output2.status.success());
    assert_eq!(String::from_utf8(output2.stdout).unwrap(), expected_verbose);
}

#[test]
fn test_cache_status_change_with_ssl() {
    let dir = TestDir::new("cache-ssl-test");

    dir.write_file("schema.sql", "CREATE TABLE users (id INTEGER PRIMARY KEY);");

    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let pg_ephemeral_bin = env!("CARGO_BIN_EXE_pg-ephemeral");

    let output1 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output1.status.success());
    let output_no_ssl = String::from_utf8(output1.stdout).unwrap();

    // Add SSL config
    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [ssl_config]
            hostname = "localhost"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let output2 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(
        output2.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output2.stderr)
    );
    let output_with_ssl = String::from_utf8(output2.stdout).unwrap();

    // Cache key should change when SSL config is added
    assert_ne!(output_no_ssl, output_with_ssl);

    // Change SSL hostname
    dir.write_file(
        "database.toml",
        indoc::indoc! {r#"
            image = "17.1"

            [ssl_config]
            hostname = "example.com"

            [instances.main.seeds.schema]
            type = "sql-file"
            path = "schema.sql"
        "#},
    );

    let output3 = std::process::Command::new(pg_ephemeral_bin)
        .args(["cache", "status"])
        .current_dir(&dir.path)
        .output()
        .unwrap();

    assert!(output3.status.success());
    let output_different_ssl = String::from_utf8(output3.stdout).unwrap();

    // Cache key should change when SSL hostname changes
    assert_ne!(output_with_ssl, output_different_ssl);
}
