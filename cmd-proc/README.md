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
| **Redirection**      | Manual `Stdio` per stream               | Typestate: `.stdout_to_stderr()`, `.stderr_to_stdout()`                     |
| **Builder pattern**  | Mutable references (`&mut self`)        | Owned builder with method chaining                                          |
| **Stdin data**       | Requires manual pipe setup              | Simple `.stdin_bytes()` method                                              |
| **Env var names**    | Accepts any `AsRef<OsStr>`              | `EnvVariableName` type with compile-time validation                         |
| **Error type**       | Separate `io::Error` and `ExitStatus`   | Unified `CommandError` with both                                            |
| **Live output**      | `Option<ChildStdout>` per stream        | Typestate: `.stdout_stream().spawn()`, typed read ends via `Child`          |
| **Process spawning** | Direct `.spawn()` on Command            | `.spawn()` returns a typed `Child`; `.build()` for native access            |

### Design philosophy

- **Debug logging**: Every command execution is logged via the `log` crate at debug level, making it easy to trace what commands are being run.
- **Stronger input types**: `EnvVariableName` prevents invalid environment variable names (empty or containing `=`) at compile time rather than runtime.
- **UTF-8 only**: environment variable values are intentionally limited to UTF-8 text. A non-UTF-8 value is rejected with an error (`EnvVariableReadError::NotUnicode` when reading), never converted lossily.
- **Typestate stream targets**: Where stdout and stderr go is part of the `Command` type, set independently per stream. A single `.run()` serves every combination and returns exactly what was captured: `Vec<u8>` for a captured stream, `()` otherwise. `.bytes()` and `.string()` only exist when exactly one stream is captured, so you can't call them on a double-capture builder, and you can't accidentally forget which stream you're capturing.
- **Typed live streams**: `.spawn()` hands out a read end only for streams set to `Stream`, so there is no `Option` to unwrap and no stream that silently isn't piped. `Child::wait()` closes the read ends it still holds before waiting, so a child writing to a stream nobody reads gets `EPIPE` instead of blocking forever.
- **Default exit code checking**: `.run()` and the capture shortcuts treat non-zero exit as an error by default, preventing accidentally ignored failures. Use `.accept_nonzero_exit()` to opt out when needed.
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
        .run().await?;

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
        .run().await?;

    // Send the child's stdout to our stderr, keeping our stdout clean
    Command::new("echo")
        .argument("progress")
        .stdout_to_stderr()
        .run().await?;
    Ok(())
}
```

## Stream Targets

`Command<StdoutTarget, StderrTarget>` tracks where each output stream goes. Each stream has one of five targets:

| Target           | Effect                                                      | Captured as |
|------------------|-------------------------------------------------------------|-------------|
| `Parent<Stdout>` | Written to the parent process's stdout                      | `()`        |
| `Parent<Stderr>` | Written to the parent process's stderr                      | `()`        |
| `Null`           | Discarded (`/dev/null`)                                     | `()`        |
| `Capture`        | Captured by `.run()`                                        | `Vec<u8>`   |
| `Stream`         | Read by the caller while the process runs (`.spawn()` only) | `()`        |

`Command::new` starts as `Command<Parent<Stdout>, Parent<Stderr>>`: both streams go where the parent's do. `.spawn()` accepts all 25 combinations; `.run()` accepts the 16 without `Stream`, since it never hands out a read end.

### Target methods

Every method works for any current targets and changes only its own stream:

| Method                | Result                                    |
|-----------------------|-------------------------------------------|
| `.stdout_to_stdout()` | `Command<Parent<Stdout>, StderrTarget>`   |
| `.stdout_to_stderr()` | `Command<Parent<Stderr>, StderrTarget>`   |
| `.stdout_null()`      | `Command<Null, StderrTarget>`             |
| `.stdout_capture()`   | `Command<Capture, StderrTarget>`          |
| `.stdout_stream()`    | `Command<Stream, StderrTarget>`           |
| `.stderr_to_stdout()` | `Command<StdoutTarget, Parent<Stdout>>`   |
| `.stderr_to_stderr()` | `Command<StdoutTarget, Parent<Stderr>>`   |
| `.stderr_null()`      | `Command<StdoutTarget, Null>`             |
| `.stderr_capture()`   | `Command<StdoutTarget, Capture>`          |
| `.stderr_stream()`    | `Command<StdoutTarget, Stream>`           |

### Terminal methods

| Type                                        | Method      | Return                                                                                       |
|---------------------------------------------|-------------|----------------------------------------------------------------------------------------------|
| `Command<StdoutTarget, StderrTarget>`       | `.run()`    | `Result<Output<StdoutTarget::Captured, StderrTarget::Captured>, CommandError>` (no `Stream`) |
| `Command<StdoutTarget, StderrTarget>`       | `.spawn()`  | `Result<Child<StdoutTarget, StderrTarget>, CommandError>`                                    |
| `Command<StdoutTarget, StderrTarget>`       | `.build()`  | `tokio::process::Command`                                                                    |
| `Command<Capture, _>` (stderr not captured) | `.bytes()`  | `Result<Vec<u8>, CommandError>` (stdout)                                                     |
| `Command<Capture, _>` (stderr not captured) | `.string()` | `Result<String, CommandError>` (stdout)                                                      |
| `Command<_, Capture>` (stdout not captured) | `.bytes()`  | `Result<Vec<u8>, CommandError>` (stderr)                                                     |
| `Command<_, Capture>` (stdout not captured) | `.string()` | `Result<String, CommandError>` (stderr)                                                      |

### Result type

`Output<StdoutCaptured, StderrCaptured>` has the fields `stdout`, `stderr` and `status: ExitStatus`. For example, `.stdout_capture().run()` returns `Output<Vec<u8>, ()>`, and `.run()` without any capture returns `Output<(), ()>`.

Every `Command` supports `.accept_nonzero_exit()` to allow non-zero exit codes without error.

## Streaming

Set a stream to `Stream` and `.spawn()` the command to read it while the process runs. Other targets combine freely: a `Capture` stream is read in the background meanwhile, so neither pipe can fill up and block the other.

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cmd_proc::Command;
    use tokio::io::{AsyncBufReadExt, AsyncReadExt};

    // Read stdout line by line, capture stderr meanwhile
    let mut child = Command::new("sh")
        .arguments(["-c", "echo one; echo two; echo warning >&2"])
        .stdout_stream()
        .stderr_capture()
        .spawn()?;

    let (stdout, &mut ()) = child.handles();
    let mut lines = tokio::io::BufReader::new(stdout).lines();
    let mut seen = Vec::new();
    while let Some(line) = lines.next_line().await? {
        seen.push(line);
    }
    assert_eq!(seen, ["one", "two"]);

    let output = child.wait().await?;
    assert_eq!(output.stderr, b"warning\n");

    // Hand a read end off: wait() no longer owns it
    let child = Command::new("echo")
        .argument("hello")
        .stdout_stream()
        .spawn()?;
    let (mut stdout, child) = child.take_stdout();
    child.wait().await?;
    let mut bytes = Vec::new();
    stdout.read_to_end(&mut bytes).await?;
    assert_eq!(bytes, b"hello\n");
    Ok(())
}
```

### Child methods

| Type                                | Method            | Return                                                                                     |
|-------------------------------------|-------------------|--------------------------------------------------------------------------------------------|
| `Child<StdoutTarget, StderrTarget>` | `.handles()`      | `(&mut StdoutTarget::Handle<Stdout>, &mut StderrTarget::Handle<Stderr>)`                   |
| `Child<StdoutTarget, StderrTarget>` | `.wait()`         | `Result<Output<StdoutTarget::Captured, StderrTarget::Captured>, CommandError>`             |
| `Child<Stream, StderrTarget>`       | `.take_stdout()`  | `(tokio::process::ChildStdout, Child<Transferred, StderrTarget>)`                          |
| `Child<StdoutTarget, Stream>`       | `.take_stderr()`  | `(tokio::process::ChildStderr, Child<StdoutTarget, Transferred>)`                          |

A handle is `ChildStdout` / `ChildStderr` for a `Stream` stream and `()` for every other target. `.wait()` closes the read ends the `Child` still holds before waiting for the process, so a child still writing to an unread stream gets `EPIPE` instead of blocking forever. After `.take_stdout()` / `.take_stderr()`, closing that read end is up to whoever holds it.

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
        .run().await?;

    // Remove a variable from the child environment
    const PATH: EnvVariableName = EnvVariableName::from_static_or_panic("PATH");
    Command::new("sh")
        .arguments(["-c", "true"])
        .env_remove(&PATH)
        .run().await?;
    Ok(())
}
```

## Error Handling

`CommandError` is an enum with two variants: `Io` for spawn/IO failures
(command not found, permission denied, ...) and `ExitStatus` for non-zero exits.

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use cmd_proc::{Command, CommandError};

    // IO error — command not found
    let error = Command::new("./nonexistent").run().await.unwrap_err();
    assert!(matches!(error, CommandError::Io(_)));

    // Non-zero exit status
    let error = Command::new("false").run().await.unwrap_err();
    assert!(matches!(error, CommandError::ExitStatus(_)));
    Ok(())
}
```
