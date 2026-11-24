//! `SGD5`: the engine's shader file. Each record holds one shader container,
//! deflate-packed. The header tables are not decoded yet, so records are
//! found by their marker and cross-checked by their sizes:
//!
//! ```text
//! u8 name length (with NUL), name, NUL, u8 kind,
//! u32 n, u32 0x000DDCE7, u32 n, u32 unpacked size, raw deflate (n - 12 bytes)
//! ```

pub const MAGIC: &[u8; 4] = b"SGD5";
const MARKER: [u8; 4] = 0x000D_DCE7_u32.to_le_bytes();
/// Containers are at most a few MiB; refuse to allocate for absurd sizes.
const MAX_UNPACKED: usize = 64 << 20;
/// A DXBC header: magic, 16-byte digest, version, total size, part count.
const DXBC_HEADER: usize = 0x20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Record<'a> {
    /// Offset of the marker in the file.
    pub offset: usize,
    /// Like `PS_<name>_<hash>_Wrapped`; `None` if the bytes before the
    /// record do not read as one.
    pub name: Option<&'a str>,
    /// Record type, meaning unknown (0x32 most shaders, 0x41 mesh shaders
    /// and some material families, 0x3c compute, 0x40, 0x42).
    pub kind: u8,
    packed: &'a [u8],
    unpacked_size: usize,
}

impl Record<'_> {
    /// The container, or `None` if the stream is corrupt.
    pub fn unpack(&self) -> Option<Vec<u8>> {
        miniz_oxide::inflate::decompress_to_vec_with_limit(self.packed, self.unpacked_size)
            .ok()
            .filter(|out| out.len() == self.unpacked_size)
    }

    /// The unpacked bytes when they are a structurally valid DXBC shader
    /// container. `unpack` only checks the deflate stream; a record can hold
    /// anything, so callers that need a container should use this instead of
    /// repeating the header checks.
    pub fn container(&self) -> Option<Vec<u8>> {
        let bytes = self.unpack()?;
        if !bytes.starts_with(b"DXBC") || bytes.len() < DXBC_HEADER {
            return None;
        }
        let u32_at = |at: usize| {
            bytes
                .get(at..at + 4)
                .and_then(|b| b.try_into().ok())
                .map(u32::from_le_bytes)
        };
        let total = u32_at(0x18)? as usize;
        let parts = u32_at(0x1c)? as usize;
        if total < DXBC_HEADER || total > bytes.len() || parts > (total - DXBC_HEADER) / 4 {
            return None;
        }
        Some(bytes)
    }

    /// The 16-byte DXBC container digest from the header, when the record
    /// holds a container. This is the hash used in shader names and cache
    /// keys, so callers can identify a shader without unpacking it twice.
    pub fn digest(&self) -> Option<[u8; 16]> {
        self.container()?.get(4..20).and_then(|d| d.try_into().ok())
    }
}

/// An SGD5 file, or a file holding one (an archive: records are found by
/// their marker, so no index is needed).
pub fn is_sgd5(file: &[u8]) -> bool {
    memchr::memmem::find(file, MAGIC).is_some()
}

/// Every record in an SGD5 file, in file order.
pub fn records(file: &[u8]) -> impl Iterator<Item = Record<'_>> {
    memchr::memmem::find_iter(file, &MARKER).filter_map(|at| record(file, at))
}

fn record(file: &[u8], at: usize) -> Option<Record<'_>> {
    let u32_at = |o: usize| {
        file.get(o..o + 4)
            .map(|b| u32::from_le_bytes(b.try_into().expect("4 bytes")) as usize)
    };
    let n = u32_at(at.checked_sub(4)?)?;
    let unpacked_size = u32_at(at + 8)?;
    if u32_at(at + 4)? != n || n < 12 || unpacked_size > MAX_UNPACKED {
        return None;
    }
    let nul = at.checked_sub(6)?;
    Some(Record {
        offset: at,
        name: (file[nul] == 0).then(|| name_before(file, nul)).flatten(),
        kind: file[at - 5],
        packed: file.get(at + 12..at + n)?,
        unpacked_size,
    })
}

/// The printable name that ends at `nul` and follows its length byte.
fn name_before(file: &[u8], nul: usize) -> Option<&str> {
    let run = file[..nul]
        .iter()
        .rposition(|b| !b.is_ascii_graphic())
        .map_or(0, |p| p + 1);
    // From 32 characters on, the length byte is printable too.
    (run.saturating_sub(1)..nul)
        .find(|&l| usize::from(file[l]) == nul - l)
        .and_then(|l| std::str::from_utf8(&file[l + 1..nul]).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record_bytes(name: &str, kind: u8, payload: &[u8]) -> Vec<u8> {
        let packed = miniz_oxide::deflate::compress_to_vec(payload, 6);
        let n = (packed.len() + 12) as u32;
        let mut out = vec![0xF4, 0x01, 0, 0, name.len() as u8 + 1];
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(&[0, kind]);
        for v in [n, 0x000D_DCE7, n, payload.len() as u32] {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out.extend_from_slice(&packed);
        out
    }

    #[test]
    fn reads_records() {
        let long = format!("PS_{}_Wrapped", "x".repeat(40));
        let mut file = MAGIC.to_vec();
        file.extend_from_slice(&[1, 2, 3, 4]);
        file.extend(record_bytes(
            "VS_Blit_0011aabb_Wrapped",
            0x32,
            b"first container",
        ));
        // A marker whose sizes disagree is not a record.
        file.extend_from_slice(&[9, 0, 0, 0, 0xE7, 0xDC, 0x0D, 0, 8, 0, 0, 0]);
        file.extend(record_bytes(&long, 0x41, &[7; 300]));

        assert!(is_sgd5(&file));
        let found: Vec<_> = records(&file).collect();
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].name, Some("VS_Blit_0011aabb_Wrapped"));
        assert_eq!(found[0].kind, 0x32);
        assert_eq!(found[0].unpack().as_deref(), Some(&b"first container"[..]));
        assert_eq!(found[1].name, Some(long.as_str()));
        assert_eq!(found[1].unpack(), Some(vec![7; 300]));

        // The same file inside an archive.
        let mut archive = vec![0x5A; 0x1000];
        archive.extend_from_slice(&file);
        assert!(is_sgd5(&archive));
        assert_eq!(records(&archive).count(), 2);
        assert!(!is_sgd5(&archive[..0x1000]));
    }

    #[test]
    fn corrupt_stream_does_not_unpack() {
        let mut file = record_bytes("CS_x", 0x3c, &[1; 64]);
        file[27] = 0xFF; // first packed byte: reserved block type
        let r = records(&file).next().unwrap();
        assert_eq!(r.unpack(), None);
    }

    /// A container with no parts, just the header. The digest is arbitrary
    /// synthetic data, not taken from any title.
    fn dxbc_bytes(digest: [u8; 16], parts: u32) -> Vec<u8> {
        let mut out = b"DXBC".to_vec();
        out.extend(digest);
        out.extend(0u32.to_le_bytes()); // version
        out.extend(((DXBC_HEADER + 4 * parts as usize) as u32).to_le_bytes());
        out.extend(parts.to_le_bytes());
        out.resize(DXBC_HEADER + 4 * parts as usize, 0);
        out
    }

    #[test]
    fn records_expose_validated_containers_and_digests() {
        let digest = [0x5a; 16];
        let payload = dxbc_bytes(digest, 2);
        let file = record_bytes("VS_x_5a_Wrapped", 0x32, &payload);
        let r = records(&file).next().unwrap();
        assert_eq!(r.unpack(), Some(payload.clone()));
        assert_eq!(r.container(), Some(payload));
        assert_eq!(r.digest(), Some(digest));

        // A deflate stream can hold any bytes; those are not containers.
        let file = record_bytes("other", 0x32, b"not a shader");
        let r = records(&file).next().unwrap();
        assert_eq!(r.unpack(), Some(b"not a shader".to_vec()));
        assert_eq!(r.container(), None);
        assert_eq!(r.digest(), None);
    }

    #[test]
    fn malformed_dxbc_headers_are_not_containers() {
        let digest = [1; 16];
        let good = dxbc_bytes(digest, 1);

        let mut bad_magic = good.clone();
        bad_magic[0] = b'X';
        let mut bad_total = good.clone();
        bad_total[0x18..0x1c].copy_from_slice(&((good.len() + 1) as u32).to_le_bytes());
        let mut short_total = good.clone();
        short_total[0x18..0x1c].copy_from_slice(&((DXBC_HEADER - 1) as u32).to_le_bytes());
        let mut bad_parts = good.clone();
        bad_parts[0x1c..0x20].copy_from_slice(&u32::MAX.to_le_bytes());

        for payload in [bad_magic, bad_total, short_total, bad_parts] {
            let file = record_bytes("VS_x", 0x32, &payload);
            let r = records(&file).next().unwrap();
            assert_eq!(r.unpack(), Some(payload), "deflate stream is still valid");
            assert_eq!(r.container(), None);
            assert_eq!(r.digest(), None);
        }

        // Truncated header.
        let short = dxbc_bytes(digest, 0)[..DXBC_HEADER - 1].to_vec();
        let file = record_bytes("VS_x", 0x32, &short);
        let r = records(&file).next().unwrap();
        assert_eq!(r.container(), None);
    }
}
