use std::str::FromStr;
use std::convert::{TryFrom,TryInto};
use std::fmt;

#[derive(Debug,Eq,PartialEq)]
pub struct ChunkType {
	ancillary: u8,
	private: u8,
	reserved: u8,
	copy_safe: u8,
}

impl ChunkType {
	pub fn bytes(&self) -> [u8;4] {
		[self.ancillary,self.private,self.reserved,self.copy_safe]
	}

	// 0 (uppercase) = critical, 1 (lowercase) = ancillary. 
	pub fn is_critical(&self) -> bool {
		self.ancillary & (1<<5) == 0
	}

	// 0 (uppercase) = public, 1 (lowercase) = private. 
	pub fn is_public(&self) -> bool {
		self.private & (1<<5) == 0
	}

	// Reserved bit must be 0 (uppercase)
	pub fn is_valid(&self) -> bool {
		self.reserved & (1<<5) == 0
	}

	// Reserved bit must be set to 0 (uppercase)
	pub fn is_reserved_bit_valid(&self) -> bool {
		self.reserved & (1<<5) == 0
	}

	// 0 (uppercase) = unsafe to copy, 1 (lowercase) = safe to copy. 
	pub fn is_safe_to_copy(&self) -> bool {
		self.copy_safe & (1<<5) != 0
	} 
}

impl TryFrom<[u8;4]> for ChunkType {
	type Error = &'static str;

	fn try_from(type_code: [u8;4]) -> Result<Self, Self::Error> {
		if !type_code.iter().all(|n| n.is_ascii_alphabetic()){
            Err("Chunk type code must only have ascii alphabetic values! Ranges [65-90] or [97-122]")
        } else {
			Ok(ChunkType{
				ancillary: type_code[0],
				private: type_code[1],
				reserved: type_code[2],
				copy_safe: type_code[3]
			})
        }
    }
}

impl FromStr for ChunkType {
	type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.len() != 4 || !s.chars().all(|c| c.is_ascii_alphabetic()) {
			Err("Chunk type code must be an ascii string of length 4 exclusively")
		} else {
			let type_code: [u8;4] = s.as_bytes().try_into().unwrap();
			Ok(ChunkType{
				ancillary: type_code[0],
				private: type_code[1],
				reserved: type_code[2],
				copy_safe: type_code[3]
			})
		}
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let type_code = [self.ancillary,self.private,self.reserved,self.copy_safe];
        write!(f, "{}", std::str::from_utf8(&type_code).unwrap())
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