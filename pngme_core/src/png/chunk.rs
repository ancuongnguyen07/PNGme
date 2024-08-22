use crate::{Error, Result};
use std::fmt::Display;
use std::io::{BufReader, Read};

use crate::png::ChunkType;
use crate::png::DisplayableVec;
use crc::{Crc, CRC_32_ISO_HDLC};

#[derive(Clone)]
pub struct Chunk {
    chunk_type: ChunkType,
    chunk_data: DisplayableVec,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: &[u8]) -> Chunk {
        let chunk_data = DisplayableVec::new(data);

        let crc = compute_crc(&chunk_type.as_bytes(), &chunk_data.0);

        Self {
            chunk_type,
            chunk_data,
            crc,
        }
    }

    /// The length of the data portion of this chunk.
    pub fn length(&self) -> u32 {
        self.data().len() as u32
    }

    /// The `ChunkType` of this chunk
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    /// The raw data contained in this chunk in bytes
    pub fn data(&self) -> &[u8] {
        &self.chunk_data.0
    }

    /// The CRC of this chunk
    pub fn crc(&self) -> u32 {
        self.crc
    }

    /// Prepends the given tag to the current chunk data
    pub fn prepend(&mut self, tag: &[u8]) -> Result<()> {
        self.chunk_data.0 = [tag, &self.chunk_data.0].concat();
        // recompute CRC as the chunk data itself has changed
        self.crc = compute_crc(&self.chunk_type.as_bytes(), &self.chunk_data.0);
        Ok(())
    }

    /// Returns the data stored in this chunk as a `String`. This function will return an error
    /// if the stored data is not valid UTF-8.
    pub fn data_as_string(&self) -> Result<String> {
        let s = String::from_utf8_lossy(&self.chunk_data.0).to_string();
        Ok(s)
    }

    /// Returns this chunk as a byte sequences described by the PNG spec.
    /// The following data is included in this byte sequence in order:
    /// 1. Length of the data **(4 bytes)**
    /// 2. Chunk type **(4 bytes)**
    /// 3. The data itself **(`length` bytes)**
    /// 4. The CRC of the chunk type and data **(4 bytes)**
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.as_bytes().iter())
            .chain(self.chunk_data.0.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

/// Compute CRC from Chunk Type and Chunk Data
fn compute_crc(chunk_type: &[u8], chunk_data: &[u8]) -> u32 {
    let crc = Crc::<u32>::new(&CRC_32_ISO_HDLC);
    let bytes_vec: Vec<u8> = chunk_type
        .iter()
        .chain(chunk_data.iter())
        .copied()
        .collect();
    crc.checksum(&bytes_vec)
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> std::result::Result<Self, Self::Error> {
        let data: Vec<u8> = value.to_vec();
        if data.len() < 4 {
            return Err(Error::InvalidLength(
                "Data is less than 4 bytes".to_string(),
            ));
        }

        // first 4 bytes for the length
        // the second 4 bytes for the chunk type
        // the following for chunk data
        // the last 4 bytes for CRC
        let mut reader = BufReader::new(value);

        let mut length_bytes: [u8; 4] = [0; 4];
        reader
            .read_exact(&mut length_bytes)
            .map_err(|e| Error::BufferReaderErr(e))?;
        let length = u32::from_be_bytes(length_bytes);

        let chunk_type_slice = value.get(4..8).expect("invalid length byte");
        let chunk_type = ChunkType::try_from(chunk_type_slice)?;

        let data_crc_border: usize = (8 + length) as usize;
        let chunk_data_slice = value.get(8..data_crc_border).expect("invalid length byte");
        let chunk_data = DisplayableVec::new(chunk_data_slice);

        let crc = value
            .get(data_crc_border..(data_crc_border + 4) as usize)
            .expect("invalid length byte");
        let mut crc_byte: [u8; 4] = [0; 4];
        crc_byte.copy_from_slice(crc);
        let crc: u32 = u32::from_be_bytes(crc_byte);

        if crc != compute_crc(chunk_type_slice, chunk_data_slice) {
            return Err(Error::InvalidCRC);
        }

        Ok(Self {
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes = DisplayableVec(self.as_bytes());
        write!(f, "{}", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::png::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, &data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let chunk = testing_chunk();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    fn test_chunk_trait_impls() {
        let chunk: Chunk = testing_chunk();
        let _chunk_string = format!("{}", chunk);
    }

    #[test]
    fn test_prepend_chunk_data() -> Result<()> {
        let mut chunk = testing_chunk();
        let tag = "MyTag".as_bytes();
        let new_chunk_data = "MyTagThis is where your secret message will be!".as_bytes();
        let new_crc = compute_crc(&chunk.chunk_type().as_bytes(), &new_chunk_data);
        assert_ne!(new_crc, chunk.crc());

        chunk.prepend(tag)?;
        assert_eq!(new_crc, chunk.crc());
        assert!(chunk.data().starts_with(tag));
        Ok(())
    }
}
