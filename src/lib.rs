#![deny(clippy::all)]
#![deny(clippy::cargo)]
#![warn(clippy::pedantic)]

#![allow(clippy::cast_sign_loss)]
#![allow(clippy::missing_errors_doc)]

use chunk::{Gen8, Optn, PNGState, Sond, SpriteState, Sprt, TextureEntry, Tpag, Txtr};

use std::{collections::HashMap, convert::TryInto, fs, io::{Cursor, Read}, path::Path};
use byteorder::{LittleEndian, ReadBytesExt};

use crate::chunk::Chunk;

pub mod chunk;

pub fn prepare_file<P: AsRef<Path>>(path: P) -> Result<DataWinReady, anyhow::Error> {
    prepare_bytes(fs::read(path.as_ref())?)
}

pub fn prepare_bytes(bytes: Vec<u8>) -> Result<DataWinReady, anyhow::Error> {
    let n_bytes = bytes.len();

    println!("Given {} bytes...", n_bytes);

    let buf = Cursor::new(bytes);

    Ok(DataWinReady {
        n_bytes,
        buf,
    })
}

pub struct DataWinReady {
    n_bytes: usize,
    buf: Cursor<Vec<u8>>,
}

impl DataWinReady {
    pub fn fetch_chunks(mut self) -> Result<DataWin, anyhow::Error> {

        let mut form_chunk_name_buf = [0_u8; 4];
        self.buf.read_exact(&mut form_chunk_name_buf)?;
        let _form_chunk_len = self.buf.read_i32::<LittleEndian>()?;

        let mut chunk_addrs = HashMap::new();
        
        if &form_chunk_name_buf == b"FORM" {
            while self.buf.position() < self.n_bytes.try_into()? {
                let mut chunk_name_buf = [0_u8; 4];
                self.buf.read_exact(&mut chunk_name_buf)?;
                let chunk_len = self.buf.read_i32::<LittleEndian>()?;
    
                println!("chunk {}, len: {}", String::from_utf8_lossy(&chunk_name_buf), chunk_len);

                let this_chunk_pos = self.buf.position();

                self.buf.set_position(this_chunk_pos + chunk_len as u64);

                chunk_addrs.insert(chunk_name_buf, this_chunk_pos);
            }

            Ok(DataWin {
                buf: self.buf,
                chunk_addrs,
                gen8: None,
                optn: None,
                sond: None,
                sprt: None,
                tpag: None,
                txtr: None,
            })
        }else {
            Err(anyhow::anyhow!("Could not find \"FORM\" chunk!"))
        }
    }
}

#[derive(Debug)]
pub struct DataWin {
    buf: Cursor<Vec<u8>>,
    chunk_addrs: HashMap<[u8; 4], u64>,
    pub gen8: Option<Gen8>,
    pub optn: Option<Optn>,
    pub sond: Option<Sond>,
    pub sprt: Option<Sprt>,
    pub tpag: Option<Tpag>,
    pub txtr: Option<Txtr>,
}

impl DataWin {
    fn parse_chunk<T: Chunk>(&mut self) -> anyhow::Result<T> {
        if let Some(addr) = self.chunk_addrs.get(&T::get_id()) {
            self.buf.set_position(*addr);
            T::parse(&mut self.buf)
        } else {
            Err(anyhow::anyhow!("Chunk \"{}\" is not present!", String::from_utf8_lossy(&T::get_id())))
        }
    }

    pub fn parse_gen8(&mut self) -> anyhow::Result<()> {
        if self.gen8.is_none() {
            self.gen8 = Some(self.parse_chunk::<Gen8>()?);
        }
        Ok(())
    }

    pub fn parse_optn(&mut self) -> anyhow::Result<()> {
        if self.optn.is_none() {
            self.optn = Some(self.parse_chunk::<Optn>()?);
        }
        Ok(())
    }

    pub fn parse_sond(&mut self) -> anyhow::Result<()> {
        if self.sond.is_none() {
            self.sond = Some(self.parse_chunk::<Sond>()?);
        }
        Ok(())
    }

    pub fn parse_sprt(&mut self) -> anyhow::Result<()> {
        if self.sprt.is_none() {
            self.sprt = Some(self.parse_chunk::<Sprt>()?);
        }
        Ok(())
    }

    pub fn parse_tpag(&mut self) -> anyhow::Result<()> {
        if self.tpag.is_none() {
            self.tpag = Some(self.parse_chunk::<Tpag>()?);
        }
        Ok(())
    }

    pub fn parse_txtr(&mut self) -> anyhow::Result<()> {
        if self.txtr.is_none() {
            self.txtr = Some(self.parse_chunk::<Txtr>()?);
        }
        Ok(())
    }

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
        } else {
            return Err(anyhow::anyhow!("TXTR chunk must be parsed before calling load_spritesheets!"));
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
                } else {
                    return Err(anyhow::anyhow!("TXTR chunk must be parsed before calling load_sprites!"));
                }
            // }
        } else {
            return Err(anyhow::anyhow!("SPRT chunk must be parsed before calling load_sprites!"));
        }

        Ok(())
    }
}