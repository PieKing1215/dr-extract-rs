use std::{collections::HashMap, convert::TryInto};

use byteorder::{LittleEndian, ReadBytesExt};
use image::DynamicImage;

use super::{Chunk, read_string_ptr};


#[derive(Debug)]
pub struct Font {
    pub fonts: HashMap<String, FontEntry>,
}

#[derive(Debug)]
pub struct FontEntry {
    pub system_name: String,
    pub em_size: f32,
    pub bold: bool,
    pub italic: bool,
    pub range_start: u16,
    pub charset: u8,
    pub antialiasing: u8,
    pub range_end: u32,
    pub tpag_addr: u32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub glyphs: HashMap<u16, Glyph>,
}

#[derive(Debug)]
pub struct Glyph {
    pub relative_x: u16,
    pub relative_y: u16,
    pub width: u16,
    pub height: u16,
    pub _unknown1: Vec<u8>,
    pub texture: Option<DynamicImage>,
}

impl Chunk for Font {
    fn parse(buf: &mut std::io::Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized {
        let f_entries_addr_ct = buf.read_i32::<LittleEndian>()?;
        let f_entries_addrs = (0..f_entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
        let mut fonts = HashMap::new();

        for f_addr in f_entries_addrs {
            buf.set_position(f_addr.try_into()?);

            let code_name = read_string_ptr(buf)?;
            let system_name = read_string_ptr(buf)?;
            let em_size = -buf.read_f32::<LittleEndian>()?;
            let bold = buf.read_u32::<LittleEndian>()? == 1;
            let italic = buf.read_u32::<LittleEndian>()? == 1;
            let range_start = buf.read_u16::<LittleEndian>()?;
            let charset = buf.read_u8()?;
            let antialiasing = buf.read_u8()?;
            let range_end = buf.read_u32::<LittleEndian>()?;
            let tpag_addr = buf.read_u32::<LittleEndian>()?;
            let scale_x = buf.read_f32::<LittleEndian>()?;
            let scale_y = buf.read_f32::<LittleEndian>()?;
            let _unknown = buf.read_i32::<LittleEndian>()?;

            let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
            let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
            let mut glyphs = HashMap::new();
            for addr in entries_addrs {
                buf.set_position(addr.try_into()?);
                // println!("{}", buf.position());
                let character = buf.read_u16::<LittleEndian>()?;
                let relative_x = buf.read_u16::<LittleEndian>()?;
                let relative_y = buf.read_u16::<LittleEndian>()?;
                let width = buf.read_u16::<LittleEndian>()?;
                let height = buf.read_u16::<LittleEndian>()?;

                let unknown1 = (0..4).map(|_| buf.read_u8()).collect::<Result<Vec<u8>, std::io::Error>>()?;

                glyphs.insert(character, Glyph {
                    relative_x,
                    relative_y,
                    width,
                    height,
                    _unknown1: unknown1,
                    texture: None,
                });
            }

            fonts.insert(code_name, FontEntry {
                system_name,
                em_size,
                bold,
                italic,
                range_start,
                charset,
                antialiasing,
                range_end,
                tpag_addr,
                scale_x,
                scale_y,
                glyphs,
            });

        }

        Ok(Font {
            fonts,
        })
    }

    fn get_id() -> [u8; 4] {
        *b"FONT"
    }
}