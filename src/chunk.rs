use crate::{self, chunk_type::ChunkType, Error};
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
    const LENGTH_SIZE: usize = 4;
    const TYPE_SIZE: usize = 4;
    const CRC_SIZE: usize = 4;
    const METADATA_SIZE: usize = Chunk::LENGTH_SIZE + Chunk::TYPE_SIZE 
        + Chunk::CRC_SIZE;

    fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk { chunk_type, data }
    }

    fn length(&self) -> u32 {
        self.data.len() as u32
    }

    fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    fn data(&self) -> &[u8] {
        &self.data
    }

    fn crc(&self) -> u32 {
        let bytes: Vec<u8> = self
            .data
            .iter()
            .cloned()
            .chain(self.chunk_type.bytes().iter().cloned())
            .collect();

        const CRC_PNG: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

        CRC_PNG.checksum(&bytes)
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < Chunk::METADATA_SIZE {
            return Err(Box::new(ChunkError::LengthError));
        }

        let length: u32 = u32::from_be_bytes(
            value[..Chunk::LENGTH_SIZE].try_into()?
        );

        let chunk_type_bytes: [u8; 4] =
            value[Chunk::LENGTH_SIZE..Chunk::LENGTH_SIZE + Chunk::TYPE_SIZE]
            .try_into()?;

        let chunk_type: ChunkType = ChunkType::try_from(chunk_type_bytes)?;

        let data: Vec<u8> =
            value[Chunk::LENGTH_SIZE + Chunk::TYPE_SIZE..value.len() - Chunk::CRC_SIZE]
            .to_vec();

        let crc: u32 = 
            u32::from_be_bytes(value[value.len() - Chunk::CRC_SIZE..]
            .try_into()?);

        if data.len() != length as usize {
            return Err(Box::new(ChunkError::DataLengthError));
        }

        let new: Self = Self { chunk_type, data };

        if new.crc() != crc {
            return Err(Box::new(ChunkError::CrcError));
        }

        Ok(new)
    }
}

#[derive(Debug)]
pub enum ChunkError {
    LengthError,
    DataLengthError,
    CrcError,
}

impl error::Error for ChunkError {}

impl Display for ChunkError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ChunkError::LengthError => {
                write!(f, "Invalid length of chunk!")
            }
            ChunkError::DataLengthError => {
                write!(f, "Invalid data length!")
            }
            ChunkError::CrcError => {
                write!(f, "Invalid crc!")
            }
        }
    }
}
