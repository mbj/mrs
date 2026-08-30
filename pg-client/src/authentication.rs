//! Cleartext and MD5 prove the client to the server and tell the client
//! nothing in return, which is the property SCRAM's server signature exists to
//! supply; GSSAPI would mean linking a Kerberos library.
//!
//! See: <https://datatracker.ietf.org/doc/html/rfc7677> for SCRAM-SHA-256 and
//! <https://datatracker.ietf.org/doc/html/rfc5802> for SCRAM itself.

use crate::name::User;

pub mod scram;

/// `pg_hba.conf` matches on the user name, and the stored verifier is per user.
#[derive(Clone, Debug)]
pub struct Config {
    pub user: User<'static>,
    pub methods: Methods,
}

/// The server chooses the method, from `pg_hba.conf`; a client cannot ask for
/// one. These are the requests this client answers, and any other ends the
/// startup with [`crate::session::StartError::MethodNotConfigured`].
#[derive(Clone, Debug, Default)]
pub struct Methods {
    /// Accept a server that sends `AuthenticationOk` without requesting any
    /// credential. On the wire the `pg_hba.conf` methods `trust`, `peer`,
    /// `ident` and `cert` all look like this, so accepting one accepts them all.
    pub accept_unchallenged: bool,
    pub scram: Option<method::Scram>,
    pub cleartext_password: Option<method::CleartextPassword>,
}

impl Methods {
    #[must_use]
    pub fn with_unchallenged(self) -> Self {
        Self {
            accept_unchallenged: true,
            ..self
        }
    }

    #[must_use]
    pub fn with_scram(self, scram: method::Scram) -> Self {
        Self {
            scram: Some(scram),
            ..self
        }
    }

    #[must_use]
    pub fn with_cleartext_password(self, cleartext_password: method::CleartextPassword) -> Self {
        Self {
            cleartext_password: Some(cleartext_password),
            ..self
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SharedPasswordError {
    #[error("a password is dynamic, resolve it with with_static_passwords first")]
    Dynamic,
    #[error("the methods carry different passwords")]
    Conflicting,
}

impl Methods {
    pub fn shared_static_password(
        &self,
    ) -> Result<Option<&crate::config::Password>, SharedPasswordError> {
        let mut shared = None;

        for password in [
            self.scram.as_ref().map(|scram| &scram.password),
            self.cleartext_password
                .as_ref()
                .map(|cleartext| &cleartext.password),
        ]
        .into_iter()
        .flatten()
        {
            let Password::Static(password) = password else {
                return Err(SharedPasswordError::Dynamic);
            };

            match shared {
                Some(previous) if previous != password => {
                    return Err(SharedPasswordError::Conflicting);
                }
                _ => shared = Some(password),
            }
        }

        Ok(shared)
    }
}

pub mod method {
    #[derive(Clone, Debug)]
    pub struct Scram {
        pub password: super::Password,
        pub channel_binding: crate::config::ChannelBinding,
        pub iteration_bounds: super::scram::IterationBounds,
    }

    impl Scram {
        #[must_use]
        pub fn new(
            password: super::Password,
            channel_binding: crate::config::ChannelBinding,
        ) -> Self {
            Self {
                password,
                channel_binding,
                iteration_bounds: super::scram::IterationBounds::DEFAULT,
            }
        }

        #[must_use]
        pub fn with_iteration_bounds(
            self,
            iteration_bounds: super::scram::IterationBounds,
        ) -> Self {
            Self {
                iteration_bounds,
                ..self
            }
        }
    }

    /// What `pg_hba.conf` methods `password`, `ldap`, `pam`, `radius` and
    /// `bsd` request, and how IAM tokens for RDS and Cloud SQL are sent.
    ///
    /// The password crosses the connection as is, so it is only sent over TLS
    /// or a Unix socket, never over unencrypted TCP.
    #[derive(Clone, Debug)]
    pub struct CleartextPassword {
        pub password: super::Password,
    }
}

#[derive(Clone)]
pub enum Password {
    Static(crate::config::Password),
    Dynamic(std::sync::Arc<dyn PasswordSource>),
}

impl Password {
    pub async fn resolve(&self) -> Result<crate::config::Password, PasswordError> {
        match self {
            Self::Static(password) => Ok(password.clone()),
            Self::Dynamic(source) => source.password().await,
        }
    }
}

impl core::fmt::Debug for Password {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Static(_) => formatter.write_str("Static(..)"),
            Self::Dynamic(_) => formatter.write_str("Dynamic(..)"),
        }
    }
}

pub type PasswordError = Box<dyn std::error::Error + Send + Sync>;

pub trait PasswordSource: Send + Sync {
    fn password(
        &self,
    ) -> core::pin::Pin<
        Box<dyn Future<Output = Result<crate::config::Password, PasswordError>> + Send + '_>,
    >;
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    struct Unasked;

    impl PasswordSource for Unasked {
        fn password(
            &self,
        ) -> core::pin::Pin<
            Box<dyn Future<Output = Result<crate::config::Password, PasswordError>> + Send + '_>,
        > {
            Box::pin(core::future::pending())
        }
    }

    fn fixed(password: &str) -> Password {
        Password::Static(password.parse().unwrap())
    }

    fn dynamic() -> Password {
        Password::Dynamic(std::sync::Arc::new(Unasked))
    }

    fn methods(scram: Option<Password>, cleartext: Option<Password>) -> Methods {
        Methods {
            scram: scram.map(|password| {
                method::Scram::new(password, crate::config::ChannelBinding::Prefer)
            }),
            cleartext_password: cleartext.map(|password| method::CleartextPassword { password }),
            ..Methods::default()
        }
    }

    #[test]
    fn shares_the_one_static_password_the_methods_carry() {
        let a: crate::config::Password = "a".parse().unwrap();

        for (scram, cleartext, expected) in [
            (None, None, Ok(None)),
            (Some(fixed("a")), None, Ok(Some(&a))),
            (None, Some(fixed("a")), Ok(Some(&a))),
            (Some(fixed("a")), Some(fixed("a")), Ok(Some(&a))),
            (
                Some(fixed("a")),
                Some(fixed("b")),
                Err(SharedPasswordError::Conflicting),
            ),
            (Some(dynamic()), None, Err(SharedPasswordError::Dynamic)),
            (
                Some(fixed("a")),
                Some(dynamic()),
                Err(SharedPasswordError::Dynamic),
            ),
        ] {
            let methods = methods(scram, cleartext);

            assert_eq!(methods.shared_static_password(), expected);
        }
    }
}
