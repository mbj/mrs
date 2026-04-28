#![doc = include_str!("../README.md")]

pub mod backend;
pub mod config;
pub mod container;
pub mod hasher;
pub mod image;
pub mod label;
pub mod platform;
pub mod reference;
pub mod testing;

pub use backend::{Backend, BridgeSubnetError, ContainerHostnameResolver, ResolveHostnameError};
pub use container::{
    Container, ContainerArgument, ContainerId, Definition, Detach, EnvironmentVariables, Entrypoint,
    ExecCommand, InspectError, Mount, Protocol, Publish, ReadHostTcpPortError, Remove, Workdir,
};
pub use image::{
    BuildArgumentKey, BuildArgumentKeyError, BuildArgumentValue, BuildDefinition, BuildSource,
    BuildTarget, Reference,
};

use cmd_proc::Command;

pub(crate) trait Apply {
    fn apply(&self, command: Command) -> Command;
}

impl<T: Apply> Apply for Vec<T> {
    fn apply(&self, command: Command) -> Command {
        self.iter()
            .fold(command, |command, item| item.apply(command))
    }
}

impl<T: Apply> Apply for Option<T> {
    fn apply(&self, command: Command) -> Command {
        match self {
            Some(item) => item.apply(command),
            None => command,
        }
    }
}

impl Apply for image::Reference {
    fn apply(&self, command: Command) -> Command {
        command.argument(self.to_string())
    }
}
