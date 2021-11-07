use std::{collections::HashMap, convert::TryInto};

use byteorder::{LittleEndian, ReadBytesExt};
use image::DynamicImage;

use super::{Chunk, read_string_ptr};


#[derive(Debug)]
pub struct Bgnd {
    pub backgrounds: HashMap<String, BackgroundEntry>,
}

#[derive(Debug)]
pub struct BackgroundEntry {
    pub _unknown1: Vec<u32>,
    pub texture: BackgroundState,
}

#[derive(Debug)]
pub enum BackgroundState {
    Unloaded {
        texture_address: i32, // addr to TPAG
    },
    Loaded {
        texture: DynamicImage,
    },
}

impl Chunk for Bgnd {
    fn parse(buf: &mut std::io::Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized {
        let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
        let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
        let mut backgrounds = HashMap::new();
        for addr in entries_addrs {
            buf.set_position(addr.try_into()?);
            // println!("{}", buf.position());

            let name = read_string_ptr(buf)?;
            let unknown1 = (0..3).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
            let texture_address = buf.read_i32::<LittleEndian>()?;

            backgrounds.insert(name, BackgroundEntry {
                _unknown1: unknown1,
                texture: BackgroundState::Unloaded {
                    texture_address,
                },
            });
        }

        Ok(Bgnd {
            backgrounds,
        })
    }

    fn get_id() -> [u8; 4] {
        *b"BGND"
    }
}