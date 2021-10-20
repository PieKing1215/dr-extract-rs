use std::convert::TryInto;

use byteorder::{LittleEndian, ReadBytesExt};
use image::DynamicImage;

use super::{Chunk, read_string_ptr};


#[derive(Debug)]
pub struct Sprt {
    pub sprites: Vec<SpriteEntry>,
}

#[derive(Debug)]
pub struct SpriteEntry {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub margin_left: i32,
    pub margin_right: i32,
    pub margin_bottom: i32,
    pub margin_top: i32,
    pub _unknown1: Vec<u32>,
    pub bbox_mode: u32,
    pub sep_masks: u32,
    pub origin_x: u32,
    pub origin_y: u32,
    pub textures: SpriteState,
    // unknown bytes to next object
}

#[derive(Debug)]
pub enum SpriteState {
    Unloaded {
        texture_count: i32,
        texture_addresses: Vec<i32>, // addrs to TPAG
    },
    Loaded {
        textures: Vec<DynamicImage>,
    },
}

impl Chunk for Sprt {
    fn parse(buf: &mut std::io::Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized {
        let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
        let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
        let mut sprites = Vec::new();
        for addr in entries_addrs {
            buf.set_position(addr.try_into()?);
            // println!("{}", buf.position());

            let name = read_string_ptr(buf)?;
            let width = buf.read_i32::<LittleEndian>()?;
            let height = buf.read_i32::<LittleEndian>()?;
            let margin_left = buf.read_i32::<LittleEndian>()?;
            let margin_right = buf.read_i32::<LittleEndian>()?;
            let margin_bottom = buf.read_i32::<LittleEndian>()?;
            let margin_top = buf.read_i32::<LittleEndian>()?;
            let unknown1 = (0..3).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
            let bbox_mode = buf.read_u32::<LittleEndian>()?;
            let sep_masks = buf.read_u32::<LittleEndian>()?;
            let origin_x = buf.read_u32::<LittleEndian>()?;
            let origin_y = buf.read_u32::<LittleEndian>()?;
            let _unknown2 = (0..7).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
            // let _unknown2 = buf.read_i32::<LittleEndian>()?;
            // let _unknown3 = buf.read_u32::<LittleEndian>()?;
            // let _unknown4 = buf.read_u32::<LittleEndian>()?;
            // let _unknown5 = buf.read_f32::<LittleEndian>()?;
            // let _unknown6 = buf.read_u32::<LittleEndian>()?;
            let texture_count = buf.read_i32::<LittleEndian>()?;
            let texture_addresses = (0..texture_count).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;

            sprites.push(SpriteEntry {
                name,
                width,
                height,
                margin_left,
                margin_right,
                margin_bottom,
                margin_top,
                _unknown1: unknown1,
                bbox_mode,
                sep_masks,
                origin_x,
                origin_y,
                textures: SpriteState::Unloaded {
                    texture_count,
                    texture_addresses,
                },
            });
        }

        Ok(Sprt {
            sprites,
        })
    }

    fn get_id() -> [u8; 4] {
        *b"SPRT"
    }
}