//! Child-side: be the reaper if this process was invoked as one.

use tokio::io::AsyncRead;

use crate::framing;
use crate::message::{Message, Registration};

/// Environment variable the parent sets to summon a child process into
/// reaper-child mode. [`summon_if_invoked`] reads this; if set, the
/// child takes over as the reaper and never returns. If unset, the
/// child returns immediately and normal program flow continues.
pub(crate) const ENV_VAR: cmd_proc::EnvVariableName =
    cmd_proc::EnvVariableName::from_static_or_panic("MREAPER_SUMMON");

/// Call at the top of `main()`. If this process was invoked as a reaper
/// child (per [`ENV_VAR`]), runs the reactor loop and never returns.
/// Otherwise returns immediately so the host's normal `main` flow
/// continues.
///
/// Must be called from an async context (typical: `#[tokio::main]`).
pub async fn summon_if_invoked() {
    if std::env::var_os(ENV_VAR.as_str()).is_none() {
        return;
    }
    run_child().await;
    std::process::exit(0);
}

async fn run_child() {
    let mut stdin = tokio::io::stdin();
    let state = accumulate(&mut stdin).await;

    match &state.termination {
        Termination::Shutdown => {}
        Termination::Eof => {
            log::warn!("mreaper: parent terminated unexpectedly; running registered sweeps");
        }
        Termination::ProtocolError(error) => {
            log::error!("mreaper: protocol error reading from parent: {error}");
            log::warn!("mreaper: parent terminated unexpectedly; running registered sweeps");
        }
    }

    for registration in state.sweeps {
        if let Err(error) = registration.execute().await {
            log::warn!("mreaper: sweep execution failed: {error}");
        }
    }
}

/// Snapshot of the parent → child message stream at end-of-stream.
struct State {
    sweeps: Vec<Registration>,
    termination: Termination,
}

/// Why the read loop ended.
#[derive(Debug)]
enum Termination {
    /// Stream ended without an explicit [`Message::Shutdown`]. Typically
    /// means the parent dropped the pipe (clean exit without sending
    /// shutdown, or the parent crashed).
    Eof,
    /// Loop observed an explicit [`Message::Shutdown`] from the parent.
    Shutdown,
    /// Loop hit a framing or decoding error and gave up on the stream.
    ProtocolError(framing::ReadError),
}

/// Read messages from `reader` until EOF, [`Message::Shutdown`], or a
/// protocol error. Pure protocol handling — no sweep execution.
async fn accumulate<R: AsyncRead + Unpin>(reader: &mut R) -> State {
    let mut sweeps = Vec::new();

    let termination = loop {
        match framing::read_message(reader).await {
            Ok(None) => break Termination::Eof,
            Ok(Some(Message::Register(registration))) => sweeps.push(registration),
            Ok(Some(Message::Shutdown)) => break Termination::Shutdown,
            Err(error) => break Termination::ProtocolError(error),
        }
    };

    State {
        sweeps,
        termination,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use ociman::label;
    use tokio::io::AsyncWriteExt;

    fn dummy_backend() -> ociman::Backend {
        ociman::Backend::Podman {
            version: semver::Version::new(5, 4, 0),
        }
    }

    fn dummy_registration() -> Registration {
        let key = label::Key::from_static_or_panic("mreaper-test.marker");
        let value = label::Value::from_static_or_panic("present");
        Registration::ContainerLabel(crate::message::ContainerLabel::new(
            dummy_backend(),
            &key,
            &value,
        ))
    }

    #[tokio::test]
    async fn empty_stream_terminates_on_eof() {
        let (writer, mut reader) = tokio::io::duplex(1024);
        drop(writer);
        let state = accumulate(&mut reader).await;
        assert!(state.sweeps.is_empty());
        assert!(matches!(state.termination, Termination::Eof));
    }

    #[tokio::test]
    async fn shutdown_only_terminates_on_shutdown() {
        let (mut writer, mut reader) = tokio::io::duplex(1024);
        framing::write_message(&mut writer, &Message::Shutdown)
            .await
            .unwrap();
        drop(writer);
        let state = accumulate(&mut reader).await;
        assert!(state.sweeps.is_empty());
        assert!(matches!(state.termination, Termination::Shutdown));
    }

    #[tokio::test]
    async fn one_register_then_eof() {
        let (mut writer, mut reader) = tokio::io::duplex(1024);
        framing::write_message(&mut writer, &Message::Register(dummy_registration()))
            .await
            .unwrap();
        drop(writer);
        let state = accumulate(&mut reader).await;
        assert_eq!(state.sweeps.len(), 1);
        assert!(matches!(state.termination, Termination::Eof));
    }

    #[tokio::test]
    async fn one_register_then_shutdown() {
        let (mut writer, mut reader) = tokio::io::duplex(1024);
        framing::write_message(&mut writer, &Message::Register(dummy_registration()))
            .await
            .unwrap();
        framing::write_message(&mut writer, &Message::Shutdown)
            .await
            .unwrap();
        drop(writer);
        let state = accumulate(&mut reader).await;
        assert_eq!(state.sweeps.len(), 1);
        assert!(matches!(state.termination, Termination::Shutdown));
    }

    #[tokio::test]
    async fn multiple_registers_preserved_in_order() {
        let (mut writer, mut reader) = tokio::io::duplex(1024);
        for _ in 0..3 {
            framing::write_message(&mut writer, &Message::Register(dummy_registration()))
                .await
                .unwrap();
        }
        drop(writer);
        let state = accumulate(&mut reader).await;
        assert_eq!(state.sweeps.len(), 3);
        assert!(matches!(state.termination, Termination::Eof));
    }

    #[tokio::test]
    async fn over_cap_length_yields_protocol_error() {
        let (mut writer, mut reader) = tokio::io::duplex(8);
        let oversize = u32::try_from(framing::MAX_PAYLOAD + 1).unwrap();
        writer.write_all(&oversize.to_le_bytes()).await.unwrap();
        drop(writer);
        let state = accumulate(&mut reader).await;
        assert!(state.sweeps.is_empty());
        assert!(matches!(
            state.termination,
            Termination::ProtocolError(framing::ReadError::PayloadTooLarge { .. })
        ));
    }

    #[tokio::test]
    async fn malformed_payload_yields_protocol_error() {
        let (mut writer, mut reader) = tokio::io::duplex(1024);
        // Length-prefix says 16 bytes of payload, then we deliver 16 bytes
        // of garbage that won't parse as a Message.
        writer.write_all(&16u32.to_le_bytes()).await.unwrap();
        writer.write_all(&[0xff; 16]).await.unwrap();
        drop(writer);
        let state = accumulate(&mut reader).await;
        assert!(state.sweeps.is_empty());
        assert!(matches!(
            state.termination,
            Termination::ProtocolError(framing::ReadError::Decode(_))
        ));
    }
}
