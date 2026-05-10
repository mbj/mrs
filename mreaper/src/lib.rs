//! Parent-death-triggered cleanup of OCI containers via label sweep.
//!
//! `mreaper` spawns a sibling reaper process at parent startup. The parent
//! registers sweep predicates (currently: container-label matchers) over the
//! reaper's stdin. When the parent dies for any reason — clean exit, panic,
//! `SIGKILL`, OOM, segfault — the reaper observes EOF on stdin and executes
//! the registered sweeps before exiting itself.
//!
//! v1 covers OCI containers via [`ociman::Backend`]. The reaper child is the
//! same binary as the parent, dispatched by the [`dispatch_if_internal`]
//! function which the host's `main()` must call at startup.

pub mod dispatch;
pub mod framing;
pub mod message;
