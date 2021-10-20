use byteorder::{LittleEndian, ReadBytesExt};

use super::{Chunk, read_string_ptr};


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

impl Chunk for Gen8 {
    fn parse(buf: &mut std::io::Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized {
        let debug = buf.read_u8()?;
        let unknown1 = buf.read_i24::<LittleEndian>()?;
        let filename = read_string_ptr(buf)?;
        let config = read_string_ptr(buf)?;
        let last_obj = buf.read_u32::<LittleEndian>()?;
        let last_tile = buf.read_u32::<LittleEndian>()?;
        let game_id = buf.read_u32::<LittleEndian>()?;
        let unknown2 = (0..4).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
        let name = read_string_ptr(buf)?;
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
        let display_name = read_string_ptr(buf)?;
        let active_targets = buf.read_u32::<LittleEndian>()?; // unknown flags: GameTargets
        let unknown3 = (0..4).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;
        let steam_app_id = buf.read_u32::<LittleEndian>()?;
        let number_count = buf.read_u32::<LittleEndian>()?;
        let numbers = (0..number_count).map(|_| buf.read_u32::<LittleEndian>()).collect::<Result<Vec<u32>, std::io::Error>>()?;

        Ok(Gen8 {
            debug,
            _unknown1: unknown1,
            filename,
            config,
            last_obj,
            last_tile,
            game_id,
            _unknown2: unknown2,
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
            _unknown3: unknown3,
            steam_app_id,
            number_count,
            numbers,
        })
    }

    fn get_id() -> [u8; 4] {
        *b"GEN8"
    }
}