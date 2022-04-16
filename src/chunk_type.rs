#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ChunkType([u8; 4]);

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::error::Error;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChunkTypeError {
    // Unknown,
    ByteOutOfRange,
    BadLen,
}

impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            // ChunkTypeError::Unknown => write!(f, "Some error happened!"),
            ChunkTypeError::ByteOutOfRange => write!(f, "Out of range byte encountered!"),
            ChunkTypeError::BadLen => write!(f, "Too few bytes to construct a Chunk Type"),
        }
    }
}

impl Error for ChunkTypeError {}

impl ChunkType {
    const PROPERTY_BIT_MASK: u8 = 32u8;

    pub fn bytes(&self) -> [u8; 4] {
        self.0.clone()
    }

    pub fn is_valid(&self) -> bool {
        self.is_reserved_bit_valid()
    }

    pub fn is_critical(&self) -> bool {
        self.0[0] & Self::PROPERTY_BIT_MASK == 0
    }

    pub fn is_public(&self) -> bool {
        self.0[1] & Self::PROPERTY_BIT_MASK == 0
    }

    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2] & Self::PROPERTY_BIT_MASK == 0
    }

    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3] & Self::PROPERTY_BIT_MASK != 0
    }
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = ChunkTypeError;
    fn try_from(v: [u8; 4]) -> Result<Self, Self::Error> {
        for b in v {
            if b < 65 || (b > 90 && b < 97) || b > 122 {
                return Err(ChunkTypeError::ByteOutOfRange);
            }
        }
        Ok(Self(v))
    }
}

impl TryFrom<&[u8]> for ChunkType {
    type Error = ChunkTypeError;
    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() != 4 {
            return Err(ChunkTypeError::BadLen);
        }

        for b in v {
            if *b < 65 || (*b > 90 && *b < 97) || *b > 122 {
                return Err(ChunkTypeError::ByteOutOfRange);
            }
        }
        Ok(Self([v[0], v[1], v[2], v[3]]))
    }
}


impl FromStr for ChunkType {
    type Err = ChunkTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.as_bytes().try_into()
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", std::str::from_utf8(&self.0).unwrap())
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