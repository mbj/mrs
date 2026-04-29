//! Streaming hasher that length-prefixes each push so consecutive segments
//! can't blend into each other under the digest.
//!
//! Callers build up a hash by pushing arbitrary byte segments — each push is
//! unambiguously separated from its neighbours regardless of its content.
//! Callers who want semantic grouping (distinguishing "build args" from
//! "labels" for instance) push a section marker segment of their choosing
//! before the group.
//!
//! # Example
//!
//! ```ignore
//! use ociman::hasher::SegmentHasher;
//!
//! let mut hasher = SegmentHasher::new();
//! hasher.push(b"content");
//! hasher.push(b"hello world");
//! hasher.push(b"labels");
//! hasher.push(b"com.example.role");
//! hasher.push(b"primary");
//! let digest = hasher.finalize();
//! ```

use sha2::{Digest, Sha256};

// Portable length-prefix hashing relies on usize fitting in u64. Every Rust
// target today has usize BITS of 16/32/64. If a future target has a larger
// usize, fail loudly at compile time rather than silently truncating lengths
// and producing bogus digests.
const _: () = assert!(
    usize::BITS <= 64,
    "SegmentHasher assumes usize fits in u64; this target has a larger usize",
);

/// Streaming SHA-256 hasher that length-prefixes each pushed segment.
pub struct SegmentHasher {
    inner: Sha256,
}

impl SegmentHasher {
    #[must_use]
    pub fn new() -> Self {
        Self {
            inner: Sha256::new(),
        }
    }

    /// Absorb `bytes` into the hash as a single segment.
    ///
    /// The length is widened to `u64` and serialised little-endian before the
    /// data, so the resulting digest is stable across hosts of different
    /// native endianness and pointer widths.
    pub fn push(&mut self, bytes: impl AsRef<[u8]>) {
        let bytes = bytes.as_ref();
        self.inner.update((bytes.len() as u64).to_le_bytes());
        self.inner.update(bytes);
    }

    #[must_use]
    pub fn finalize(self) -> sha2::digest::Output<Sha256> {
        self.inner.finalize()
    }
}

impl Default for SegmentHasher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Absolute-digest tests. If these ever fail, the segment-framing scheme
    // has changed and the cache keys produced elsewhere in this crate are no
    // longer compatible with anything previously written.

    fn hex(digest: sha2::digest::Output<Sha256>) -> String {
        ::hex::encode(digest)
    }

    #[test]
    fn empty_digest() {
        // SHA-256 of the empty byte stream.
        assert_eq!(
            hex(SegmentHasher::new().finalize()),
            "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
        );
    }

    #[test]
    fn single_empty_push() {
        // Hasher absorbs only the 8-byte zero length prefix.
        let mut hasher = SegmentHasher::new();
        hasher.push(b"");
        assert_eq!(
            hex(hasher.finalize()),
            "af5570f5a1810b7af78caf4bc70a660f0df51e42baf91d4de5b2328de0e83dfc",
        );
    }

    #[test]
    fn single_hello_push() {
        // Hasher absorbs `05 00 00 00 00 00 00 00 68 65 6c 6c 6f`.
        let mut hasher = SegmentHasher::new();
        hasher.push(b"hello");
        assert_eq!(
            hex(hasher.finalize()),
            "fe745503750fdbf3e6ef676d16d85ee0d63626c594222f7e991908bdffef7ac9",
        );
    }

    #[test]
    fn two_pushes_ab_c() {
        let mut hasher = SegmentHasher::new();
        hasher.push(b"ab");
        hasher.push(b"c");
        assert_eq!(
            hex(hasher.finalize()),
            "43ee655579de01ca739b3f95c1c2d3f46d353b2c0df818064ea594506cdb2617",
        );
    }

    #[test]
    fn two_pushes_a_bc() {
        // Different segment boundaries from two_pushes_ab_c — must produce a
        // different digest (this is the core collision-resistance property).
        let mut hasher = SegmentHasher::new();
        hasher.push(b"a");
        hasher.push(b"bc");
        assert_eq!(
            hex(hasher.finalize()),
            "9a8acca1b6c6c0befd3fbc756aed625da998c998f7252e738c4ef061906b9b21",
        );
    }
}
