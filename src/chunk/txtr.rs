use std::convert::TryInto;

use byteorder::{LittleEndian, ReadBytesExt};
use image::DynamicImage;

use super::Chunk;


#[derive(Debug)]
pub struct Txtr {
    pub spritesheets: Vec<SpritesheetEntry>,
}

#[derive(Debug)]
pub struct SpritesheetEntry {
    pub _unknown1: u32,
    pub _unknown2: u32,
    pub png: PNGState,
}

#[derive(Debug)]
pub enum PNGState {
    Unloaded {
        png_addr: u32,
    },
    Loaded {
        texture: DynamicImage,
    },
}

impl Chunk for Txtr {
    fn parse(buf: &mut std::io::Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized {
        let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
        let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
        let mut spritesheets = Vec::new();
        for addr in entries_addrs {
            buf.set_position(addr.try_into()?);

            let unknown1 = buf.read_u32::<LittleEndian>()?;
            let unknown2 = buf.read_u32::<LittleEndian>()?; // this differs from the unpacking page, but is necessary now
            let png_addr = buf.read_u32::<LittleEndian>()?;

            spritesheets.push(SpritesheetEntry {
                _unknown1: unknown1,
                _unknown2: unknown2,
                png: PNGState::Unloaded {
                    png_addr,
                },
            });
        }

        Ok(Txtr {
            spritesheets,
        })
    }
}