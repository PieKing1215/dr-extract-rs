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
    pub _unknown2: u32,
    pub tile_width: u32,
    pub tile_height: u32,
    pub margin_x: u32,
    pub margin_y: u32,
    pub columns: u32,
    pub _unknown3: u32,
    pub _unknown4: u32,
    pub ids: Vec<u32>,
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
            let unknown2 = buf.read_u32::<LittleEndian>()?;
            let tile_width = buf.read_u32::<LittleEndian>()?;
            let tile_height = buf.read_u32::<LittleEndian>()?;
            let margin_x = buf.read_u32::<LittleEndian>()?;
            let margin_y = buf.read_u32::<LittleEndian>()?;
            let columns = buf.read_u32::<LittleEndian>()?;
            let count_per = buf.read_u32::<LittleEndian>()?;
            let count = buf.read_u32::<LittleEndian>()?;
            let unknown3 = buf.read_u32::<LittleEndian>()?;
            let unknown4 = buf.read_u32::<LittleEndian>()?;
            let ids = (0..count*count_per).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;

            backgrounds.insert(name, BackgroundEntry {
                _unknown1: unknown1,
                texture: BackgroundState::Unloaded {
                    texture_address,
                },
                _unknown2: unknown2,
                tile_width,
                tile_height,
                margin_x,
                margin_y,
                columns,
                _unknown3: unknown3,
                _unknown4: unknown4,
                ids,
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