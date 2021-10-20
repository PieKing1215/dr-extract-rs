use std::convert::TryInto;

use byteorder::{LittleEndian, ReadBytesExt};

use super::Chunk;


#[derive(Debug)]
pub struct Tpag {
    pub textures: Vec<TextureEntry>,
}

#[derive(Debug)]
pub struct TextureEntry {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub render_x: u16,
    pub render_y: u16,
    pub bouding_x: u16,
    pub bouding_y: u16,
    pub bouding_width: u16,
    pub bouding_height: u16,
    pub spritesheet_id: u16,
}

impl Chunk for Tpag {
    fn parse(buf: &mut std::io::Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized {
        let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
        let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
        let mut textures = Vec::new();
        for addr in entries_addrs {
            buf.set_position(addr.try_into()?);
            // println!("{}", buf.position());

            let x = buf.read_u16::<LittleEndian>()?;
            let y = buf.read_u16::<LittleEndian>()?;
            let width = buf.read_u16::<LittleEndian>()?;
            let height = buf.read_u16::<LittleEndian>()?;
            let render_x = buf.read_u16::<LittleEndian>()?;
            let render_y = buf.read_u16::<LittleEndian>()?;
            let bouding_x = buf.read_u16::<LittleEndian>()?;
            let bouding_y = buf.read_u16::<LittleEndian>()?;
            let bouding_width = buf.read_u16::<LittleEndian>()?;
            let bouding_height = buf.read_u16::<LittleEndian>()?;
            let spritesheet_id = buf.read_u16::<LittleEndian>()?;

            textures.push(TextureEntry {
                x,
                y,
                width,
                height,
                render_x,
                render_y,
                bouding_x,
                bouding_y,
                bouding_width,
                bouding_height,
                spritesheet_id,
            });
        }

        Ok(Tpag {
            textures,
        })
    }
}