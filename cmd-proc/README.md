# cmd-proc - Process Command Builder

A wrapper around `tokio::process::Command` providing debug logging, stronger input types, and a fluent builder API with automatic exit code checking.

> **Status**: Pre-1.0 - exists to serve [mbj/mrs](https://github.com/mbj/mrs) monorepo, expect breaking changes without notice.

## Why cmd-proc?

`std::process::Command` is powerful but requires verbose boilerplate for common patterns:

```rust
// std::process - capturing stdout as string
let output = std::process::Command::new("git")
    .args(["rev-parse", "HEAD"])
    .output()?;
if !output.status.success() {
    return Err(/* construct error with exit status */);
}
let stdout = String::from_utf8(output.stdout)?;

// cmd-proc - same operation
let stdout = cmd_proc::Command::new("git")
    .arguments(["rev-parse", "HEAD"])
    .capture_stdout()
    .string().await?;
```

### Key differences from `std::process`

| Feature | `std::process` | `cmd-proc` |
|---------|---------------|------------|
| **Debug logging** | None | Automatic debug logging of commands before execution |
| **Exit code checking** | Manual | Automatic - non-zero exits return `Err` |
| **Output capture** | Returns raw `Output` struct | Two-step pattern: `.capture_stdout().string()`, `.capture_stderr().bytes()` |
| **Builder pattern** | Mutable references | Owned builder with method chaining |
| **Stdin data** | Requires manual pipe setup | Simple `.stdin_bytes()` method |
| **Env var names** | Accepts any `&str` | `EnvVariableName` type with compile-time validation |
| **Error type** | Separate `io::Error` and `ExitStatus` | Unified `CommandError` with both |
| **Process spawning** | Direct `.spawn()` on Command | Separate `Spawn` builder with `Stdio` configuration |

### Design philosophy

- **Debug logging**: Every command execution is logged via the `log` crate at debug level, making it easy to trace what commands are being run.
- **Stronger input types**: `EnvVariableName` prevents invalid environment variable names (empty or containing `=`) at compile time rather than runtime.
- **Two-step capture pattern**: The `.capture_stdout()` and `.capture_stderr()` methods return a `Capture` builder, which provides `.bytes()` and `.string()` methods. This separates stream selection from output format.
- **Default exit code checking**: Capture methods treat non-zero exit as an error by default, preventing accidentally ignored failures. Use `.accept_nonzero_exit()` to opt out when needed.
- **Fluent API**: Chain configuration methods naturally without `&mut self` gymnastics.

## Usage

```rust
use cmd_proc::{Command, Capture, Output, EnvVariableName};

// Capture stdout as string (two-step pattern)
let sha = Command::new("git")
    .arguments(["rev-parse", "HEAD"])
    .capture_stdout()
    .string().await?;

// Capture stderr as bytes
let errors = Command::new("cargo")
    .argument("build")
    .capture_stderr()
    .bytes().await?;

// Run without capturing (just check success)
Command::new("cargo")
    .arguments(["fmt", "--check"])
    .status().await?;

// Pass stdin data
let output = Command::new("cat")
    .stdin_bytes(b"hello world")
    .capture_stdout()
    .string().await?;

// Set environment variables (compile-time validated)
const MY_VAR: EnvVariableName = EnvVariableName::from_static_or_panic("MY_VAR");
Command::new("sh")
    .arguments(["-c", "echo $MY_VAR"])
    .env(&MY_VAR, "value")
    .status().await?;

// Set working directory
Command::new("cargo")
    .argument("build")
    .working_directory("/path/to/project")
    .status().await?;
```

## Arguments vs Options

In CLI terminology:
- **Argument**: positional value (e.g., `git clone <url>`)
- **Option**: named parameter with value (e.g., `--message "text"`, `-o file`)

cmd-proc provides methods for both:

```rust
// Arguments (positional)
Command::new("git")
    .argument("clone")              // single argument
    .argument(url)                  // another argument
    .arguments(["--depth", "1"])    // multiple arguments
    .optional_argument(maybe_path)  // argument only if Some
    .status().await?;

// Options (name + value pairs)
Command::new("git")
    .argument("commit")
    .option("--message", "fix bug")           // required option
    .optional_option("--author", maybe_author) // option only if Some
    .status().await?;
```

The `option` and `optional_option` methods simplify the common pattern of adding a flag followed by its value:

```rust
// Instead of:
if let Some(author) = maybe_author {
    command = command.argument("--author").argument(author);
}

// Use:
command.optional_option("--author", maybe_author)
```

## The Capture Pattern

Output capture uses a two-step pattern via the `Capture` struct:

```rust
Command::new("git")
    .argument("status")
    .capture_stdout()  // Returns Capture (selects which stream)
    .string().await?;        // Executes and returns output in chosen format
```

The `Capture` struct is returned by `.capture_stdout()` or `.capture_stderr()` and provides:
- `.bytes()` - Execute and return output as `Vec<u8>`
- `.string()` - Execute and return output as `String` (with UTF-8 validation)
- `.accept_nonzero_exit()` - Allow non-zero exit codes without returning an error

For capturing both streams simultaneously, use `.capture_stderr_stdout()` which returns a `CaptureAllStreams` builder:

```rust
let output = Command::new("my-command")
    .capture_stderr_stdout()
    .accept_nonzero_exit()  // Optional: don't treat non-zero exit as error
    .output().await?;

println!("stdout: {:?}", output.stdout);
println!("stderr: {:?}", output.stderr);
```

This separation makes the API explicit about which stream(s) are being captured and in what format.

## Full Output Access

When you need both streams or want to handle failures with stderr access, use `.output()`:

```rust
let output = Command::new("git")
    .arguments(["show", "some-ref:path"])
    .output().await?;

if output.success() {
    let content = output.into_stdout_string()?;
    // use content
} else {
    let error_message = output.into_stderr_string()?;
    // handle error with stderr
}
```

The `Output` struct provides:
- `stdout: Vec<u8>` - Raw stdout bytes
- `stderr: Vec<u8>` - Raw stderr bytes
- `status: ExitStatus` - Exit status
- `.success()` - Check if command succeeded
- `.into_stdout_string()` - Convert stdout to String (strict UTF-8, consumes self)
- `.into_stderr_string()` - Convert stderr to String (strict UTF-8, consumes self)

Unlike `Capture`, `.output()` does not treat non-zero exit as an error - it only fails on IO errors.

## Spawning Long-Running Processes

For processes that need interactive stdin/stdout or run in the background, use `.spawn()`:

```rust
use cmd_proc::{Command, Stdio};
use tokio::io::AsyncBufReadExt;

let mut child = Command::new("my-server")
    .argument("--port=8080")
    .spawn()
    .stdin(Stdio::Piped)
    .stdout(Stdio::Piped)
    .stderr(Stdio::Inherit)
    .run()?;

// Read from stdout
let line = tokio::io::BufReader::new(child.stdout().unwrap())
    .lines()
    .next()
    .unwrap()?;

// Close stdin to signal shutdown
drop(child.take_stdin());

// Wait for exit
let status = child.wait().await?;
```

The `Stdio` enum controls stream handling:
- `Stdio::Piped` - Capture the stream for reading/writing
- `Stdio::Inherit` - Pass through to parent process (default)
- `Stdio::Null` - Redirect to /dev/null

The `Child` struct provides:
- `.stdin()`, `.stdout()`, `.stderr()` - Mutable references to handles
- `.take_stdin()`, `.take_stdout()`, `.take_stderr()` - Take ownership of handles
- `.wait()` - Wait for exit, returns `ExitStatus`
- `.wait_with_output()` - Wait and collect output as `Output`

## Environment Variables

Environment variable names are validated via `EnvVariableName`:

- **Cannot be empty** - an empty name is meaningless
- **Cannot contain `=`** - the OS uses `=` as separator between name and value; a name containing `=` corrupts the environment block

`std::process::Command::env()` silently accepts invalid names, causing mysterious runtime failures. `EnvVariableName` catches errors at compile time (when `from_static_or_panic` is used in a `const` context) or parse time (`FromStr`).

```rust
use cmd_proc::{Command, EnvVariableName};

// Compile-time validated (panics at compile time if invalid)
const MY_VAR: EnvVariableName = EnvVariableName::from_static_or_panic("MY_VAR");
Command::new("sh")
    .env(&MY_VAR, "value")
    .status().await?;

// Set multiple variables from an iterator
let vars = [
    (EnvVariableName::from_static_or_panic("FOO"), "1"),
    (EnvVariableName::from_static_or_panic("BAR"), "2"),
];
Command::new("sh")
    .envs(vars)
    .status().await?;

// Remove a variable from the child environment
const PATH: EnvVariableName = EnvVariableName::from_static_or_panic("PATH");
Command::new("sh")
    .env_remove(&PATH)
    .status().await?;
```

## Error Handling

`CommandError` unifies IO errors (command not found, permission denied) and non-zero exit status:

```rust
match Command::new("might-fail").status().await {
    Ok(()) => println!("Success"),
    Err(e) => {
        if let Some(io_error) = &e.io_error {
            eprintln!("IO error: {io_error}");
        }
        if let Some(exit_status) = &e.exit_status {
            eprintln!("Exit code: {:?}", exit_status.code());
        }
    }
}
```
