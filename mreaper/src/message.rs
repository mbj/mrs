//! Wire messages exchanged over the parent → child pipe, plus their
//! execution semantics.
//!
//! The wire is NDJSON: one [`Message`] JSON object per line, terminated by
//! `\n`, written over the child's stdin. The child reads every line until
//! EOF, accumulates the registrations, then unconditionally executes them.
//!
//! There is intentionally no "stand down" message — the sweep is
//! idempotent (an already-cleaned label set returns no containers), and a
//! "skip work" toggle would be a footgun (forgotten toggle = leaked
//! container). [`Message::Shutdown`] is *not* a stand-down message; it
//! tells the child "this exit was intentional," so EOF after a `Shutdown`
//! is silent. EOF without a prior `Shutdown` is logged as a warning
//! ("parent terminated unexpectedly"). Either way the registered sweeps
//! run.
//!
//! Each cleanup message carries its own already-resolved [`ociman::Backend`]
//! — the reaper itself is backend-agnostic, and the parent's resolution is
//! authoritative (the child does not re-resolve, so env / PATH / config
//! drift between parent and child invocation cannot pick a different
//! backend).
//!
//! Wire fields use string forms for label key/value (since
//! [`ociman::label::Key`] / [`ociman::label::Value`] do not derive
//! [`serde::Serialize`]); the typed forms are recovered via [`std::str::FromStr`]
//! inside [`ContainerLabel::execute`]. The parent constructs `ContainerLabel`
//! via [`ContainerLabel::new`] which requires already-validated typed values,
//! so a child-side parse failure indicates an internal bug rather than user
//! data error.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// One NDJSON line over the parent → child pipe.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Message {
    /// Register a sweep predicate. Multiple registrations accumulate; on
    /// EOF every registered sweep runs. The inner [`Registration`] is
    /// sub-tagged on `kind` so future registration types (process sweep,
    /// volume sweep, …) extend without changing the outer envelope.
    Register(Registration),
    /// Mark the parent's exit as intentional. Sweeps still run on EOF, but
    /// the child suppresses the "parent terminated unexpectedly" warning.
    /// Send this once, just before dropping the handle, on the clean exit
    /// path.
    Shutdown,
}

/// What is being registered. Sub-tagged on `kind`; new registration types
/// add new variants here without touching [`Message`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Registration {
    ContainerLabel(ContainerLabel),
}

impl Registration {
    /// Dispatch to the inner variant's execution. Single point to add new
    /// registration types.
    pub async fn execute(&self) -> Result<(), ExecuteError> {
        match self {
            Self::ContainerLabel(sweep) => sweep.execute().await,
        }
    }
}

/// Sweep predicate: list containers in `backend` whose label `label_key`
/// exactly equals `label_value`, then stop and remove each.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerLabel {
    pub backend: ociman::Backend,
    pub label_key: String,
    pub label_value: String,
}

impl ContainerLabel {
    /// Construct a sweep from already-validated typed values. Use this on the
    /// parent side; the typed `&Key` / `&Value` enforces that the child will
    /// be able to round-trip them via `FromStr`.
    #[must_use]
    pub fn new(
        backend: ociman::Backend,
        label_key: &ociman::label::Key,
        label_value: &ociman::label::Value,
    ) -> Self {
        Self {
            backend,
            label_key: label_key.as_str().to_owned(),
            label_value: label_value.as_str().to_owned(),
        }
    }

    /// List containers matching the label and best-effort force-remove
    /// each (`docker container rm --force`). Per-container failures are
    /// logged and swallowed so a single failure does not abort the rest
    /// of the sweep. Force-remove is appropriate here because the parent
    /// already failed to coordinate a graceful shutdown — waiting on
    /// SIGTERM grace periods only delays leaked-resource recovery.
    pub async fn execute(&self) -> Result<(), ExecuteError> {
        let label_key =
            ociman::label::Key::from_str(&self.label_key).map_err(ExecuteError::ParseLabelKey)?;
        let label_value = ociman::label::Value::from_str(&self.label_value)
            .map_err(ExecuteError::ParseLabelValue)?;

        let containers = self
            .backend
            .list_containers_by_label(&label_key, Some(&label_value))
            .await
            .map_err(ExecuteError::ListContainers)?;

        for mut container in containers {
            if let Err(error) = container.remove_force().await {
                log::warn!(
                    "mreaper: failed to force-remove container {id}: {error}",
                    id = container.id().as_str(),
                );
            }
        }

        Ok(())
    }
}

/// Errors from [`ContainerLabel::execute`].
///
/// Per-container stop / remove failures are not represented here — those are
/// best-effort and logged at the call site. Variants here represent
/// failures that abort the entire sweep.
#[derive(Debug, thiserror::Error)]
pub enum ExecuteError {
    #[error("failed to parse label key from wire form")]
    ParseLabelKey(#[source] ociman::label::Error),
    #[error("failed to parse label value from wire form")]
    ParseLabelValue(#[source] ociman::label::Error),
    #[error("failed to list containers by label")]
    ListContainers(#[source] ociman::backend::ListContainersError),
}
