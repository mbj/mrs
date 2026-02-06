# cmd-proc - Process Command Builder

A wrapper around `tokio::process::Command` providing debug logging, stronger input types, and a fluent builder API with automatic exit code checking.

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

## Why cmd-proc?

`std::process::Command` (and its async wrapper `tokio::process::Command`) is powerful but requires verbose boilerplate for common patterns:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // std/tokio process - capturing stdout as string
    let output = std::process::Command::new("echo")
        .arg("hello")
        .output()?;
    if !output.status.success() {
        return Err("non-zero exit".into());
    }
    let stdout = String::from_utf8(output.stdout)?;
    assert_eq!(stdout, "hello\n");

    // cmd-proc - same operation
    let stdout = cmd_proc::Command::new("echo")
        .argument("hello")
        .stdout_capture()
        .string().await?;
    assert_eq!(stdout, "hello\n");
    Ok(())
}
```

### Key differences from `std` / `tokio` process

| Feature              | `std` / `tokio` process                 | `cmd-proc`                                                                  |
|----------------------|-----------------------------------------|-----------------------------------------------------------------------------|
| **Debug logging**    | None                                    | Automatic debug logging of commands before execution                        |
| **Exit code check**  | Manual                                  | Automatic - non-zero exits return `Err`                                     |
| **Output capture**   | Returns raw `Output` struct             | Typestate: `.stdout_capture().string()`, `.stderr_capture().bytes()`        |
| **Builder pattern**  | Mutable references (`&mut self`)        | Owned builder with method chaining                                          |
| **Stdin data**       | Requires manual pipe setup              | Simple `.stdin_bytes()` method                                              |
| **Env var names**    | Accepts any `AsRef<OsStr>`              | `EnvVariableName` type with compile-time validation                         |
| **Error type**       | Separate `io::Error` and `ExitStatus`   | Unified `CommandError` with both                                            |
| **Process spawning** | Direct `.spawn()` on Command            | `.build()` for native `tokio::process::Command` access                      |

### Design philosophy

- **Debug logging**: Every command execution is logged via the `log` crate at debug level, making it easy to trace what commands are being run.
- **Stronger input types**: `EnvVariableName` prevents invalid environment variable names (empty or containing `=`) at compile time rather than runtime.
- **Typestate capture pattern**: Stream capture methods transition between builder types (`Command` -> `CaptureSingle<S>` -> `CaptureAll`), enforcing valid API usage at compile time. You can't call `.bytes()` on a double-capture builder, and you can't accidentally forget which stream you're capturing.
- **Default exit code checking**: Capture methods treat non-zero exit as an error by default, preventing accidentally ignored failures. Use `.accept_nonzero_exit()` to opt out when needed.
- **Fluent API**: Chain configuration methods naturally without `&mut self` gymnastics.

## Usage

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cmd_proc::Command;

    // Capture stdout as string
    let output = Command::new("echo")
        .argument("hello")
        .stdout_capture()
        .string().await?;
    assert_eq!(output, "hello\n");

    // Capture stderr as bytes
    let errors = Command::new("sh")
        .arguments(["-c", "echo err >&2"])
        .stderr_capture()
        .bytes().await?;
    assert_eq!(errors, b"err\n");

    // Run without capturing (just check success)
    Command::new("true")
        .status().await?;

    // Capture both stdout and stderr
    let result = Command::new("sh")
        .arguments(["-c", "echo out; echo err >&2; exit 1"])
        .stdout_capture()
        .stderr_capture()
        .accept_nonzero_exit()
        .run().await?;
    assert_eq!(result.stdout, b"out\n");
    assert_eq!(result.stderr, b"err\n");

    // Pass stdin data
    let output = Command::new("cat")
        .stdin_bytes(b"hello world")
        .stdout_capture()
        .string().await?;
    assert_eq!(output, "hello world");

    // Redirect streams to /dev/null
    Command::new("sh")
        .arguments(["-c", "echo noise >&2"])
        .stderr_null()
        .status().await?;
    Ok(())
}
```

## Stream Capture Typestate

Stream configuration uses a typestate pattern with three builder types. Each method consumes `self` and returns the appropriate type, enforcing valid transitions at compile time.

### State transition matrix

| From                     | Method              | To                       |
|--------------------------|---------------------|--------------------------|
| `Command`                | `.stdout_capture()` | `CaptureSingle<Stdout>`  |
| `Command`                | `.stderr_capture()` | `CaptureSingle<Stderr>`  |
| `Command`                | `.stdout_null()`    | `Command`                |
| `Command`                | `.stderr_null()`    | `Command`                |
| `Command`                | `.stdout_inherit()` | `Command`                |
| `Command`                | `.stderr_inherit()` | `Command`                |
| `CaptureSingle<Stdout>`  | `.stderr_capture()` | `CaptureAll`             |
| `CaptureSingle<Stdout>`  | `.stderr_null()`    | `CaptureSingle<Stdout>`  |
| `CaptureSingle<Stdout>`  | `.stderr_inherit()` | `CaptureSingle<Stdout>`  |
| `CaptureSingle<Stdout>`  | `.stdout_null()`    | `Command`                |
| `CaptureSingle<Stdout>`  | `.stdout_inherit()` | `Command`                |
| `CaptureSingle<Stderr>`  | `.stdout_capture()` | `CaptureAll`             |
| `CaptureSingle<Stderr>`  | `.stdout_null()`    | `CaptureSingle<Stderr>`  |
| `CaptureSingle<Stderr>`  | `.stdout_inherit()` | `CaptureSingle<Stderr>`  |
| `CaptureSingle<Stderr>`  | `.stderr_null()`    | `Command`                |
| `CaptureSingle<Stderr>`  | `.stderr_inherit()` | `Command`                |
| `CaptureAll`             | `.stdout_null()`    | `CaptureSingle<Stderr>`  |
| `CaptureAll`             | `.stdout_inherit()` | `CaptureSingle<Stderr>`  |
| `CaptureAll`             | `.stderr_null()`    | `CaptureSingle<Stdout>`  |
| `CaptureAll`             | `.stderr_inherit()` | `CaptureSingle<Stdout>`  |

### Terminal methods

| Type               | Method      | Return                                      |
|--------------------|-------------|---------------------------------------------|
| `Command`          | `.status()` | `Result<(), CommandError>`                  |
| `Command`          | `.build()`  | `tokio::process::Command`                   |
| `CaptureSingle<S>` | `.run()`    | `Result<CaptureSingleResult, CommandError>` |
| `CaptureSingle<S>` | `.bytes()`  | `Result<Vec<u8>, CommandError>`             |
| `CaptureSingle<S>` | `.string()` | `Result<String, CommandError>`              |
| `CaptureAll`       | `.run()`    | `Result<CaptureAllResult, CommandError>`    |

### Result types

- **`CaptureSingleResult`**: `bytes: Vec<u8>`, `status: ExitStatus`
- **`CaptureAllResult`**: `stdout: Vec<u8>`, `stderr: Vec<u8>`, `status: ExitStatus`

All builders support `.accept_nonzero_exit()` to allow non-zero exit codes without error.

## Arguments vs Options

In CLI terminology:
- **Argument**: positional value (e.g., `git clone <url>`)
- **Option**: named parameter with value (e.g., `--message "text"`, `-o file`)

cmd-proc provides methods for both:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cmd_proc::Command;

    // Arguments (positional)
    let output = Command::new("echo")
        .argument("one")                       // single argument
        .arguments(["two", "three"])           // multiple arguments
        .optional_argument(Some("four"))       // argument only if Some
        .optional_argument(None::<&str>)       // skipped when None
        .stdout_capture()
        .string().await?;
    assert_eq!(output, "one two three four\n");

    // Options (name + value pairs)
    let output = Command::new("echo")
        .option("-n", "hello")                           // required option
        .optional_option("--unused", None::<&str>)       // skipped when None
        .stdout_capture()
        .string().await?;
    assert_eq!(output, "hello");

    // optional_option simplifies the common flag + value pattern:
    let maybe_author: Option<&str> = Some("Alice");
    let output = Command::new("echo")
        .argument("-n")
        .optional_option("--author", maybe_author)
        .stdout_capture()
        .string().await?;
    assert_eq!(output, "--author Alice");
    Ok(())
}
```

## Native Process Access

For interactive processes or other use cases beyond capture, use `.build()` to get
the underlying `tokio::process::Command`:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cmd_proc::Command;
    use tokio::io::AsyncBufReadExt;

    let mut child = Command::new("echo")
        .argument("hello")
        .build()
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;

    // Read a line from stdout
    let mut line = String::new();
    tokio::io::BufReader::new(child.stdout.as_mut().unwrap())
        .read_line(&mut line)
        .await?;
    assert_eq!(line, "hello\n");

    // Close stdin to signal shutdown
    drop(child.stdin.take());

    // Wait for exit
    child.wait().await?;
    Ok(())
}
```

## Environment Variables

Environment variable names are validated via `EnvVariableName`:

- **Cannot be empty** - an empty name is meaningless
- **Cannot contain `=`** - the OS uses `=` as separator between name and value; a name containing `=` corrupts the environment block

`std::process::Command::env()` silently accepts invalid names, causing mysterious runtime failures. `EnvVariableName` catches errors at compile time (when `from_static_or_panic` is used in a `const` context) or parse time (`FromStr`).

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cmd_proc::{Command, EnvVariableName};

    // Compile-time validated (panics at compile time if invalid)
    const MY_VAR: EnvVariableName = EnvVariableName::from_static_or_panic("MY_VAR");
    let output = Command::new("sh")
        .arguments(["-c", "echo $MY_VAR"])
        .env(&MY_VAR, "hello")
        .stdout_capture()
        .string().await?;
    assert_eq!(output, "hello\n");

    // Set multiple variables from an iterator
    let vars = [
        (EnvVariableName::from_static_or_panic("FOO"), "1"),
        (EnvVariableName::from_static_or_panic("BAR"), "2"),
    ];
    Command::new("sh")
        .arguments(["-c", "true"])
        .envs(vars)
        .status().await?;

    // Remove a variable from the child environment
    const PATH: EnvVariableName = EnvVariableName::from_static_or_panic("PATH");
    Command::new("sh")
        .arguments(["-c", "true"])
        .env_remove(&PATH)
        .status().await?;
    Ok(())
}
```

## Error Handling

`CommandError` has two mutually exclusive failure modes: an IO error
(command not found, permission denied) or a non-zero exit status — never both.

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cmd_proc::Command;

    // IO error — command not found
    let error = Command::new("./nonexistent").status().await.unwrap_err();
    assert!(error.io_error.is_some());
    assert!(error.exit_status.is_none());

    // Non-zero exit status
    let error = Command::new("false").status().await.unwrap_err();
    assert!(error.io_error.is_none());
    assert!(error.exit_status.is_some());
    Ok(())
}
```
