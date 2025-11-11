use std::ffi::OsStr;

pub struct Command {
    inner: std::process::Command,
}

impl Command {
    pub fn new(value: impl AsRef<OsStr>) -> Self {
        Command {
            inner: std::process::Command::new(value),
        }
    }

    pub fn argument(mut self, value: impl AsRef<OsStr>) -> Self {
        self.inner.arg(value);

        self
    }

    pub fn optional_argument(mut self, optional: Option<impl AsRef<OsStr>>) -> Self {
        match optional {
            Some(value) => {
                self.inner.arg(value);
                self
            }
            None => self,
        }
    }

    pub fn arguments<T: AsRef<OsStr>>(mut self, value: impl IntoIterator<Item = T>) -> Self {
        self.inner.args(value);

        self
    }

    pub fn capture_only_stdout_result(mut self) -> Result<Vec<u8>, std::io::Error> {
        log::debug!("{:#?}", self.inner);

        // Command::output sadly also captures stderr which we do not want in this case.
        match self.inner.stdout(std::process::Stdio::piped()).spawn() {
            Ok(mut child) => {
                let mut buf = vec![];

                let mut stdout = child.stdout.as_mut().unwrap();

                let _ = std::io::Read::read_to_end(&mut stdout, &mut buf);

                let status = child.wait().unwrap();

                if !status.success() {
                    panic!("Command exited nonzero unexpected: {status:#?}")
                }

                Ok(buf)
            }
            Err(error) => Err(error),
        }
    }

    pub fn capture_only_stdout(self) -> Vec<u8> {
        self.capture_only_stdout_result().unwrap()
    }

    pub fn capture_only_stdout_string(self) -> String {
        std::str::from_utf8(&self.capture_only_stdout())
            .unwrap()
            .to_string()
    }

    pub fn status(mut self) -> std::process::ExitStatus {
        log::debug!("{:#?}", self.inner);

        match self.inner.status() {
            Ok(status) => status,
            Err(error) => panic!("Failed to run container command: {error:#?}"),
        }
    }
}
