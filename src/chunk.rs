use std::convert::{TryFrom,TryInto};
use std::fmt;

use crate::chunk_type::ChunkType;
use crate::{Error,Result};

pub struct Chunk {
	len: u32,
	chunk_type: ChunkType,
	data: Vec<u8>,
	crc: u32
}

impl Chunk {
    pub fn new(chunk_type: ChunkType,data: Vec<u8>) -> Chunk {
        let checksum_bytes: Vec<u8> = chunk_type.bytes().iter()
            .chain(data.iter())
            .copied()
            .collect();

        let crc = crc::crc32::checksum_ieee(&checksum_bytes);
        Chunk{
            len: data.len() as u32,
            chunk_type,
            data,
            crc
        }
    }

    pub fn length(&self) -> u32 {
        self.len
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    
    pub fn crc(&self) -> u32 {
        self.crc
    }
    
    pub fn data_as_string(&self) -> Result<String> {
        match String::from_utf8(self.data.clone()) {
            Err(_) => Err(Error::from("Could not convert data into string")),
            Ok(data_string) => Ok(data_string)
        }
    }
    
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(12+self.len as usize);
        bytes.extend(self.len.to_be_bytes().iter());
        bytes.extend(self.chunk_type.bytes().clone().iter());
        bytes.extend(self.data.iter());
        bytes.extend(self.crc.to_be_bytes().iter());
        
        bytes
    }
}


impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(chunk: &[u8]) -> Result<Self> {
        if chunk.len() < 8 {
            return Err(Error::from("Validated chunk must be at least of length 8!"));
        }

        let len: [u8;4] = chunk[..4].try_into().unwrap();
        let len = u32::from_be_bytes(len);
        
        let chunk_type: [u8;4] = chunk[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type)?;

        let data: Vec<u8> = if len > 0 {
            Vec::from(&chunk[8..(8+len as usize)])
        } else {
            Vec::new()
        };

        let crc = chunk[(8+len as usize)..(12+len as usize)].try_into().unwrap();
        let crc = u32::from_be_bytes(crc);

        let checksum_bytes: Vec<u8> = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect();

        let calculated_crc: Vec<u8> = chunk_type.bytes().iter().chain(data.iter()).copied().collect();
        let calculated_crc = crc::crc32::checksum_ieee(&calculated_crc);
        if crc != calculated_crc {
            let error_str = format!("Invalid chunk crc! {} vs {}",crc,calculated_crc);
            return Err(Error::from(error_str));
        }

        Ok(Chunk{
            len,
            chunk_type,
            data,
            crc
        })
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Chunk {} with len {}\n{:?}", self.chunk_type, self.len, self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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