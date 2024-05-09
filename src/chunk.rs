use crate::{chunk_type::ChunkType, Error};
use crc::{Crc, CRC_32_ISO_HDLC};
use std::{
    convert::TryFrom,
    error,
    fmt::{self, Display, Formatter},
};

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl Chunk {
    pub const LENGTH_SIZE: usize = 4;
    pub const TYPE_SIZE: usize = 4;
    pub const CRC_SIZE: usize = 4;
    pub const METADATA_SIZE: usize =
        Chunk::LENGTH_SIZE + Chunk::TYPE_SIZE + Chunk::CRC_SIZE;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk { chunk_type, data }
    }
    pub fn length(&self) -> u32 {
        self.data.len() as u32
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        let bytes: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .cloned()
            .chain(self.data.iter().cloned())
            .collect();

        const CRC_PNG: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        CRC_PNG.checksum(&bytes)
    }
    pub fn data_as_string(&self) -> Result<String, crate::Error> {
        Ok(String::from_utf8(self.data.clone())?)
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        self.length()
            .to_be_bytes()
            .iter()
            .cloned()
            .chain(self.chunk_type.bytes().iter().cloned())
            .chain(self.data.iter().cloned())
            .chain(self.crc().to_be_bytes().iter().cloned())
            .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < Chunk::METADATA_SIZE {
            return Err(Box::new(ChunkError::TooSmall));
        }

        let length: u32 =
            u32::from_be_bytes(value[..Chunk::LENGTH_SIZE].try_into()?);
        let chunk_type_bytes: [u8; 4] = value
            [Chunk::LENGTH_SIZE..Chunk::LENGTH_SIZE + Chunk::TYPE_SIZE]
            .try_into()?;
        let chunk_type: ChunkType = ChunkType::try_from(chunk_type_bytes)?;
        let data: Vec<u8> = value[Chunk::LENGTH_SIZE + Chunk::TYPE_SIZE
            ..value.len() - Chunk::CRC_SIZE]
            .to_vec();
        let crc: u32 = u32::from_be_bytes(
            value[value.len() - Chunk::CRC_SIZE..].try_into()?,
        );

        if data.len() != length as usize {
            return Err(Box::new(ChunkError::ToSmallData));
        }

        let new: Self = Self { chunk_type, data };

        if new.crc() != crc {
            return Err(Box::new(ChunkError::InvalidCrc));
        }

        Ok(new)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.data_as_string().unwrap())
    }
}

#[derive(Debug)]
pub enum ChunkError {
    TooSmall,
    ToSmallData,
    InvalidCrc,
}

impl error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ChunkError::TooSmall => {
                write!(f, "Invalid length of chunk!")
            }
            ChunkError::ToSmallData => {
                write!(f, "Invalid data length!")
            }
            ChunkError::InvalidCrc => {
                write!(f, "Invalid crc!")
            }
        }
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
        let message_bytes =
            "This is where your secret message will be!".as_bytes();
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
        let expected_chunk_string =
            String::from("This is where your secret message will be!");
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
        let message_bytes =
            "This is where your secret message will be!".as_bytes();
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
        let expected_chunk_string =
            String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes =
            "This is where your secret message will be!".as_bytes();
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
        let message_bytes =
            "This is where your secret message will be!".as_bytes();
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
