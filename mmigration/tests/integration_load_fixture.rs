use std::path::PathBuf;

use indoc::indoc;
use mmigration::{AddError, DefinedMigrations, LoadError, PendingError, PendingMigration};
use pretty_assertions::assert_eq;

fn fixture_dir(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

#[test]
fn load_defined_migrations_from_fixture_directory() {
    let migrations = DefinedMigrations::load(&fixture_dir("basic")).unwrap();

    let actual: Vec<PendingMigration> = migrations
        .select_pending(None)
        .unwrap()
        .into_iter()
        .cloned()
        .collect();

    let expected = vec![
        PendingMigration {
            index: 0_u32.into(),
            name: "init_schema".parse().unwrap(),
            raw_sql: "CREATE SCHEMA app;\n".into(),
        },
        PendingMigration {
            index: 1_u32.into(),
            name: "add_users".parse().unwrap(),
            raw_sql: "CREATE TABLE app.users (id bigint PRIMARY KEY);\n".into(),
        },
    ];

    assert_eq!(expected, actual);
}

#[test]
fn load_defined_migrations_from_file_path_returns_read_dir_error() {
    let file_path = fixture_dir("basic").join("0_init_schema.sql");

    let error = DefinedMigrations::load(&file_path).unwrap_err();

    let LoadError::IoError {
        operation,
        path,
        source,
    } = error
    else {
        panic!("expected IoError");
    };

    assert_eq!("read_dir", operation);
    assert_eq!(file_path, path);
    assert_eq!(std::io::ErrorKind::NotADirectory, source.kind());
}

#[test]
fn load_defined_migrations_with_directory_entry_returns_non_file_entry_error() {
    let non_file_path = fixture_dir("contains_subdir").join("not_a_migration");

    let error = DefinedMigrations::load(&fixture_dir("contains_subdir")).unwrap_err();

    let LoadError::NonFileEntry { path } = error else {
        panic!("expected NonFileEntry");
    };

    assert_eq!(non_file_path, path);
}

#[test]
fn load_defined_migrations_with_invalid_filename_returns_error() {
    let invalid_file_path = fixture_dir("invalid_filename").join("invalid.sql");

    let error = DefinedMigrations::load(&fixture_dir("invalid_filename")).unwrap_err();

    let LoadError::InvalidFileName { path, report } = error else {
        panic!("expected InvalidFileName");
    };

    let expected_report = indoc! {"
        0: at line 1, in Digit:
        invalid.sql
        ^

        1: at line 1, in migration index:
        invalid.sql
        ^

        2: at line 1, in migration filename ({index}_{name}.sql):
        invalid.sql
        ^

    "};

    assert_eq!(invalid_file_path, path);
    assert_eq!(expected_report, report);
}

#[test]
fn load_defined_migrations_with_non_consecutive_indexes_returns_add_error() {
    let error = DefinedMigrations::load(&fixture_dir("non_consecutive_after_initial")).unwrap_err();

    let LoadError::AddError { source } = error else {
        panic!("expected AddError");
    };

    assert_eq!(
        AddError::NonConsecutive {
            expected: 1_u32.into(),
            got: 2_u32.into(),
        },
        source
    );
}

#[test]
fn load_defined_migrations_with_non_initial_index_returns_pending_add_error() {
    let error = DefinedMigrations::load(&fixture_dir("initial_non_zero")).unwrap_err();

    let LoadError::AddError { source } = error else {
        panic!("expected AddError");
    };

    assert_eq!(
        AddError::PendingError {
            source: PendingError::InitialIndex { got: 1_u32.into() }
        },
        source
    );
}

// macOS rejects creating path components with invalid byte sequences (EILSEQ),
// so this test can only run on Unix platforms that allow non-UTF-8 filenames.
#[cfg(all(unix, not(target_os = "macos")))]
#[test]
fn load_defined_migrations_with_non_utf8_filename_returns_error() {
    use std::os::unix::ffi::OsStringExt;

    let base = std::env::temp_dir().join(format!(
        "mmigration-non-utf8-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&base).unwrap();

    let invalid_name = std::ffi::OsString::from_vec(vec![b'1', b'_', 0xff, b'.', b's', b'q', b'l']);
    let file_path = base.join(&invalid_name);
    std::fs::write(&file_path, "SELECT 1;\n").unwrap();

    let error = DefinedMigrations::load(&base).unwrap_err();

    let LoadError::NonUtf8FileName { path } = error else {
        panic!("expected NonUtf8FileName");
    };

    assert_eq!(file_path, path);

    std::fs::remove_file(&file_path).unwrap();
    std::fs::remove_dir(&base).unwrap();
}
