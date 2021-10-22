use std::{convert::TryInto, io::Read};

use byteorder::{LittleEndian, ReadBytesExt};

use super::Chunk;


#[derive(Debug)]
pub struct Audo {
    pub sounds: Vec<Vec<u8>>,
}

impl Chunk for Audo {
    fn parse(buf: &mut std::io::Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized {
        let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
        let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
        let mut sounds = Vec::new();
        for addr in entries_addrs {
            buf.set_position(addr.try_into()?);

            let length = buf.read_u32::<LittleEndian>()?;

            let mut bytes = vec![0_u8; length.try_into()?];
            buf.read_exact(&mut bytes)?;

            assert!(bytes.len() == length.try_into()?);

            sounds.push(bytes);
        }

        Ok(Audo {
            sounds,
        })
    }

    fn get_id() -> [u8; 4] {
        *b"AUDO"
    }
}