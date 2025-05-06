use std::ffi::OsStr;

struct Command {
    inner: std::process::Command,
}

impl Command {
    fn new(value: impl AsRef<OsStr>) -> Self {
        Command {
            inner: std::process::Command::new(value),
        }
    }

    fn argument(mut self, value: impl AsRef<OsStr>) -> Self {
        self.inner.arg(value);

        self
    }

    fn optional_argument(mut self, optional: Option<impl AsRef<OsStr>>) -> Self {
        match optional {
            Some(value) => {
                self.inner.arg(value);
                self
            }
            None => self,
        }
    }

    fn arguments<T: AsRef<OsStr>>(mut self, value: impl IntoIterator<Item = T>) -> Self {
        self.inner.args(value);

        self
    }

    fn capture_only_stdout(mut self) -> Vec<u8> {
        eprintln!("{:#?}", self.inner);

        // Command::output sadly also captures stderr which we do not want in this case.
        match self.inner.stdout(std::process::Stdio::piped()).spawn() {
            Ok(mut child) => {
                let mut buf = vec![];

                let mut stdout = child.stdout.as_mut().unwrap();

                let _ = std::io::Read::read_to_end(&mut stdout, &mut buf);

                let status = child.wait().unwrap();

                assert!(status.success());

                buf
            }
            Err(error) => panic!("Failed to run container command: {error:#?}"),
        }
    }

    fn status(mut self) -> std::process::ExitStatus {
        eprintln!("{:#?}", self.inner);

        match self.inner.status() {
            Ok(status) => status,
            Err(error) => panic!("Failed to run container command: {error:#?}"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Detach {
    Detach,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Remove {
    Remove,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Mount(String);

impl From<String> for Mount {
    fn from(value: String) -> Self {
        Self(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Image(String);

impl AsRef<std::ffi::OsStr> for Image {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

impl From<String> for Image {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Image {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Publish(pub String);

impl AsRef<std::ffi::OsStr> for Publish {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

impl From<&str> for Publish {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Entrypoint {
    command: String,
    arguments: Vec<String>,
}

impl Entrypoint {
    fn arguments(&self) -> Vec<String> {
        vec![
            "--entrypoint".to_string(),
            if self.arguments.is_empty() {
                self.command.to_string()
            } else {
                let mut arguments: Vec<String> = vec![];

                arguments.push(self.command.clone());
                arguments.append(&mut self.arguments.clone());

                serde_json::to_string(&arguments).unwrap()
            },
        ]
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Definition {
    detach: Option<Detach>,
    entrypoint: Option<Entrypoint>,
    env: std::collections::BTreeMap<String, String>,
    image: Image,
    remove: Option<Remove>,
    mounts: Vec<Mount>,
    publish: Vec<Publish>,
}

impl Definition {
    pub fn new(image: Image) -> Definition {
        Definition {
            detach: None,
            entrypoint: None,
            env: std::collections::BTreeMap::new(),
            image,
            mounts: vec![],
            publish: vec![],
            remove: None,
        }
    }

    pub fn entrypoint(self, command: String, arguments: Vec<String>) -> Self {
        Self {
            entrypoint: Some(Entrypoint { command, arguments }),
            ..self
        }
    }

    pub fn env(self, key: &str, value: &str) -> Self {
        let mut env = self.env.clone();

        env.insert(key.to_string(), value.to_string());

        Self { env, ..self }
    }

    pub fn envs<K: ToString, V: ToString>(self, values: impl IntoIterator<Item = (K, V)>) -> Self {
        let mut env = self.env.clone();

        for (key, value) in values {
            env.insert(key.to_string(), value.to_string());
        }

        Self { env, ..self }
    }

    pub fn remove(self) -> Self {
        Self {
            remove: Some(Remove::Remove),
            ..self
        }
    }

    pub fn no_remove(self) -> Self {
        Self {
            remove: None,
            ..self
        }
    }

    pub fn detach(self) -> Self {
        Self {
            detach: Some(Detach::Detach),
            ..self
        }
    }

    pub fn no_detach(self) -> Self {
        Self {
            detach: None,
            ..self
        }
    }

    pub fn publish(self, value: Publish) -> Self {
        let mut publish = self.publish.clone();

        publish.push(value);

        Self { publish, ..self }
    }

    pub fn mount(self, value: Mount) -> Self {
        let mut mounts = self.mounts.clone();

        mounts.push(value);

        Self { mounts, ..self }
    }

    pub fn mounts(self, values: impl IntoIterator<Item = Mount>) -> Self {
        let mut mounts = self.mounts.clone();

        for value in values.into_iter() {
            mounts.push(value);
        }

        Self { mounts, ..self }
    }

    pub fn run_detached(self) -> Container {
        let stdout = self.detach().run_output();

        Container {
            id: ContainerId::try_from(strip_nl_end(&stdout)).unwrap(),
            stopped: false,
        }
    }

    pub fn run_capture_only_stdout(self) -> Vec<u8> {
        self.no_detach().run_output()
    }

    fn run_output(&self) -> Vec<u8> {
        Command::new("podman")
            .argument("run")
            .optional_argument(self.detach.as_ref().map(|_| "--detach"))
            .optional_argument(self.remove.as_ref().map(|_| "--rm"))
            .arguments(
                self.env
                    .iter()
                    .flat_map(|(key, value)| ["--env".to_string(), format!("{key}={value}")]),
            )
            .arguments(
                self.publish
                    .iter()
                    .flat_map(|publish| ["--publish".to_string(), publish.0.clone()]),
            )
            .arguments(
                self.mounts
                    .iter()
                    .flat_map(|mount| ["--mount".to_string(), mount.0.clone()]),
            )
            .arguments(
                self.entrypoint
                    .as_ref()
                    .map(|entrypoint| entrypoint.arguments())
                    .unwrap_or_default(),
            )
            .argument(&self.image)
            .capture_only_stdout()
    }
}

fn strip_nl_end(value: &[u8]) -> &[u8] {
    match value.split_last() {
        Some((last, prefix)) => {
            if *last == b'\n' {
                prefix
            } else {
                panic!("last byte not a newline")
            }
        }
        None => panic!("empty slice"),
    }
}

#[derive(Debug)]
pub struct ContainerId(String);

impl std::convert::TryFrom<&[u8]> for ContainerId {
    type Error = std::str::Utf8Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        std::str::from_utf8(value).map(|str| ContainerId(str.to_string()))
    }
}

impl AsRef<std::ffi::OsStr> for ContainerId {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

#[derive(Debug)]
pub struct Container {
    id: ContainerId,
    stopped: bool,
}

impl Container {
    pub fn stop(&mut self) {
        let status = Command::new("podman")
            .arguments(["container", "stop"])
            .argument(&self.id)
            .status();

        assert!(status.success());

        self.stopped = true;
    }

    pub fn exec_capture_only_stdout<T: AsRef<OsStr>>(
        &self,
        environment: impl IntoIterator<Item = (&'static str, String)>,
        executable: T,
        arguments: impl IntoIterator<Item = T>,
    ) -> Vec<u8> {
        Command::new("podman")
            .argument("exec")
            .arguments(
                environment
                    .into_iter()
                    .flat_map(|(key, value)| ["--env".to_string(), format!("{key}={value}")]),
            )
            .argument(&self.id)
            .argument(executable)
            .arguments(arguments)
            .capture_only_stdout()
    }

    pub fn exec_interactive<T: AsRef<OsStr>>(
        &self,
        environment: impl IntoIterator<Item = (&'static str, String)>,
        executable: T,
        arguments: impl IntoIterator<Item = T>,
    ) {
        let _status = Command::new("podman")
            .argument("exec")
            .argument("--tty")
            .argument("--interactive")
            .arguments(
                environment
                    .into_iter()
                    .flat_map(|(key, value)| ["--env".to_string(), format!("{key}={value}")]),
            )
            .argument(&self.id)
            .argument(executable)
            .arguments(arguments)
            .status();
    }

    pub fn inspect(&self) -> serde_json::Value {
        let stdout = Command::new("podman")
            .argument("inspect")
            .argument(&self.id)
            .capture_only_stdout();

        serde_json::from_slice(&stdout).expect("invalid json")
    }

    pub fn inspect_format(&self, format: &str) -> String {
        let bytes = Command::new("podman")
            .argument("inspect")
            .argument("--format")
            .argument(format)
            .argument(&self.id)
            .capture_only_stdout();

        std::str::from_utf8(strip_nl_end(&bytes))
            .expect("invalid utf8")
            .to_string()
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        if !self.stopped {
            self.stop()
        }
    }
}
