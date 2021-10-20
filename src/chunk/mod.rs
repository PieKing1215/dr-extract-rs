

use std::io::Cursor;

mod gen8;
mod optn;
mod sond;
mod sprt;
mod tpag;
mod txtr;
use byteorder::{LittleEndian, ReadBytesExt};
pub use gen8::*;
pub use optn::*;
pub use sond::*;
pub use sprt::*;
pub use tpag::*;
pub use txtr::*;

pub trait Chunk {
    fn parse(buf: &mut Cursor<Vec<u8>>) -> anyhow::Result<Self> where Self: std::marker::Sized;
    fn get_id() -> [u8; 4];
}

fn read_string_ptr(buf: &mut Cursor<Vec<u8>>) -> Result<String, anyhow::Error> {
    read_string_at(buf.read_i32::<LittleEndian>()? as u64, buf)
}

fn read_string_at(pos: u64, buf: &mut Cursor<Vec<u8>>) -> Result<String, anyhow::Error> {
    if pos == 0 {
        return Ok("".to_string());
    }

    let pos_before = buf.position();
    buf.set_position(pos);
    
    // explicitly don't use ? here, since we wouldn't get to do the buf.set_position
    let str = read_string_raw(buf);

    buf.set_position(pos_before);

    str
}

fn read_string_raw(buf: &mut Cursor<Vec<u8>>) -> Result<String, anyhow::Error> {
    let mut build = Vec::new();

    loop {
        let val = buf.read_u8()?;
        if val == 0 {
            break;
        }

        build.push(val);
    }

    Ok(String::from_utf8(build)?)
}