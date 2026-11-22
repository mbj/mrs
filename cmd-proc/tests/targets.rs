// Targets writing to the parent process can only be observed from outside it:
// each case re-executes this binary and captures what the inner process wrote
// to its own stdout and stderr. Runs without the libtest harness, which would
// write to stdout itself.

use cmd_proc::Command;

const CHILD: &str = "child";

struct Case {
    name: &'static str,
    stdout: &'static str,
    stderr: &'static str,
}

const CASES: &[Case] = &[
    Case {
        name: "run:stdout_to_stdout,stderr_to_stdout",
        stdout: "outerr",
        stderr: "",
    },
    Case {
        name: "run:stdout_to_stdout,stderr_to_stderr",
        stdout: "out",
        stderr: "err",
    },
    Case {
        name: "run:stdout_to_stdout,stderr_null",
        stdout: "out",
        stderr: "",
    },
    Case {
        name: "run:stdout_to_stdout,stderr_capture",
        stdout: "out",
        stderr: "",
    },
    Case {
        name: "run:stdout_to_stderr,stderr_to_stdout",
        stdout: "err",
        stderr: "out",
    },
    Case {
        name: "run:stdout_to_stderr,stderr_to_stderr",
        stdout: "",
        stderr: "outerr",
    },
    Case {
        name: "run:stdout_to_stderr,stderr_null",
        stdout: "",
        stderr: "out",
    },
    Case {
        name: "run:stdout_to_stderr,stderr_capture",
        stdout: "",
        stderr: "out",
    },
    Case {
        name: "run:stdout_null,stderr_to_stdout",
        stdout: "err",
        stderr: "",
    },
    Case {
        name: "run:stdout_null,stderr_to_stderr",
        stdout: "",
        stderr: "err",
    },
    Case {
        name: "run:stdout_null,stderr_null",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "run:stdout_null,stderr_capture",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "run:stdout_capture,stderr_to_stdout",
        stdout: "err",
        stderr: "",
    },
    Case {
        name: "run:stdout_capture,stderr_to_stderr",
        stdout: "",
        stderr: "err",
    },
    Case {
        name: "run:stdout_capture,stderr_null",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "run:stdout_capture,stderr_capture",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_to_stdout,stderr_to_stdout",
        stdout: "outerr",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_to_stdout,stderr_to_stderr",
        stdout: "out",
        stderr: "err",
    },
    Case {
        name: "spawn:stdout_to_stdout,stderr_null",
        stdout: "out",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_to_stdout,stderr_capture",
        stdout: "out",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_to_stdout,stderr_stream",
        stdout: "out",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_to_stderr,stderr_to_stdout",
        stdout: "err",
        stderr: "out",
    },
    Case {
        name: "spawn:stdout_to_stderr,stderr_to_stderr",
        stdout: "",
        stderr: "outerr",
    },
    Case {
        name: "spawn:stdout_to_stderr,stderr_null",
        stdout: "",
        stderr: "out",
    },
    Case {
        name: "spawn:stdout_to_stderr,stderr_capture",
        stdout: "",
        stderr: "out",
    },
    Case {
        name: "spawn:stdout_to_stderr,stderr_stream",
        stdout: "",
        stderr: "out",
    },
    Case {
        name: "spawn:stdout_null,stderr_to_stdout",
        stdout: "err",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_null,stderr_to_stderr",
        stdout: "",
        stderr: "err",
    },
    Case {
        name: "spawn:stdout_null,stderr_null",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_null,stderr_capture",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_null,stderr_stream",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_capture,stderr_to_stdout",
        stdout: "err",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_capture,stderr_to_stderr",
        stdout: "",
        stderr: "err",
    },
    Case {
        name: "spawn:stdout_capture,stderr_null",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_capture,stderr_capture",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_capture,stderr_stream",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_stream,stderr_to_stdout",
        stdout: "err",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_stream,stderr_to_stderr",
        stdout: "",
        stderr: "err",
    },
    Case {
        name: "spawn:stdout_stream,stderr_null",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_stream,stderr_capture",
        stdout: "",
        stderr: "",
    },
    Case {
        name: "spawn:stdout_stream,stderr_stream",
        stdout: "",
        stderr: "",
    },
];

fn command() -> Command {
    Command::new("sh").arguments(["-c", "printf out; printf err >&2"])
}

async fn read_all(reader: &mut (impl tokio::io::AsyncRead + Unpin)) -> Vec<u8> {
    use tokio::io::AsyncReadExt;

    let mut bytes = Vec::new();
    reader.read_to_end(&mut bytes).await.unwrap();
    bytes
}

async fn run_child(name: &str) {
    match name {
        "run:stdout_to_stdout,stderr_to_stdout" => {
            let output = command()
                .stdout_to_stdout()
                .stderr_to_stdout()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "run:stdout_to_stdout,stderr_to_stderr" => {
            let output = command()
                .stdout_to_stdout()
                .stderr_to_stderr()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "run:stdout_to_stdout,stderr_null" => {
            let output = command()
                .stdout_to_stdout()
                .stderr_null()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "run:stdout_to_stdout,stderr_capture" => {
            let output = command()
                .stdout_to_stdout()
                .stderr_capture()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            assert_eq!(output.stderr, b"err");
        }
        "run:stdout_to_stderr,stderr_to_stdout" => {
            let output = command()
                .stdout_to_stderr()
                .stderr_to_stdout()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "run:stdout_to_stderr,stderr_to_stderr" => {
            let output = command()
                .stdout_to_stderr()
                .stderr_to_stderr()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "run:stdout_to_stderr,stderr_null" => {
            let output = command()
                .stdout_to_stderr()
                .stderr_null()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "run:stdout_to_stderr,stderr_capture" => {
            let output = command()
                .stdout_to_stderr()
                .stderr_capture()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            assert_eq!(output.stderr, b"err");
        }
        "run:stdout_null,stderr_to_stdout" => {
            let output = command()
                .stdout_null()
                .stderr_to_stdout()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "run:stdout_null,stderr_to_stderr" => {
            let output = command()
                .stdout_null()
                .stderr_to_stderr()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "run:stdout_null,stderr_null" => {
            let output = command().stdout_null().stderr_null().run().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "run:stdout_null,stderr_capture" => {
            let output = command()
                .stdout_null()
                .stderr_capture()
                .run()
                .await
                .unwrap();
            let () = output.stdout;
            assert_eq!(output.stderr, b"err");
        }
        "run:stdout_capture,stderr_to_stdout" => {
            let output = command()
                .stdout_capture()
                .stderr_to_stdout()
                .run()
                .await
                .unwrap();
            assert_eq!(output.stdout, b"out");
            let () = output.stderr;
        }
        "run:stdout_capture,stderr_to_stderr" => {
            let output = command()
                .stdout_capture()
                .stderr_to_stderr()
                .run()
                .await
                .unwrap();
            assert_eq!(output.stdout, b"out");
            let () = output.stderr;
        }
        "run:stdout_capture,stderr_null" => {
            let output = command()
                .stdout_capture()
                .stderr_null()
                .run()
                .await
                .unwrap();
            assert_eq!(output.stdout, b"out");
            let () = output.stderr;
        }
        "run:stdout_capture,stderr_capture" => {
            let output = command()
                .stdout_capture()
                .stderr_capture()
                .run()
                .await
                .unwrap();
            assert_eq!(output.stdout, b"out");
            assert_eq!(output.stderr, b"err");
        }
        "spawn:stdout_to_stdout,stderr_to_stdout" => {
            let child = command()
                .stdout_to_stdout()
                .stderr_to_stdout()
                .spawn()
                .unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_to_stdout,stderr_to_stderr" => {
            let child = command()
                .stdout_to_stdout()
                .stderr_to_stderr()
                .spawn()
                .unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_to_stdout,stderr_null" => {
            let child = command().stdout_to_stdout().stderr_null().spawn().unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_to_stdout,stderr_capture" => {
            let child = command()
                .stdout_to_stdout()
                .stderr_capture()
                .spawn()
                .unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            assert_eq!(output.stderr, b"err");
        }
        "spawn:stdout_to_stdout,stderr_stream" => {
            let mut child = command()
                .stdout_to_stdout()
                .stderr_stream()
                .spawn()
                .unwrap();
            let (&mut (), stderr) = child.handles();
            assert_eq!(read_all(stderr).await, b"err");
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_to_stderr,stderr_to_stdout" => {
            let child = command()
                .stdout_to_stderr()
                .stderr_to_stdout()
                .spawn()
                .unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_to_stderr,stderr_to_stderr" => {
            let child = command()
                .stdout_to_stderr()
                .stderr_to_stderr()
                .spawn()
                .unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_to_stderr,stderr_null" => {
            let child = command().stdout_to_stderr().stderr_null().spawn().unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_to_stderr,stderr_capture" => {
            let child = command()
                .stdout_to_stderr()
                .stderr_capture()
                .spawn()
                .unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            assert_eq!(output.stderr, b"err");
        }
        "spawn:stdout_to_stderr,stderr_stream" => {
            let mut child = command()
                .stdout_to_stderr()
                .stderr_stream()
                .spawn()
                .unwrap();
            let (&mut (), stderr) = child.handles();
            assert_eq!(read_all(stderr).await, b"err");
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_null,stderr_to_stdout" => {
            let child = command().stdout_null().stderr_to_stdout().spawn().unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_null,stderr_to_stderr" => {
            let child = command().stdout_null().stderr_to_stderr().spawn().unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_null,stderr_null" => {
            let child = command().stdout_null().stderr_null().spawn().unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_null,stderr_capture" => {
            let child = command().stdout_null().stderr_capture().spawn().unwrap();
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            assert_eq!(output.stderr, b"err");
        }
        "spawn:stdout_null,stderr_stream" => {
            let mut child = command().stdout_null().stderr_stream().spawn().unwrap();
            let (&mut (), stderr) = child.handles();
            assert_eq!(read_all(stderr).await, b"err");
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_capture,stderr_to_stdout" => {
            let child = command()
                .stdout_capture()
                .stderr_to_stdout()
                .spawn()
                .unwrap();
            let output = child.wait().await.unwrap();
            assert_eq!(output.stdout, b"out");
            let () = output.stderr;
        }
        "spawn:stdout_capture,stderr_to_stderr" => {
            let child = command()
                .stdout_capture()
                .stderr_to_stderr()
                .spawn()
                .unwrap();
            let output = child.wait().await.unwrap();
            assert_eq!(output.stdout, b"out");
            let () = output.stderr;
        }
        "spawn:stdout_capture,stderr_null" => {
            let child = command().stdout_capture().stderr_null().spawn().unwrap();
            let output = child.wait().await.unwrap();
            assert_eq!(output.stdout, b"out");
            let () = output.stderr;
        }
        "spawn:stdout_capture,stderr_capture" => {
            let child = command().stdout_capture().stderr_capture().spawn().unwrap();
            let output = child.wait().await.unwrap();
            assert_eq!(output.stdout, b"out");
            assert_eq!(output.stderr, b"err");
        }
        "spawn:stdout_capture,stderr_stream" => {
            let mut child = command().stdout_capture().stderr_stream().spawn().unwrap();
            let (&mut (), stderr) = child.handles();
            assert_eq!(read_all(stderr).await, b"err");
            let output = child.wait().await.unwrap();
            assert_eq!(output.stdout, b"out");
            let () = output.stderr;
        }
        "spawn:stdout_stream,stderr_to_stdout" => {
            let mut child = command()
                .stdout_stream()
                .stderr_to_stdout()
                .spawn()
                .unwrap();
            let (stdout, &mut ()) = child.handles();
            assert_eq!(read_all(stdout).await, b"out");
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_stream,stderr_to_stderr" => {
            let mut child = command()
                .stdout_stream()
                .stderr_to_stderr()
                .spawn()
                .unwrap();
            let (stdout, &mut ()) = child.handles();
            assert_eq!(read_all(stdout).await, b"out");
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_stream,stderr_null" => {
            let mut child = command().stdout_stream().stderr_null().spawn().unwrap();
            let (stdout, &mut ()) = child.handles();
            assert_eq!(read_all(stdout).await, b"out");
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        "spawn:stdout_stream,stderr_capture" => {
            let mut child = command().stdout_stream().stderr_capture().spawn().unwrap();
            let (stdout, &mut ()) = child.handles();
            assert_eq!(read_all(stdout).await, b"out");
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            assert_eq!(output.stderr, b"err");
        }
        "spawn:stdout_stream,stderr_stream" => {
            let mut child = command().stdout_stream().stderr_stream().spawn().unwrap();
            let (stdout, stderr) = child.handles();
            let (stdout, stderr) = tokio::join!(read_all(stdout), read_all(stderr));
            assert_eq!(stdout, b"out");
            assert_eq!(stderr, b"err");
            let output = child.wait().await.unwrap();
            let () = output.stdout;
            let () = output.stderr;
        }
        _ => panic!("unknown case: {name}"),
    }
}

async fn run_cases() {
    let executable = std::env::current_exe().unwrap();

    for case in CASES {
        let output = Command::new(&executable)
            .argument(CHILD)
            .argument(case.name)
            .stdout_capture()
            .stderr_capture()
            .accept_nonzero_exit()
            .run()
            .await
            .unwrap();

        assert_eq!(
            (
                case.name,
                output.status.success(),
                std::str::from_utf8(&output.stdout).unwrap(),
                std::str::from_utf8(&output.stderr).unwrap(),
            ),
            (case.name, true, case.stdout, case.stderr),
        );
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let arguments: Vec<String> = std::env::args().skip(1).collect();

    match arguments.as_slice() {
        [role, name] if role == CHILD => run_child(name).await,
        _ => run_cases().await,
    }
}
