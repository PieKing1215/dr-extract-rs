#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![warn(clippy::pedantic)]

#![allow(clippy::cast_sign_loss)]
#![allow(clippy::missing_errors_doc)]

use chunk::{Gen8, Optn, PNGState, Sond, SpriteState, Sprt, TextureEntry, Tpag, Txtr};

use std::{convert::TryInto, fs, io::{Cursor, Read}, path::Path};
use byteorder::{LittleEndian, ReadBytesExt};
use thiserror::Error;

use crate::chunk::Chunk;

pub mod chunk;

#[allow(clippy::too_many_lines)]
pub fn parse<P: AsRef<Path>>(path: P) -> Result<DataWin, anyhow::Error> {
    let p = path.as_ref();

    let bytes = fs::read(p)?;
    let n_bytes = bytes.len();

    println!("Parsing {} bytes...", n_bytes);

    let mut buf = Cursor::new(bytes);

    let mut form_chunk_name_buf = [0_u8; 4];
    buf.read_exact(&mut form_chunk_name_buf).expect("failed to read");
    let form_chunk_len = buf.read_i32::<LittleEndian>()?;

    let mut gen8: Option<Gen8> = None;
    let mut optn: Option<Optn> = None;
    let mut sond: Option<Sond> = None;
    let mut sprt: Option<Sprt> = None;
    let mut tpag: Option<Tpag> = None;
    let mut txtr: Option<Txtr> = None;

    println!("chunk {}, len: {}", String::from_utf8_lossy(&form_chunk_name_buf), form_chunk_len);
    if &form_chunk_name_buf == b"FORM" {
        while buf.position() < n_bytes.try_into()? {
            let mut chunk_name_buf = [0_u8; 4];
            buf.read_exact(&mut chunk_name_buf).expect("failed to read");
            let chunk_len = buf.read_i32::<LittleEndian>()?;

            println!("chunk {}, len: {}", String::from_utf8_lossy(&chunk_name_buf), chunk_len);
            let this_chunk_pos = buf.position();

            #[allow(clippy::match_same_arms)]
            match &chunk_name_buf {
                b"GEN8" => {
                    gen8 = Some(Gen8::parse(&mut buf)?);
                },
                b"OPTN" => {
                    optn = Some(Optn::parse(&mut buf)?);
                },
                b"EXTN" => {
                    // unused
                },
                b"SOND" => {
                    sond = Some(Sond::parse(&mut buf)?);
                },
                b"AGRP" => {
                    // TODO: List<AudioGroup>
                },
                b"SPRT" => {
                    sprt = Some(Sprt::parse(&mut buf)?);
                },
                b"BGND" => {
                    // TODO: List<Background>
                },
                b"PATH" => {
                    // TODO: List<Path>
                },
                b"SCPT" => {
                    // TODO: List<ScriptDefinition>
                },
                b"SHDR" => {
                    // unused
                },
                b"FONT" => {
                    // TODO: List<Font>
                },
                b"TMLN" => {
                    // unused
                },
                b"OBJT" => {
                    // TODO: List<GameObjectDefinition>
                },
                b"ROOM" => {
                    // TODO: List<Room>
                },
                b"DAFL" => {
                    // unused
                },
                b"TPAG" => {
                    tpag = Some(Tpag::parse(&mut buf)?);
                },
                b"CODE" => {
                    // TODO
                },
                b"VARI" => {
                    // TODO
                },
                b"FUNC" => {
                    // TODO
                },
                b"STRG" => {
                    // TODO: List<(length: u32, value: String)>
                },
                b"TXTR" => {
                    txtr = Some(Txtr::parse(&mut buf)?);
                },
                b"AUDO" => {
                    // TODO
                },
                _ => {}
            }

            buf.set_position(this_chunk_pos + chunk_len as u64);
        }
    }else {
        println!("FORM chunk not found");
    }

    Ok(DataWin {
        buf,
        gen8,
        optn,
        sond,
        sprt,
        tpag,
        txtr,
    })
}

#[derive(Debug)]
pub struct DataWin {
    buf: Cursor<Vec<u8>>,
    pub gen8: Option<Gen8>,
    pub optn: Option<Optn>,
    pub sond: Option<Sond>,
    pub sprt: Option<Sprt>,
    pub tpag: Option<Tpag>,
    pub txtr: Option<Txtr>,
}

impl DataWin {
    pub fn load_spritesheets(&mut self) -> anyhow::Result<()> {

        if let Some(txtr) = &mut self.txtr {
            for spr in &mut txtr.spritesheets {
                if let PNGState::Unloaded { png_addr } = spr.png {
                    self.buf.set_position(png_addr.into());

                    let texture = image::io::Reader::new(&mut self.buf)
                        .with_guessed_format()?
                        .decode()?;

                    spr.png = PNGState::Loaded {
                        texture
                    };
                }
            }
        }

        Ok(())
    }

    pub fn load_sprites(&mut self) -> anyhow::Result<()> {

        if let Some(sprt) = &mut self.sprt {
            // we don't actually use the values already stored in TPAG since the sprite only knows the address, not the index...
            //   not really sure if it would be worth adding a hashmap or something to save the TPAG entries' addresses so we can look them up here?

            // if let Some(tpag) = &mut self.tpag {
                if let Some(txtr) = &mut self.txtr {
                    for spr in &mut sprt.sprites {
                        if let SpriteState::Unloaded { texture_count: _, texture_addresses } = &spr.textures {

                            let mut textures = Vec::new();

                            for addr in texture_addresses {
                                self.buf.set_position(*addr as u64);

                                if *addr == 0 {
                                    println!("sprite {} has a texture_addr == 0", spr.name);
                                }else{
                                    let x = self.buf.read_u16::<LittleEndian>()?;
                                    let y = self.buf.read_u16::<LittleEndian>()?;
                                    let width = self.buf.read_u16::<LittleEndian>()?;
                                    let height = self.buf.read_u16::<LittleEndian>()?;
                                    let render_x = self.buf.read_u16::<LittleEndian>()?;
                                    let render_y = self.buf.read_u16::<LittleEndian>()?;
                                    let bouding_x = self.buf.read_u16::<LittleEndian>()?;
                                    let bouding_y = self.buf.read_u16::<LittleEndian>()?;
                                    let bouding_width = self.buf.read_u16::<LittleEndian>()?;
                                    let bouding_height = self.buf.read_u16::<LittleEndian>()?;
                                    let spritesheet_id = self.buf.read_u16::<LittleEndian>()?;

                                    let tex = TextureEntry {
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
                                    };

                                    // println!("addr = {}, spritesheet_id = {}", addr, spritesheet_id);

                                    let sheet = &mut txtr.spritesheets[tex.spritesheet_id as usize];
                                    let texture = match &mut sheet.png {
                                        PNGState::Loaded { texture } => {
                                            Ok(texture.crop(u32::from(tex.x), u32::from(tex.y), u32::from(tex.width), u32::from(tex.height)))
                                        }
                                        PNGState::Unloaded{ .. } => Err(anyhow::anyhow!("Spritesheet not loaded!")),
                                    }?;

                                    textures.push(texture);
                                }
                            }

                            // assert_eq!(textures.len(), texture_addresses.len());

                            spr.textures = SpriteState::Loaded {
                                textures
                            };
                        }
                    }
                }
            // }
        }

        Ok(())
    }
}