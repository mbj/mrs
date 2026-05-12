//! Wire framing for parent → child messages over the stdin pipe.
//!
//! Format: `<u32 LE payload_length><JSON payload>`. The fixed 4-byte
//! prefix bounds reads — the reader does `read_exact(4)` then
//! `read_exact(len)`, never buffering until a delimiter. Sanity-capped
//! at [`MAX_PAYLOAD`] to keep a misbehaving sender from making the
//! child allocate gigabytes.

use std::io;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::message::Message;

/// Maximum payload bytes per message. Real messages are well under
/// 1 KB; this cap protects the child from a misbehaving sender.
pub const MAX_PAYLOAD: usize = 1024 * 1024;

pub async fn write_message<W: AsyncWrite + Unpin>(
    writer: &mut W,
    message: &Message,
) -> Result<(), WriteError> {
    let payload = serde_json::to_vec(message).map_err(WriteError::Encode)?;
    let len = u32::try_from(payload.len()).map_err(|_| WriteError::PayloadTooLarge)?;
    writer
        .write_all(&len.to_le_bytes())
        .await
        .map_err(WriteError::Io)?;
    writer.write_all(&payload).await.map_err(WriteError::Io)?;
    writer.flush().await.map_err(WriteError::Io)?;
    Ok(())
}

/// Read one framed message. Returns `Ok(None)` on clean EOF (zero
/// bytes read at the start of a frame); `Err(_)` on partial frames,
/// over-cap lengths, IO failure, or malformed JSON.
pub async fn read_message<R: AsyncRead + Unpin>(
    reader: &mut R,
) -> Result<Option<Message>, ReadError> {
    let mut len_buf = [0u8; 4];
    match reader.read_exact(&mut len_buf).await {
        Ok(_) => {}
        Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => return Ok(None),
        Err(error) => return Err(ReadError::Io(error)),
    }
    let len = u32::from_le_bytes(len_buf) as usize;
    if len > MAX_PAYLOAD {
        return Err(ReadError::PayloadTooLarge { len });
    }
    let mut payload = vec![0u8; len];
    reader
        .read_exact(&mut payload)
        .await
        .map_err(ReadError::Io)?;
    let message = serde_json::from_slice(&payload).map_err(ReadError::Decode)?;
    Ok(Some(message))
}

#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    #[error("payload exceeds u32::MAX bytes")]
    PayloadTooLarge,
    #[error("failed to encode message as JSON")]
    Encode(#[source] serde_json::Error),
    #[error("failed to write framed message")]
    Io(#[source] io::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("payload length {len} exceeds maximum {MAX_PAYLOAD}")]
    PayloadTooLarge { len: usize },
    #[error("failed to decode JSON payload")]
    Decode(#[source] serde_json::Error),
    #[error("failed to read framed message")]
    Io(#[source] io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn round_trip_shutdown_in_memory() {
        let (mut writer, mut reader) = tokio::io::duplex(1024);
        write_message(&mut writer, &Message::Shutdown)
            .await
            .unwrap();
        drop(writer);
        let read_back = read_message(&mut reader).await.unwrap().unwrap();
        assert!(matches!(read_back, Message::Shutdown));
    }

    #[tokio::test]
    async fn eof_returns_none() {
        let (writer, mut reader) = tokio::io::duplex(1024);
        drop(writer);
        let result = read_message(&mut reader).await.unwrap();
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn over_cap_length_errors() {
        let (mut writer, mut reader) = tokio::io::duplex(8);
        let oversize = u32::try_from(MAX_PAYLOAD + 1).unwrap();
        writer.write_all(&oversize.to_le_bytes()).await.unwrap();
        let result = read_message(&mut reader).await;
        assert!(matches!(result, Err(ReadError::PayloadTooLarge { .. })));
    }
}
