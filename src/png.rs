use crate::{
    chunk::Chunk,
    Error,
};
use std::{
    convert::TryFrom,
    error,
    fmt::{self, Display, Formatter},
};

struct Png {
    chunks: Vec<Chunk>,
}

impl Png {
    const STANDARD_HEADER: [u8; 8] = [
        137, 80, 78, 71, 13, 10, 26, 10
    ];
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < Png::STANDARD_HEADER.len() {
            return Err(Box::new(PngError::TooSmall));
        }

        if value[0..Png::STANDARD_HEADER.len()] != Png::STANDARD_HEADER {
            return Err(Box::new(PngError::InvalidHeader));
        }

        let mut chunks: Vec<u8> = Vec::new();
        let mut index: usize = 
    }
}

#[derive(Debug)]
enum PngError {
    TooSmall,
    InvalidHeader,
}

impl error::Error for PngError {}

impl Display for PngError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            PngError::TooSmall => {
                write!(f, "Invalid Png size!")
            }
            PngError::InvalidHeader => {
                write!(f, "Invalid header!")
            }
        }
    }
}
