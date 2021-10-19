use std::{convert::TryInto, fs, io::{Cursor, Read}, path::Path};
use byteorder::{LittleEndian, ReadBytesExt};
use image::DynamicImage;
use thiserror::Error;
use tuple_transpose::TupleTranspose;

#[derive(Error, Debug)]
pub enum ExtractError {
    #[error("failed to find file at the provided position")]
    MissingFile,
    #[error("io error")]
    IOError(std::io::Error),
    #[error("unknown extract error")]
    Unknown,
}

pub fn parse<P: AsRef<Path>>(path: P) -> Result<DataWin, anyhow::Error>{
    let p = path.as_ref();

    let bytes = fs::read(p)?;
    let n_bytes = bytes.len();

    println!("Parsing {} bytes...", n_bytes);

    let mut buf = Cursor::new(bytes);

    let mut form_chunk_name_buf = [0u8; 4];
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
        while buf.position() < n_bytes.try_into().unwrap() {
            let mut chunk_name_buf = [0u8; 4];
            buf.read_exact(&mut chunk_name_buf).expect("failed to read");
            let chunk_len = buf.read_i32::<LittleEndian>()?;

            println!("chunk {}, len: {}", String::from_utf8_lossy(&chunk_name_buf), chunk_len);
            let this_chunk_pos = buf.position();

            match &chunk_name_buf {
                b"GEN8" => {
                    let debug = buf.read_u8()?;
                    let _unknown1 = buf.read_i24::<LittleEndian>()?;
                    let filename = read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf)?;
                    let config = read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf)?;
                    let last_obj = buf.read_u32::<LittleEndian>()?;
                    let last_tile = buf.read_u32::<LittleEndian>()?;
                    let game_id = buf.read_u32::<LittleEndian>()?;
                    let _unknown2 = (0..4).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
                    let name = read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf)?;
                    let major = buf.read_i32::<LittleEndian>()?;
                    let minor = buf.read_i32::<LittleEndian>()?;
                    let release = buf.read_i32::<LittleEndian>()?;
                    let build = buf.read_i32::<LittleEndian>()?;
                    let default_window_width = buf.read_i32::<LittleEndian>()?;
                    let default_window_height = buf.read_i32::<LittleEndian>()?;
                    let info = buf.read_u32::<LittleEndian>()?; // could parse more: InfoFlags
                    let license_md5 = (0..0x10).map(|_| buf.read_u8()).collect::<Result<Vec<u8>, std::io::Error>>()?;
                    let license_crc32 = buf.read_u32::<LittleEndian>()?;
                    let timestamp = buf.read_u64::<LittleEndian>()?;
                    let display_name = read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf)?;
                    let active_targets = buf.read_u32::<LittleEndian>()?; // unknown flags: GameTargets
                    let _unknown3 = (0..4).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
                    let steam_app_id = buf.read_u32::<LittleEndian>()?;
                    let number_count = buf.read_u32::<LittleEndian>()?;
                    let numbers = (0..number_count).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;

                    gen8 = Some(Gen8 {
                        debug,
                        _unknown1,
                        filename,
                        config,
                        last_obj,
                        last_tile,
                        game_id,
                        _unknown2,
                        name,
                        major,
                        minor,
                        release,
                        build,
                        default_window_width,
                        default_window_height,
                        info,
                        license_md5,
                        license_crc32,
                        timestamp,
                        display_name,
                        active_targets,
                        _unknown3,
                        steam_app_id,
                        number_count,
                        numbers,
                    });
                },
                b"OPTN" => {
                    let _unknown1 = (0..2).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
                    let info = buf.read_u32::<LittleEndian>()?; // could parse more: InfoFlags
                    let _unknown2 = (0..0xC).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
                    let constants_addr_ct = buf.read_i32::<LittleEndian>()?;
                    let constants_addrs = (0..constants_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
                    let constant_map = (0..constants_addrs.len()).map(|_| {
                        (read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf), read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf)).transpose()
                    }).collect::<Result<Vec<(String, String)>, anyhow::Error>>()?;

                    optn = Some(Optn {
                        _unknown1,
                        info,
                        _unknown2,
                        constant_map,
                    })
                },
                b"EXTN" => {
                    // unused
                },
                b"SOND" => {
                    let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
                    let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
                    let mut sounds = Vec::new();
                    for i in 0..entries_addrs.len() {
                        buf.set_position(entries_addrs[i].try_into().unwrap());

                        let name = read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf)?;
                        let flags = buf.read_u32::<LittleEndian>()?;
                        let type_ = read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf)?;
                        let file = read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf)?;
                        let _unknown1 = buf.read_u32::<LittleEndian>()?;
                        let volume = buf.read_f32::<LittleEndian>()?;
                        let pitch = buf.read_f32::<LittleEndian>()?;
                        let group_id = buf.read_i32::<LittleEndian>()?;
                        let audio_id = buf.read_i32::<LittleEndian>()?;

                        sounds.push(SoundEntry {
                            name,
                            flags,
                            type_,
                            file,
                            _unknown1,
                            volume,
                            pitch,
                            group_id,
                            audio_id,
                        });
                    }

                    sond = Some(Sond {
                        sounds,
                    });
                },
                b"AGRP" => {
                    // TODO: List<AudioGroup>
                },
                b"SPRT" => {
                    let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
                    let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
                    let mut sprites = Vec::new();
                    for i in 0..entries_addrs.len() {
                        buf.set_position(entries_addrs[i].try_into().unwrap());
                        // println!("{}", buf.position());

                        let name = read_string_at(buf.read_i32::<LittleEndian>()? as u64, &mut buf)?;
                        let width = buf.read_i32::<LittleEndian>()?;
                        let height = buf.read_i32::<LittleEndian>()?;
                        let margin_left = buf.read_i32::<LittleEndian>()?;
                        let margin_right = buf.read_i32::<LittleEndian>()?;
                        let margin_bottom = buf.read_i32::<LittleEndian>()?;
                        let margin_top = buf.read_i32::<LittleEndian>()?;
                        let _unknown1 = (0..3).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
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
                        for addr in &texture_addresses {
                            // assert!(*addr != 0, "texture_address == 0 in sprite @ position {}", entries_addrs[i]);
                        }

                        sprites.push(SpriteEntry {
                            name,
                            width,
                            height,
                            margin_left,
                            margin_right,
                            margin_bottom,
                            margin_top,
                            _unknown1,
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

                    sprt = Some(Sprt {
                        sprites,
                    });
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
                    let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
                    let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
                    let mut textures = Vec::new();
                    for i in 0..entries_addrs.len() {
                        buf.set_position(entries_addrs[i].try_into().unwrap());
                        // println!("{}", buf.position());
                        // if buf.position() == 0x011c9c72 {
                        //     panic!("here!!");
                        // }

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
                        })
                    }

                    tpag = Some(Tpag {
                        textures,
                    });
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
                    let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
                    let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
                    let mut spritesheets = Vec::new();
                    for i in 0..entries_addrs.len() {
                        buf.set_position(entries_addrs[i].try_into().unwrap());

                        let _unknown1 = buf.read_u32::<LittleEndian>()?;
                        let _unknown2 = buf.read_u32::<LittleEndian>()?; // this differs from the unpacking page, but is necessary now
                        let png_addr = buf.read_u32::<LittleEndian>()?;

                        spritesheets.push(SpritesheetEntry {
                            _unknown1,
                            _unknown2,
                            png: PNGState::Unloaded {
                                png_addr,
                            },
                        });
                    }

                    txtr = Some(Txtr {
                        spritesheets,
                    });
                },
                b"AUDO" => {

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

fn read_string_at(pos: u64, buf: &mut Cursor<Vec<u8>>) -> Result<String, anyhow::Error> {
    if pos == 0 {
        return Ok("".to_string());
    }

    let pos_before = buf.position();
    buf.set_position(pos);
    let mut build = Vec::new();

    loop {
        let val = buf.read_u8()?;
        if val == 0 {
            break;
        }

        build.push(val);
    }

    buf.set_position(pos_before);

    // println!("read_string_at {}: {:?}", pos, build);

    Ok(String::from_utf8(build)?)
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
                match spr.png {
                    PNGState::Unloaded { png_addr } => {
                        self.buf.set_position(png_addr.into());

                        let texture = image::io::Reader::new(&mut self.buf)
                            .with_guessed_format()?
                            .decode()?;

                        spr.png = PNGState::Loaded {
                            texture
                        };
                    }
                    _ => {},
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
                        match &spr.textures {
                            SpriteState::Unloaded { texture_count: _, texture_addresses } => {

                                let mut textures = Vec::new();

                                for addr in texture_addresses {
                                    self.buf.set_position(*addr as u64);

                                    if *addr != 0 {
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
                                                Ok(texture.crop(tex.x as u32, tex.y as u32, tex.width as u32, tex.height as u32))
                                            }
                                            _ => Err(anyhow::anyhow!("Spritesheet not loaded!")),
                                        }?;

                                        textures.push(texture);
                                    }else{
                                        println!("sprite {} has a texture_addr == 0", spr.name);
                                    }
                                }

                                // assert_eq!(textures.len(), texture_addresses.len());

                                spr.textures = SpriteState::Loaded {
                                    textures
                                };
                            }
                            _ => {},
                        }
                    }
                }
            // }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Gen8 {
    pub debug: u8,
    pub _unknown1: i32,
    pub filename: String,
    pub config: String,
    pub last_obj: u32,
    pub last_tile: u32,
    pub game_id: u32,
    pub _unknown2: Vec<u32>,
    pub name: String,
    pub major: i32,
    pub minor: i32,
    pub release: i32,
    pub build: i32,
    pub default_window_width: i32,
    pub default_window_height: i32,
    pub info: u32, // InfoFlags
    pub license_md5: Vec<u8>,
    pub license_crc32: u32,
    pub timestamp: u64,
    pub display_name: String,
    pub active_targets: u32, // GameTargets,
    pub _unknown3: Vec<u32>,
    pub steam_app_id: u32,
    pub number_count: u32,
    pub numbers: Vec<u32>,
}

#[derive(Debug)]
pub struct Optn {
    pub _unknown1: Vec<u32>, // len=2
    pub info: u32, // InfoFlags,
    pub _unknown2: Vec<u32>, // len=0xC
    pub constant_map: Vec<(String, String)>,
}

#[derive(Debug)]
pub struct Sond {
    pub sounds: Vec<SoundEntry>,
}

#[derive(Debug)]
pub struct SoundEntry {
    pub name: String,
    pub flags: u32, // SoundEntryFlags
    pub type_: String,
    pub file: String,
    pub _unknown1: u32,
    pub volume: f32,
    pub pitch: f32,
    pub group_id: i32,
    pub audio_id: i32,
}

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