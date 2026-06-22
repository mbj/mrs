//! Host-local advisory lock that deduplicates concurrent cache builds.
//!
//! The first process to arrive builds the seed chain while others block on the
//! lock, then wake to find the images already present and do no build work.
//! Keyed by the cache image name. Released explicitly via [`CacheLock::release`];
//! the kernel also frees it when the holding process exits, so a crashed holder
//! never strands it.

/// Failure acquiring the [`CacheLock`].
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to open cache lock file {}", path.display())]
    Open {
        path: std::path::PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("Failed to acquire cache lock {}", path.display())]
    Lock {
        path: std::path::PathBuf,
        #[source]
        source: rustix::io::Errno,
    },
    #[error("Failed to release cache lock {}", path.display())]
    Unlock {
        path: std::path::PathBuf,
        #[source]
        source: rustix::io::Errno,
    },
    #[error(
        "No per-user directory available for the cache lock (no XDG_RUNTIME_DIR, state, or cache dir)"
    )]
    NoLockDir,
    #[error("Cache lock task panicked")]
    Join(#[source] tokio::task::JoinError),
}

/// Held for the duration of a cache build. Release it with
/// [`CacheLock::release`], which unlocks explicitly and surfaces any error. If
/// the guard is instead dropped (an error or panic path), the kernel still
/// frees the `flock` when the descriptor closes — but that backstop swallows
/// errors, so the explicit path is preferred.
#[derive(Debug)]
pub struct CacheLock {
    file: std::fs::File,
    path: std::path::PathBuf,
}

impl CacheLock {
    /// Acquire the exclusive build lock for `image_name`, blocking until free.
    /// Logs once at info level only when another process already holds it.
    pub async fn acquire(image_name: &ociman::reference::Name) -> Result<Self, Error> {
        let path = lock_path(image_name).ok_or(Error::NoLockDir)?;
        // open, the non-blocking try, and the blocking wait can all park an OS
        // thread, so the whole acquisition runs on the blocking pool.
        let acquire_path = path.clone();
        let file = tokio::task::spawn_blocking(move || acquire_blocking(acquire_path))
            .await
            .map_err(Error::Join)??;
        Ok(Self { file, path })
    }

    /// Release the lock explicitly, surfacing any unlock error. Consumes the
    /// guard; the descriptor closes as it drops.
    pub async fn release(self) -> Result<(), Error> {
        let Self { file, path } = self;
        tokio::task::spawn_blocking(move || {
            rustix::fs::flock(&file, rustix::fs::FlockOperation::Unlock)
                .map_err(|source| Error::Unlock { path, source })
        })
        .await
        .map_err(Error::Join)?
    }
}

fn open(path: &std::path::Path) -> Result<std::fs::File, Error> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|source| Error::Open {
            path: path.to_path_buf(),
            source,
        })?;
    }
    std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(false)
        .open(path)
        .map_err(|source| Error::Open {
            path: path.to_path_buf(),
            source,
        })
}

/// Open the lock file and take the exclusive lock, blocking until free. Logs
/// once only when another process already holds it. Runs entirely on a blocking
/// thread (see [`CacheLock::acquire`]).
fn acquire_blocking(path: std::path::PathBuf) -> Result<std::fs::File, Error> {
    let file = open(&path)?;

    // Try without blocking first so we only log on real contention.
    match rustix::fs::flock(&file, rustix::fs::FlockOperation::NonBlockingLockExclusive) {
        Ok(()) => {}
        Err(errno)
            if errno == rustix::io::Errno::WOULDBLOCK || errno == rustix::io::Errno::AGAIN =>
        {
            log::info!(
                "Waiting for cache build lock held by another process: {}",
                path.display()
            );
            rustix::fs::flock(&file, rustix::fs::FlockOperation::LockExclusive)
                .map_err(|source| Error::Lock { path, source })?;
        }
        Err(source) => return Err(Error::Lock { path, source }),
    }

    Ok(file)
}

/// Lock file path for a cache image name. The name is sanitized into one
/// filesystem-safe component; distinct names can only collide into over-
/// serialization, never a missed build, so an exact mapping is not required.
fn lock_path(image_name: &ociman::reference::Name) -> Option<std::path::PathBuf> {
    let sanitized: String = image_name
        .to_string()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();

    let mut path = lock_dir()?;
    path.push(format!("{sanitized}.lock"));
    Some(path)
}

/// Per-user base directory for lock files. Prefers the runtime dir
/// (`$XDG_RUNTIME_DIR`, mode 0700), falling back to the state then cache dir —
/// all owned by the current user under a non-shared parent, so no other user
/// can squat the path. `None` when none resolve (e.g. no `$HOME`); callers
/// then refuse to fall back to a world-shared location.
fn lock_dir() -> Option<std::path::PathBuf> {
    let mut path = dirs::runtime_dir()
        .or_else(dirs::state_dir)
        .or_else(dirs::cache_dir)?;
    path.push("pg-ephemeral");
    path.push("locks");
    Some(path)
}
