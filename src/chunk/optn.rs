use byteorder::{LittleEndian, ReadBytesExt};
use tuple_transpose::TupleTranspose;

use super::{Chunk, read_string_ptr};


#[derive(Debug)]
pub struct Optn {
    pub _unknown1: Vec<u32>, // len=2
    pub info: u32, // InfoFlags,
    pub _unknown2: Vec<u32>, // len=0xC
    pub constant_map: Vec<(String, String)>,
}

impl Chunk for Optn {
    fn parse(buf: &mut std::io::Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized {
        let unknown1 = (0..2).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
        let info = buf.read_u32::<LittleEndian>()?; // could parse more: InfoFlags
        let unknown2 = (0..0xC).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
        let constants_addr_ct = buf.read_i32::<LittleEndian>()?;
        let constants_addrs = (0..constants_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
        let constant_map = (0..constants_addrs.len()).map(|_| {
            (read_string_ptr(buf), read_string_ptr(buf)).transpose() // transpose does (Result<>, Result<>) => Result<( , )>
        }).collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

        Ok(Optn {
            _unknown1: unknown1,
            info,
            _unknown2: unknown2,
            constant_map,
        })
    }

    fn get_id() -> [u8; 4] {
        *b"OPTN"
    }
}