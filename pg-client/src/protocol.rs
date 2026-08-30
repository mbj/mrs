//! See: <https://www.postgresql.org/docs/current/protocol-message-formats.html>

pub mod backend;
pub mod frontend;

use core::future::Future;
use core::ops::Range;
use std::borrow::Cow;
use std::sync::Arc;

pub trait Writer {
    fn write<'a>(
        &'a mut self,
        bytes: &'a [u8],
    ) -> impl Future<Output = std::io::Result<usize>> + Send + 'a;

    /// Nothing for a socket, which writes straight to the kernel. TLS encrypts
    /// into a buffer of its own, which an accepted write does not promise to
    /// have drained.
    fn flush(&mut self) -> impl Future<Output = std::io::Result<()>> + Send;
}

pub trait Reader {
    fn read<'a>(
        &'a mut self,
        buffer: &'a mut Vec<u8>,
    ) -> impl Future<Output = std::io::Result<usize>> + Send + 'a;
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Format {
    #[default]
    Text,
    Binary,
}

impl Format {
    #[must_use]
    pub const fn code(self) -> i16 {
        match self {
            Self::Text => 0,
            Self::Binary => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Version {
    V3_0,
    V3_2,
}

impl Version {
    const CODE_3_0: u32 = 0x0003_0000;
    const CODE_3_2: u32 = 0x0003_0002;

    #[must_use]
    pub const fn from_code(code: u32) -> Option<Self> {
        match code {
            Self::CODE_3_0 => Some(Self::V3_0),
            Self::CODE_3_2 => Some(Self::V3_2),
            _ => None,
        }
    }

    #[must_use]
    pub const fn code(self) -> u32 {
        match self {
            Self::V3_0 => Self::CODE_3_0,
            Self::V3_2 => Self::CODE_3_2,
        }
    }
}

/// The protocol's [`String`](https://www.postgresql.org/docs/current/protocol-message-types.html)
/// data type.
///
/// On the wire a `String` is terminated by NUL, so a NUL inside the text would
/// end the field early and leave the rest of the message misread. Making
/// "contains no NUL" a property of the value means it is checked once where it
/// is made, instead of being a promise each caller has to keep.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct String<'a>(Cow<'a, str>);

impl String<'static> {
    pub const EMPTY: Self = Self(Cow::Borrowed(""));

    #[must_use]
    pub const fn from_static_or_panic(text: &'static str) -> Self {
        let bytes = text.as_bytes();
        let mut index = 0;

        while index < bytes.len() {
            assert!(bytes[index] != 0, "protocol string carries a NUL");
            index += 1;
        }

        Self(Cow::Borrowed(text))
    }
}

impl<'a> String<'a> {
    pub fn new(text: impl Into<Cow<'a, str>>) -> Result<Self, StringError> {
        let text = text.into();

        match text.bytes().position(|byte| byte == 0) {
            Some(index) => Err(StringError::ContainsNul { index }),
            None => Ok(Self(text)),
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    #[must_use]
    pub fn into_string(self) -> std::string::String {
        self.0.into_owned()
    }

    #[must_use]
    pub fn into_owned(self) -> String<'static> {
        String(Cow::Owned(self.0.into_owned()))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum StringError {
    #[error("text contains a NUL at byte {index}, which would end the string there")]
    ContainsNul { index: usize },
}

/// Upstream writes the wire field as `Int32`, meaning four bytes, but
/// PostgreSQL's own `Oid` is unsigned, and a cluster that has been running
/// long enough issues values above `i32::MAX`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Oid(u32);

impl Oid {
    /// Several fields use this to mean the object is not identified.
    pub const NONE: Self = Self(0);

    #[must_use]
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    #[must_use]
    pub const fn get(self) -> u32 {
        self.0
    }

    #[must_use]
    pub const fn is_none(self) -> bool {
        self.0 == Self::NONE.0
    }
}

const LENGTH_PREFIX_SIZE: usize = 4;

pub const HEADER_SIZE: usize = 1 + LENGTH_PREFIX_SIZE;

/// Largest body PostgreSQL can construct.
///
/// Backend messages are built in a `StringInfo`, which refuses to grow beyond
/// `MaxAllocSize`, so no server can emit a larger body and no server accepts
/// one. A frame claiming more than this did not come from PostgreSQL.
pub const MAX_BODY_LENGTH: usize = 0x3FFF_FFFF;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReceiveBufferSize {
    ordinary: usize,
    oversized: usize,
}

impl ReceiveBufferSize {
    /// `ordinary` stays below glibc's 128 KiB `M_MMAP_THRESHOLD`, so that
    /// allocating fresh storage while frames are outstanding is served from the
    /// heap rather than taking a page fault per page. `oversized` clears the
    /// frame sizes ordinary schemas produce, leaving only large values to be
    /// released.
    pub const DEFAULT: Self = Self {
        ordinary: 64 * 1024,
        oversized: 512 * 1024,
    };

    /// Set `oversized` above the largest frame the workload produces routinely.
    /// A routine frame size above it becomes two allocations on every read,
    /// which glibc absorbs and musl does not: measured at 16x on a stream where
    /// five percent of frames sat just above the budget.
    pub const fn new(ordinary: usize, oversized: usize) -> Result<Self, ReceiveBufferSizeError> {
        if ordinary < HEADER_SIZE {
            return Err(ReceiveBufferSizeError::OrdinaryBelowHeader { ordinary });
        }

        if oversized < ordinary {
            return Err(ReceiveBufferSizeError::OversizedBelowOrdinary {
                ordinary,
                oversized,
            });
        }

        Ok(Self {
            ordinary,
            oversized,
        })
    }

    #[must_use]
    pub const fn ordinary(self) -> usize {
        self.ordinary
    }

    #[must_use]
    pub const fn oversized(self) -> usize {
        self.oversized
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ReceiveBufferSizeError {
    #[error("ordinary size {ordinary} cannot hold a frame header of {HEADER_SIZE} bytes")]
    OrdinaryBelowHeader { ordinary: usize },
    #[error("oversized threshold {oversized} is below the ordinary size {ordinary}")]
    OversizedBelowOrdinary { ordinary: usize, oversized: usize },
}

pub struct ReceiveBuffer {
    chunk: Arc<Vec<u8>>,
    consumed: usize,
    size: ReceiveBufferSize,
}

impl core::fmt::Debug for ReceiveBuffer {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("ReceiveBuffer")
            .field("capacity", &self.chunk.capacity())
            .field("filled", &self.chunk.len())
            .field("consumed", &self.consumed)
            .finish()
    }
}

impl ReceiveBuffer {
    #[must_use]
    pub fn new(size: ReceiveBufferSize) -> Self {
        Self {
            chunk: Arc::new(Vec::with_capacity(size.ordinary())),
            consumed: 0,
            size,
        }
    }

    pub async fn read_frame<R: Reader>(&mut self, reader: &mut R) -> Result<Frame, ReceiveError> {
        loop {
            let unparsed = &self.chunk[self.consumed..];

            let Some(header) = unparsed.get(..HEADER_SIZE) else {
                self.fill(reader, HEADER_SIZE).await?;
                continue;
            };

            let declared = i32::from_be_bytes([header[1], header[2], header[3], header[4]]);

            let body_length = usize::try_from(declared)
                .ok()
                .and_then(|value| value.checked_sub(LENGTH_PREFIX_SIZE))
                .ok_or(InvalidLength::TooShort { length: declared })?;

            if body_length > MAX_BODY_LENGTH {
                return Err(InvalidLength::TooLong { length: declared }.into());
            }

            let length = HEADER_SIZE + body_length;

            if unparsed.len() >= length {
                return Ok(self.claim(length));
            }

            self.fill(reader, length).await?;
        }
    }

    async fn fill<R: Reader>(
        &mut self,
        reader: &mut R,
        required: usize,
    ) -> Result<(), ReceiveError> {
        let outstanding = self.consumed..self.chunk.len();
        let outstanding_length = outstanding.len();

        let grown = self.chunk.capacity();

        let baseline = if grown <= self.size.oversized() {
            grown
        } else {
            self.size.ordinary()
        };

        match Arc::get_mut(&mut self.chunk) {
            Some(chunk)
                if chunk.capacity() >= required && chunk.capacity() <= self.size.oversized() =>
            {
                if chunk.capacity() - self.consumed < required {
                    chunk.copy_within(outstanding, 0);
                    chunk.truncate(outstanding_length);

                    self.consumed = 0;
                }

                Self::read(reader, chunk).await?;
            }
            _ => {
                let mut chunk = Vec::with_capacity(required.max(baseline));

                chunk.extend_from_slice(&self.chunk[outstanding]);

                Self::read(reader, &mut chunk).await?;

                self.chunk = Arc::new(chunk);
                self.consumed = 0;
            }
        }

        Ok(())
    }

    async fn read<R: Reader>(reader: &mut R, chunk: &mut Vec<u8>) -> Result<(), ReceiveError> {
        if reader.read(chunk).await.map_err(ReceiveError::Read)? == 0 {
            return Err(ReceiveError::Closed);
        }

        Ok(())
    }

    fn claim(&mut self, length: usize) -> Frame {
        let start = self.consumed;
        let tag = self.chunk[start];
        let body = (start + HEADER_SIZE)..(start + length);

        self.consumed += length;

        Frame::new(tag, Body::claim(Arc::clone(&self.chunk), body))
    }
}

impl Default for ReceiveBuffer {
    fn default() -> Self {
        Self::new(ReceiveBufferSize::DEFAULT)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ReceiveError {
    #[error("stream closed")]
    Closed,
    #[error("failed to read frame from stream")]
    Read(#[source] std::io::Error),
    #[error(transparent)]
    InvalidLength(#[from] InvalidLength),
}

/// PostgreSQL sets `TCP_NODELAY`, so this buffer is the only coalescing there
/// is.
#[derive(Debug, Default)]
pub struct SendBuffer {
    bytes: Vec<u8>,
    written: usize,
}

#[derive(Debug, thiserror::Error)]
pub enum SendError {
    #[error("stream closed")]
    Closed,
    #[error("failed to write frames to stream")]
    Write(#[source] std::io::Error),
}

impl SendBuffer {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message(
        &mut self,
        tag: u8,
        body: impl FnOnce(&mut frontend::Writer<'_>) -> Result<(), frontend::MessageError>,
    ) -> Result<(), frontend::MessageError> {
        self.discarding_on_error(|bytes| {
            bytes.push(tag);

            Self::frame(bytes, body)
        })
    }

    pub fn untagged(
        &mut self,
        body: impl FnOnce(&mut frontend::Writer<'_>) -> Result<(), frontend::MessageError>,
    ) -> Result<(), frontend::MessageError> {
        self.discarding_on_error(|bytes| Self::frame(bytes, body))
    }

    // A message that fails part way must not leave its first bytes queued.
    fn discarding_on_error(
        &mut self,
        write: impl FnOnce(&mut Vec<u8>) -> Result<(), frontend::MessageError>,
    ) -> Result<(), frontend::MessageError> {
        let start = self.bytes.len();

        write(&mut self.bytes).inspect_err(|_| self.bytes.truncate(start))
    }

    fn frame(
        bytes: &mut Vec<u8>,
        body: impl FnOnce(&mut frontend::Writer<'_>) -> Result<(), frontend::MessageError>,
    ) -> Result<(), frontend::MessageError> {
        let mut writer = frontend::Writer::new(bytes);

        let length = writer.reserve::<LENGTH_PREFIX_SIZE>();

        body(&mut writer)?;

        // Unlike a parameter's, a message's length counts its own four bytes.
        let total = writer.written_since(&length) + LENGTH_PREFIX_SIZE;
        let size = i32::try_from(total).map_err(|_| frontend::TooLong { length: total })?;

        writer.fill(length, size.to_be_bytes());

        Ok(())
    }

    pub async fn flush<W: Writer>(&mut self, writer: &mut W) -> Result<(), SendError> {
        while self.written < self.bytes.len() {
            let count = writer
                .write(&self.bytes[self.written..])
                .await
                .map_err(SendError::Write)?;

            if count == 0 {
                return Err(SendError::Closed);
            }

            self.written += count;
        }

        writer.flush().await.map_err(SendError::Write)?;

        self.bytes.clear();
        self.written = 0;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Body {
    chunk: Arc<Vec<u8>>,
    range: Range<usize>,
}

impl core::fmt::Debug for Body {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("Body")
            .field("length", &self.range.len())
            .finish()
    }
}

impl Body {
    #[must_use]
    pub const fn claim(chunk: Arc<Vec<u8>>, range: Range<usize>) -> Self {
        Self { chunk, range }
    }

    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.chunk[self.range.start..self.range.end]
    }
}

#[derive(Clone, Debug)]
pub struct Frame {
    tag: u8,
    body: Body,
}

impl Frame {
    #[must_use]
    pub const fn new(tag: u8, body: Body) -> Self {
        Self { tag, body }
    }

    #[must_use]
    pub const fn tag(&self) -> u8 {
        self.tag
    }

    #[must_use]
    pub const fn body(&self) -> &Body {
        &self.body
    }
}

pub fn encode(tag: u8, body: &[u8], target: &mut Vec<u8>) {
    assert!(body.len() <= MAX_BODY_LENGTH);

    let length = i32::try_from(body.len() + LENGTH_PREFIX_SIZE)
        .expect("frame length exceeds the Int32 length field");

    target.reserve(HEADER_SIZE + body.len());
    target.push(tag);
    target.extend_from_slice(&length.to_be_bytes());
    target.extend_from_slice(body);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum InvalidLength {
    #[error("frame declares length {length}, minimum is {LENGTH_PREFIX_SIZE}")]
    TooShort { length: i32 },
    #[error("frame declares length {length}, maximum body is {MAX_BODY_LENGTH}")]
    TooLong { length: i32 },
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn refuses_an_ordinary_size_that_cannot_hold_a_frame_header() {
        assert_eq!(
            ReceiveBufferSize::new(4, 1024),
            Err(ReceiveBufferSizeError::OrdinaryBelowHeader { ordinary: 4 })
        );
    }

    #[test]
    fn encodes_tag_length_and_body() {
        let mut target = Vec::new();

        encode(b'P', &[0x01, 0x02, 0x03], &mut target);

        assert_eq!(target, vec![b'P', 0x00, 0x00, 0x00, 0x07, 0x01, 0x02, 0x03]);
    }

    async fn read_frame(bytes: &[u8]) -> Result<Frame, ReceiveError> {
        ReceiveBuffer::new(ReceiveBufferSize::DEFAULT)
            .read_frame(&mut std::io::Cursor::new(bytes.to_vec()))
            .await
    }

    fn invalid_length(error: ReceiveError) -> InvalidLength {
        match error {
            ReceiveError::InvalidLength(invalid) => invalid,
            other => panic!("expected an invalid length, got {other:?}"),
        }
    }

    #[test]
    fn writes_a_tagged_message_and_an_untagged_one() {
        let mut buffer = SendBuffer::new();

        buffer
            .message(b'P', |writer| {
                writer.string(&String::EMPTY);
                writer.int16(0);

                Ok(())
            })
            .unwrap();

        buffer
            .untagged(|writer| {
                writer.int32(196_608);
                writer.byte(0);

                Ok(())
            })
            .unwrap();

        assert_eq!(
            buffer.bytes,
            [
                [b'P', 0x00, 0x00, 0x00, 0x07, 0x00, 0x00, 0x00].as_slice(),
                &[0x00, 0x00, 0x00, 0x09, 0x00, 0x03, 0x00, 0x00, 0x00],
            ]
            .concat(),
        );
    }

    #[tokio::test]
    async fn reads_a_frame() {
        let frame = read_frame(&[b'P', 0x00, 0x00, 0x00, 0x07, 0x01, 0x02, 0x03])
            .await
            .unwrap();

        assert_eq!(frame.tag(), b'P');
        assert_eq!(frame.body().as_slice(), &[0x01, 0x02, 0x03]);
    }

    #[tokio::test]
    async fn rejects_a_length_no_server_can_send() {
        assert_eq!(
            invalid_length(
                read_frame(&[b'D', 0x40, 0x00, 0x00, 0x04])
                    .await
                    .unwrap_err()
            ),
            InvalidLength::TooLong {
                length: 0x4000_0004
            },
        );
        assert_eq!(
            invalid_length(
                read_frame(&[b'D', 0x00, 0x00, 0x00, 0x03])
                    .await
                    .unwrap_err()
            ),
            InvalidLength::TooShort { length: 3 },
        );
    }
}
