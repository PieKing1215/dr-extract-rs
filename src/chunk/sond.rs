use std::{collections::HashMap, convert::TryInto};

use byteorder::{LittleEndian, ReadBytesExt};

use super::{Chunk, read_string_ptr};


#[derive(Debug)]
pub struct Sond {
    pub sounds: HashMap<String, SoundEntry>,
}

#[derive(Debug)]
pub struct SoundEntry {
    pub flags: u32, // SoundEntryFlags
    pub type_: String,
    pub file: String,
    pub _unknown1: u32,
    pub volume: f32,
    pub pitch: f32,
    pub group_id: i32,
    pub audio_id: i32,
    pub audio_data: Option<AudioType>,
}

#[derive(Debug)]
pub enum AudioType {
    Internal(Vec<u8>),
    External,
}

impl Chunk for Sond {
    fn parse(buf: &mut std::io::Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized {
        let entries_addr_ct = buf.read_i32::<LittleEndian>()?;
        let entries_addrs = (0..entries_addr_ct).map(|_| buf.read_i32::<LittleEndian>()).collect::<Result<Vec<i32>, std::io::Error>>()?;
        let mut sounds = HashMap::new();
        for addr in entries_addrs {
            buf.set_position(addr.try_into()?);

            let name = read_string_ptr(buf)?;
            let flags = buf.read_u32::<LittleEndian>()?;
            let type_ = read_string_ptr(buf)?;
            let file = read_string_ptr(buf)?;
            let unknown1 = buf.read_u32::<LittleEndian>()?;
            let volume = buf.read_f32::<LittleEndian>()?;
            let pitch = buf.read_f32::<LittleEndian>()?;
            let group_id = buf.read_i32::<LittleEndian>()?;
            let audio_id = buf.read_i32::<LittleEndian>()?;

            sounds.insert(name, SoundEntry {
                flags,
                type_,
                file,
                _unknown1: unknown1,
                volume,
                pitch,
                group_id,
                audio_id,
                audio_data: None,
            });
        }

        Ok(Sond {
            sounds,
        })
    }

    fn get_id() -> [u8; 4] {
        *b"SOND"
    }
}