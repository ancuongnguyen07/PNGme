use crate::{chunk_type, Error, Result};
use std::{fmt::Display, io::ErrorKind};

use crate::chunk_type::ChunkType;
use crc::{Crc, CRC_32_ISO_HDLC};

pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    chunk_data: DisplayableVec,
    crc: u32,
}

pub struct DisplayableVec(Vec<u8>);

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;

        let chunk_data: Vec<u8> = data.to_vec();
        let chunk_data: DisplayableVec = DisplayableVec(chunk_data);

        let crc = compute_crc(&chunk_type.bytes(), &chunk_data.0);

        Self {
            length,
            chunk_type,
            chunk_data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.chunk_data.0
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let s = String::from_utf8(self.chunk_data.0.clone())?;
        Ok(s)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length
            .to_be_bytes()
            .iter()
            .chain(self.chunk_type.bytes().iter())
            .chain(self.chunk_data.0.iter())
            .chain(self.crc.to_be_bytes().iter())
            .copied()
            .collect()
    }
}

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
            return Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidInput,
                "invalid length of chunk, must greater 4 bytes",
            )));
        }

        // first 4 bytes for the length
        // the second 4 bytes for the chunk type
        // the following for chunk data
        // the last 4 bytes for CRC
        let length_byte = value.get(..4).expect("invalid length byte");
        let mut length_slice: [u8; 4] = [0; 4];
        length_slice.copy_from_slice(length_byte);
        let length = u32::from_be_bytes(length_slice);

        let chunk_type_slice = value.get(4..8).expect("invalid length byte");
        let chunk_type = ChunkType::try_from(chunk_type_slice)?;

        let data_crc_border: usize = (8 + length) as usize;
        let chunk_data_slice = value.get(8..data_crc_border).expect("invalid length byte");
        let chunk_data = DisplayableVec(chunk_data_slice.to_vec());

        let crc = value
            .get(data_crc_border..(data_crc_border + 4) as usize)
            .expect("invalid length byte");
        let mut crc_byte: [u8; 4] = [0; 4];
        crc_byte.copy_from_slice(crc);
        let crc: u32 = u32::from_be_bytes(crc_byte);

        if crc != compute_crc(chunk_type_slice, chunk_data_slice) {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::InvalidInput,
                "Invalid CRC",
            )));
        }

        Ok(Self {
            length,
            chunk_type,
            chunk_data,
            crc,
        })
    }
}

impl Display for DisplayableVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for bytes in self.0.chunks(4) {
            let s: Vec<String> = bytes.iter().map(|&b| b.to_string()).collect();
            let s = s.join(" ");

            writeln!(f, "{}", s)?;
        }
        Ok(())
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bytes: DisplayableVec = DisplayableVec(self.as_bytes());
        write!(f, "{}", bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
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
        let chunk = Chunk::new(chunk_type, data);
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

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

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
    pub fn test_chunk_trait_impls() {
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

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }
}
