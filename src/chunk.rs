
use std::{fmt::{Formatter, Display}, string::FromUtf8Error, error::Error};
use crc::{Crc, CRC_32_ISO_HDLC};

use crate::chunk_type::{ChunkType, ChunkTypeError};
const CRC: Crc<u32> = Crc::<u32>::new(&CRC_32_ISO_HDLC);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ChunkError {
    BadLen,
    BadDataLen,
    ChunkType(ChunkTypeError),
    BadCrc,
    Utf8(FromUtf8Error),
}

impl Display for ChunkError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::BadLen => write!(f, "Too few bytes to parse as a chunk"),
            ChunkError::BadDataLen => write!(f, "Data length does not match header"),
            ChunkError::BadCrc => write!(f, "CRC mismatch"),
            ChunkError::ChunkType(c) => {
                write!(f, "ChunkTypeError: ")?;
                c.fmt(f)
            },
            ChunkError::Utf8(e) => {
                write!(f, "Error parsing data as utf-8: ")?;
                e.fmt(f)
            }
        }
    }
}

impl Error for ChunkError {}

impl Chunk {
    pub const LENGTH_FIELD_BYTES: usize = 4;
    pub const CHUNK_TYPE_FIELD_BYTES: usize = 4;
    pub const CRC_FIELD_BYTES: usize = 4;
    pub const NON_DATA_FIELDS_COMBINED_BYTES: usize = Self::LENGTH_FIELD_BYTES
        + Self::CHUNK_TYPE_FIELD_BYTES
        + Self::CRC_FIELD_BYTES;

    fn crc_digest(chunk_type_slice: &[u8], data_slice: &[u8]) -> u32 {
        let mut d = CRC.digest();
        d.update(chunk_type_slice);
        d.update(data_slice);
        d.finalize()
    }

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let crc = Self::crc_digest(&chunk_type.bytes(), data.as_ref());
        Self {
            length: data.len() as u32,
            chunk_type,
            data: data.to_vec(),
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
        self.data.as_ref()
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String, ChunkError> {
        String::from_utf8(self.data().to_vec()).map_err(|e| ChunkError::Utf8(e))
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.length.to_be_bytes().iter()
        .chain(self.chunk_type.bytes().iter())
        .chain(self.data.iter())
        .chain(self.crc.to_be_bytes().iter())
        .copied()
        .collect()
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = ChunkError;
    fn try_from(v: &[u8]) -> Result<Self, Self::Error> {
        if v.len() < Self::NON_DATA_FIELDS_COMBINED_BYTES {
            return Err(ChunkError::BadLen);
        }
        let length = u32::from_be_bytes(v[..Self::LENGTH_FIELD_BYTES].try_into().unwrap());
        if length as usize != v.len() - Self::NON_DATA_FIELDS_COMBINED_BYTES {
            return Err(ChunkError::BadDataLen);
        }
        let chunk_type_slice = &v[Self::LENGTH_FIELD_BYTES..Self::LENGTH_FIELD_BYTES + Self::CHUNK_TYPE_FIELD_BYTES];
        let data_slice = &v[Self::LENGTH_FIELD_BYTES + Self::CHUNK_TYPE_FIELD_BYTES .. v.len() - Self::CRC_FIELD_BYTES];
        let crc_slice = &v[v.len() - Self::CRC_FIELD_BYTES ..];
        let chunk_type: ChunkType = chunk_type_slice.try_into().map_err(|e| ChunkError::ChunkType(e))?;
        let crc_calculated = Self::crc_digest(chunk_type_slice, data_slice);
        let crc = u32::from_be_bytes(crc_slice.try_into().unwrap());
        if crc != crc_calculated {
            return Err(ChunkError::BadCrc);
        }
        Ok(Self {
            length,
            chunk_type,
            data: data_slice.to_vec(),
            crc,
        })
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Length: {}, Type: {}, Data: {:x?}, CRC: {:x?}",
            self.length,
            self.chunk_type,
            self.data,
            self.crc,
        )   
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
        let data = "This is where your secret message will be!".as_bytes().to_vec();
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
        assert_eq!(chunk.as_bytes(), chunk_data);
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