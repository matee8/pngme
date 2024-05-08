use crate::Error;
use std::{
    convert::TryFrom,
    error,
    fmt::{self, Display, Formatter},
    str::{self, FromStr},
};

#[derive(PartialEq, Eq, Debug)]
pub struct ChunkType {
    value: [u8; 4],
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.value
    }
    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool {
        (self.value[0] >> 5) & 1 == 0
    }
    pub fn is_public(&self) -> bool {
        (self.value[1] >> 5) & 1 == 0
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        (self.value[2] >> 5) & 1 == 0
    }
    pub fn is_safe_to_copy(&self) -> bool {
        (self.value[3] >> 5) & 1 == 1
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(value: [u8; 4]) -> Result<Self, Self::Error> {
        let valid_chars: bool = value.iter().all(|&b| b.is_ascii_alphabetic());

        if !valid_chars {
            return Err(Box::new(ChunkTypeError::InvalidCharError));
        }

        Ok(ChunkType { value })
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.len() != 4 {
            return Err(Box::new(ChunkTypeError::ByteLengthError));
        }

        let valid_chars: bool = value
            .bytes()
            .all(|b| (65..=90).contains(&b) || (97..=122).contains(&b));

        if !valid_chars {
            return Err(Box::new(ChunkTypeError::InvalidCharError));
        }

        Ok(ChunkType {
            value: value.as_bytes().try_into()?,
        })
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", str::from_utf8(&self.value).unwrap())
    }
}

#[derive(Debug)]
pub enum ChunkTypeError {
    ByteLengthError,
    InvalidCharError,
}

impl error::Error for ChunkTypeError {}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            ChunkTypeError::ByteLengthError => {
                write!(f, "Invalid length of chunk type!")
            }
            ChunkTypeError::InvalidCharError => {
                write!(f, "Invalid character in chunk type!")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
