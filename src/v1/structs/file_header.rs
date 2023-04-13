use crate::util::{AsBytes, FileHeaderTrait, u16_from_le_bytes, u32_from_le_bytes, u8_from_le_bytes};

#[derive(Clone, Default)]
pub struct FileHeader {
    /// File storage method
    ///
    /// V1: `5` if zlib, `4` if store
    ///
    /// V1a: `1` if zlib, `0` if store
    pub method: u8,

    /// How many additional copies of this specific file exist
    ///
    /// Offsets of additional copies are stored after the file name
    pub file_copies: u8,

    /// How many additional copies of this specific file exist - alternative, probably unused.
    ///
    /// Offsets of additional copies are stored after the file name
    pub file_copies_a: u16,

    /// Offset at which the actual file data is found
    ///
    /// Offset from `0h`
    pub data_offset: u32,

    /// Unpacked size of the file, in bytes
    pub unpacked_size: u32,

    /// Packed size of the file, in bytes
    pub packed_size: u32,

    /// CRC-32/JAMCRC of the compressed data
    pub crc32: u32,

    /// Position of the folder name in decoded Vec with the filename data
    pub file_name_length: u16,

    /// Offsets of additional copies
    pub file_copies_offsets: Vec<u32>,
}

impl AsBytes for FileHeader {
    const BYTE_COUNT: usize = 22;

    fn from_bytes(bytes: Vec<u8>) -> Self {
        let mut bytes = bytes;
        Self {
            method: u8_from_le_bytes(&mut bytes),
            file_copies: u8_from_le_bytes(&mut bytes),
            file_copies_a: u16_from_le_bytes(&mut bytes),
            data_offset: u32_from_le_bytes(&mut bytes),
            unpacked_size: u32_from_le_bytes(&mut bytes),
            packed_size: u32_from_le_bytes(&mut bytes),
            crc32: u32_from_le_bytes(&mut bytes),
            file_name_length: u16_from_le_bytes(&mut bytes),
            file_copies_offsets: vec![],
        }
    }

    fn to_bytes(self) -> Vec<u8> {
        let mut result = Vec::new();
        result.extend_from_slice(&self.method.to_le_bytes());
        result.extend_from_slice(&self.file_copies.to_le_bytes());
        result.extend_from_slice(&self.file_copies_a.to_le_bytes());
        result.extend_from_slice(&self.data_offset.to_le_bytes());
        result.extend_from_slice(&self.unpacked_size.to_le_bytes());
        result.extend_from_slice(&self.packed_size.to_le_bytes());
        result.extend_from_slice(&self.crc32.to_le_bytes());
        result.extend_from_slice(&self.file_name_length.to_le_bytes());
        for file_copies_offset in self.file_copies_offsets {
            result.extend_from_slice(&file_copies_offset.to_le_bytes());
        }
        result
    }
}

impl FileHeaderTrait for FileHeader {
    fn get_method(&self) -> u8 {
        self.method
    }

    fn get_data_offset(&self) -> u32 {
        self.data_offset
    }

    fn get_unpacked_size(&self) -> u32 {
        self.unpacked_size
    }

    fn get_packed_size(&self) -> u32 {
        self.packed_size
    }

    fn get_file_copies_offsets(&self) -> Vec<u32> {
        self.file_copies_offsets.clone()
    }

    fn get_file_copies_num(&self) -> (u8, u16) {
        (self.file_copies, self.file_copies_a)
    }

    fn is_compressed(&self) -> bool { self.unpacked_size != self.packed_size }
}