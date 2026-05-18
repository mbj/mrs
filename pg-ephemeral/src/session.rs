//! Named pg-ephemeral sessions — running containers tagged with the
//! [`crate::label::SESSION_KEY`] label.

use crate::label;

/// User-facing identifier for a named container, paired with the
/// runtime-level OCI container name it derives at construction time.
///
/// Naming a container is independent of its lifecycle — see
/// [`crate::Definition::session_name`]. The OCI name is prefixed with
/// `pg-ephemeral-session-` so a session named `foo` cannot collide with an
/// unrelated `foo` container the user happens to be running.
///
/// The user-facing name is validated against [`ociman::ContainerName`]'s
/// rules at parse time, and the derived OCI name is constructed and
/// validated alongside it — so downstream callers can use either value
/// without re-parsing or unwrapping.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name {
    name: String,
    container_name: ociman::ContainerName,
}

impl Name {
    /// Prefix applied to the user-facing name to derive the OCI container
    /// name.
    pub const OCI_PREFIX: &'static str = "pg-ephemeral-session-";

    /// The user-facing name, e.g. `foo`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.name
    }

    /// The runtime-level OCI container name, e.g. `pg-ephemeral-session-foo`.
    #[must_use]
    pub fn container_name(&self) -> &ociman::ContainerName {
        &self.container_name
    }
}

impl std::str::FromStr for Name {
    type Err = ociman::ContainerNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        // Validate the user-facing shape using ContainerName's rules — we
        // discard the parsed value and keep the raw String, but the parse
        // is the validation gate.
        let _: ociman::ContainerName = value.parse()?;
        let container_name: ociman::ContainerName =
            format!("{}{value}", Self::OCI_PREFIX).parse()?;
        Ok(Self {
            name: value.to_owned(),
            container_name,
        })
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.name)
    }
}

/// A running pg-ephemeral container marked as a named session.
///
/// Pairs the OCI [`ociman::Container`] handle with the decoded
/// [`Name`] taken from the container's `pg-ephemeral.session` label.
#[derive(Debug)]
pub struct Session {
    container: ociman::Container,
    name: Name,
}

impl Session {
    /// The user-facing session name.
    #[must_use]
    pub fn name(&self) -> &Name {
        &self.name
    }

    /// The underlying container handle.
    #[must_use]
    pub fn container(&self) -> &ociman::Container {
        &self.container
    }

    /// Consume this session, returning its ociman container handle.
    #[must_use]
    pub fn into_ociman_container(self) -> ociman::Container {
        self.container
    }

    /// Read and decode pg-ephemeral [`crate::label::Metadata`] from this
    /// session's container labels. Used by staleness detection and any
    /// other surface that needs to compare a session's recorded
    /// configuration to current state.
    pub async fn metadata(&self) -> Result<crate::label::Metadata, MetadataError> {
        let labels = self.container.labels().await?;
        Ok(crate::label::read_container(&labels)?)
    }

    /// Stop and remove the underlying container, consuming this `Session`.
    ///
    /// Force-removes via the runtime, which stops the container first if
    /// it's still running. After this call returns successfully the
    /// session no longer exists on the backend.
    pub async fn stop(mut self) -> Result<(), StopError> {
        self.container
            .remove_force()
            .await
            .map_err(StopError::Remove)
    }

    /// List every running pg-ephemeral session on the given backend.
    ///
    /// Containers are discovered by filtering on the
    /// [`crate::label::SESSION_KEY`] label. For each match, the container's
    /// labels are inspected to decode the session name — so this performs
    /// one `ps --filter` plus one `inspect` per match.
    pub async fn list(backend: &ociman::Backend) -> Result<Vec<Self>, ListError> {
        Self::list_filtered(backend, None).await
    }

    /// Look up a session by its user-facing name.
    ///
    /// Returns `Ok(None)` when no session with this name is running.
    /// Multiple matches indicate an invariant violation (the OCI name
    /// derivation enforces uniqueness on `start`) and surface as
    /// [`FindError::MultipleMatches`].
    pub async fn find(backend: &ociman::Backend, name: &Name) -> Result<Option<Self>, FindError> {
        // `Name` is already validated as a `ContainerName`, whose character
        // set is a strict subset of what a label value allows — so this
        // conversion cannot fail in practice.
        let value: ociman::label::Value = name.as_str().to_string().try_into().unwrap();
        let mut sessions = Self::list_filtered(backend, Some(&value))
            .await
            .map_err(FindError::List)?;
        match sessions.len() {
            0 => Ok(None),
            1 => Ok(Some(sessions.pop().unwrap())),
            count => Err(FindError::MultipleMatches { count }),
        }
    }

    async fn list_filtered(
        backend: &ociman::Backend,
        value: Option<&ociman::label::Value>,
    ) -> Result<Vec<Self>, ListError> {
        let key = label::SESSION_KEY;
        let filter = match value {
            None => ociman::label::Filter::key_only(&key),
            Some(value) => ociman::label::Filter::exact(&key, value),
        };
        let entries = backend
            .container_list_with_name([filter])
            .await
            .map_err(ListError::ListWithName)?;

        entries
            .into_iter()
            .map(|(container, container_name)| {
                let raw = container_name
                    .as_str()
                    .strip_prefix(Name::OCI_PREFIX)
                    .ok_or(ListError::MissingOciPrefix {
                        container_name: container_name.clone(),
                    })?;
                let name: Name = raw.parse().map_err(ListError::InvalidSessionName)?;
                Ok(Self { container, name })
            })
            .collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum StopError {
    #[error("failed to remove session container")]
    Remove(#[source] cmd_proc::CommandError),
}

#[derive(Debug, thiserror::Error)]
pub enum FindError {
    #[error(transparent)]
    List(#[from] ListError),
    /// More than one container carried the same session label value.
    /// `Definition::session_name` derives a deterministic OCI container
    /// name and the runtime atomically rejects duplicates on `start`, so
    /// this should be impossible — it indicates a tampered or
    /// hand-crafted container that bypassed the start path.
    #[error("multiple containers ({count}) carry the same session label value")]
    MultipleMatches { count: usize },
}

#[derive(Debug, thiserror::Error)]
pub enum MetadataError {
    #[error("failed to read session container labels")]
    ReadLabels(#[from] ociman::label::ContainerError),
    #[error("failed to decode pg-ephemeral metadata from session labels")]
    Decode(#[from] crate::label::ReadError),
}

/// Seed-chain status of a running session relative to the current
/// instance config.
#[derive(Clone, Debug, PartialEq)]
pub enum SeedStatus {
    /// Chain shape and every seed hash match — re-booting today would
    /// land on the same cache layer the session is running.
    Sync,
    /// Anything else: base image differs, chain shape differs (seeds
    /// added/removed/renamed/reordered), or at least one seed hash
    /// differs at a matching-name position.
    Diverged,
}

impl std::fmt::Display for SeedStatus {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Sync => formatter.write_str("sync"),
            Self::Diverged => formatter.write_str("diverged"),
        }
    }
}

/// Diff a session's stored seed chain (decoded from labels) against the
/// current instance config's chain. Binary outcome — see [`SeedStatus`].
///
/// Pure: no IO. Callers feed it the decoded [`crate::label::Metadata`]
/// (from [`Session::metadata`]) plus the current
/// [`crate::seed::LoadedSeeds`] (from `definition.load_seeds(...)`).
#[must_use]
pub fn compute_seed_status(
    stored_image: &ociman::image::Reference,
    stored_seeds: &[crate::label::SeedEntry],
    current_image: &crate::image::Image,
    current_seeds: &crate::seed::LoadedSeeds<'_>,
) -> SeedStatus {
    let current_image_reference = ociman::image::Reference::from(current_image);
    if stored_image != &current_image_reference {
        return SeedStatus::Diverged;
    }
    let current: Vec<_> = current_seeds.iter_seeds().collect();
    if stored_seeds.len() != current.len() {
        return SeedStatus::Diverged;
    }
    for (stored_entry, current_seed) in stored_seeds.iter().zip(current.iter()) {
        if &stored_entry.name != current_seed.name() {
            return SeedStatus::Diverged;
        }
        if stored_entry.hash.as_ref() != current_seed.cache_status().hash() {
            return SeedStatus::Diverged;
        }
    }
    SeedStatus::Sync
}

#[derive(Debug, thiserror::Error)]
pub enum ListError {
    #[error("failed to list session containers")]
    ListWithName(#[source] ociman::backend::ContainerListWithNameError),
    /// A container carried [`label::SESSION_KEY`] but its runtime name did not
    /// start with [`Name::OCI_PREFIX`]. Sessions can only be created through
    /// our codepath (which sets the name via [`Name::container_name`]), so
    /// this indicates a hand-crafted or tampered container.
    #[error(
        "container {container_name} matched session label filter but name does not start with {:?}",
        Name::OCI_PREFIX
    )]
    MissingOciPrefix {
        container_name: ociman::ContainerName,
    },
    #[error("session container name suffix is not a valid session name")]
    InvalidSessionName(#[source] ociman::ContainerNameError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn name_derives_prefixed_oci_name() {
        let session: Name = "foo".parse().unwrap();
        assert_eq!(session.as_str(), "foo");
        assert_eq!(session.to_string(), "foo");
        assert_eq!(
            session.container_name().as_str(),
            "pg-ephemeral-session-foo"
        );
    }

    #[test]
    fn name_rejects_invalid_user_facing() {
        // Empty input fails ContainerName validation.
        assert!("".parse::<Name>().is_err());
        // Leading dash is invalid for ContainerName.
        assert!("-foo".parse::<Name>().is_err());
    }
}
