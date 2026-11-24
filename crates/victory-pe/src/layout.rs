//! Raw byte offsets of PE32+ header fields, for code that patches images.
//!
//! All offsets are relative to the start of the structure named in the
//! constant's prefix. They follow the published PE/COFF specification.

/// Offset of `e_lfanew` (u32) in the DOS header.
pub const DOS_E_LFANEW: usize = 0x3C;

/// Size of the `PE\0\0` signature that precedes the file header.
pub const NT_SIGNATURE_SIZE: usize = 4;
/// Size of `IMAGE_FILE_HEADER`.
pub const FILE_HEADER_SIZE: usize = 20;

/// `IMAGE_FILE_HEADER::SizeOfOptionalHeader` (u16).
const FILE_HEADER_SIZE_OF_OPTIONAL_HEADER: usize = 16;

/// `IMAGE_FILE_HEADER::Characteristics` (u16).
pub const FILE_HEADER_CHARACTERISTICS: usize = 18;

/// `IMAGE_OPTIONAL_HEADER64::AddressOfEntryPoint` (u32).
pub const OPT_ADDRESS_OF_ENTRY_POINT: usize = 16;
/// `IMAGE_OPTIONAL_HEADER64::CheckSum` (u32).
pub const OPT_CHECKSUM: usize = 64;
/// `IMAGE_OPTIONAL_HEADER64::DllCharacteristics` (u16).
pub const OPT_DLL_CHARACTERISTICS: usize = 70;
/// `IMAGE_OPTIONAL_HEADER64::NumberOfRvaAndSizes` (u32).
pub const OPT_NUMBER_OF_RVA_AND_SIZES: usize = 108;
/// First `IMAGE_DATA_DIRECTORY` entry (each is `{ rva: u32, size: u32 }`).
pub const OPT_DATA_DIRECTORY: usize = 112;
/// Size of one `IMAGE_DATA_DIRECTORY`.
pub const DATA_DIRECTORY_SIZE: usize = 8;

/// `IMAGE_TLS_DIRECTORY64::AddressOfCallBacks` (u64 VA).
pub const TLS64_ADDRESS_OF_CALLBACKS: usize = 24;

/// Data directory indices.
pub mod dir {
    pub const IMPORT: usize = 1;
    pub const EXCEPTION: usize = 3;
    pub const BASERELOC: usize = 5;
    pub const LOAD_CONFIG: usize = 10;
    pub const BOUND_IMPORT: usize = 11;
    pub const IAT: usize = 12;
}

/// `IMAGE_FILE_HEADER::Characteristics` flags.
pub mod file_flags {
    pub const RELOCS_STRIPPED: u16 = 0x0001;
    pub const DLL: u16 = 0x2000;
}

/// `IMAGE_OPTIONAL_HEADER64::DllCharacteristics` flags.
pub mod dll_flags {
    pub const HIGH_ENTROPY_VA: u16 = 0x0020;
    pub const DYNAMIC_BASE: u16 = 0x0040;
    pub const FORCE_INTEGRITY: u16 = 0x0080;
    pub const NX_COMPAT: u16 = 0x0100;
    pub const APPCONTAINER: u16 = 0x1000;
    pub const GUARD_CF: u16 = 0x4000;
}

/// `IMAGE_OPTIONAL_HEADER64::Subsystem` values.
pub mod subsystem {
    pub const WINDOWS_GUI: u16 = 2;
    pub const WINDOWS_CUI: u16 = 3;
    pub const XBOX: u16 = 14;
    pub const XBOX_CODE_CATALOG: u16 = 17;
}

/// Offset of the optional header from the start of the file.
pub fn optional_header_offset(data: &[u8]) -> Option<usize> {
    let lfanew = read_u32(data, DOS_E_LFANEW)? as usize;
    lfanew.checked_add(NT_SIGNATURE_SIZE + FILE_HEADER_SIZE)
}

/// Offset of the file header from the start of the file.
pub fn file_header_offset(data: &[u8]) -> Option<usize> {
    let lfanew = read_u32(data, DOS_E_LFANEW)? as usize;
    lfanew.checked_add(NT_SIGNATURE_SIZE)
}

/// Offset of data directory `index` from the start of the file.
pub fn data_directory_offset(data: &[u8], index: usize) -> Option<usize> {
    let file_header = file_header_offset(data)?;
    let size_at = file_header.checked_add(FILE_HEADER_SIZE_OF_OPTIONAL_HEADER)?;
    let size_end = size_at.checked_add(2)?;
    let size = u16::from_le_bytes(data.get(size_at..size_end)?.try_into().ok()?);
    let opt = file_header.checked_add(FILE_HEADER_SIZE)?;
    let header = data.get(opt..opt.checked_add(usize::from(size))?)?;
    let count = read_u32(header, OPT_NUMBER_OF_RVA_AND_SIZES)? as usize;
    if index >= count {
        return None;
    }
    let entry = OPT_DATA_DIRECTORY.checked_add(index.checked_mul(DATA_DIRECTORY_SIZE)?)?;
    header.get(entry..entry.checked_add(DATA_DIRECTORY_SIZE)?)?;
    opt.checked_add(entry)
}

pub fn read_u32(data: &[u8], at: usize) -> Option<u32> {
    let end = at.checked_add(4)?;
    Some(u32::from_le_bytes(data.get(at..end)?.try_into().ok()?))
}

pub fn read_u64(data: &[u8], at: usize) -> Option<u64> {
    let end = at.checked_add(8)?;
    Some(u64::from_le_bytes(data.get(at..end)?.try_into().ok()?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_entries_must_fit_in_the_optional_header_and_file() {
        const FILE_HEADER: usize = DOS_E_LFANEW + 4 + NT_SIGNATURE_SIZE;
        const OPTIONAL_HEADER: usize = FILE_HEADER + FILE_HEADER_SIZE;
        const HEADER_SIZE: usize = OPT_DATA_DIRECTORY + DATA_DIRECTORY_SIZE;
        const ENTRY: usize = OPTIONAL_HEADER + OPT_DATA_DIRECTORY;

        let mut data = [0u8; OPTIONAL_HEADER + HEADER_SIZE];
        let nt_signature = (FILE_HEADER - NT_SIGNATURE_SIZE) as u32;
        data[DOS_E_LFANEW..][..4].copy_from_slice(&nt_signature.to_le_bytes());
        data[FILE_HEADER + FILE_HEADER_SIZE_OF_OPTIONAL_HEADER..][..2]
            .copy_from_slice(&(HEADER_SIZE as u16).to_le_bytes());
        data[OPTIONAL_HEADER + OPT_NUMBER_OF_RVA_AND_SIZES..][..4]
            .copy_from_slice(&1u32.to_le_bytes());

        assert_eq!(data_directory_offset(&data, 0), Some(ENTRY));
        assert_eq!(data_directory_offset(&data, 1), None);
        assert_eq!(data_directory_offset(&data[..data.len() - 1], 0), None);

        data[FILE_HEADER + FILE_HEADER_SIZE_OF_OPTIONAL_HEADER..][..2]
            .copy_from_slice(&((HEADER_SIZE - 1) as u16).to_le_bytes());
        assert_eq!(data_directory_offset(&data, 0), None);
    }

    #[test]
    fn reads_reject_overflowing_offsets() {
        let data = [0u8; 16];
        assert_eq!(read_u32(&data, usize::MAX - 1), None);
        assert_eq!(read_u64(&data, usize::MAX - 2), None);
    }
}
