use crate::{
    Error,
    chunk_type::ChunkType,
};
use std::convert::TryFrom;

pub struct Chunk {
    length: u32,
    r#type: ChunkType,
    data: Vec<u8>,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        
    }
}

pub enum ChunkError {
    InputTooSmall
}
