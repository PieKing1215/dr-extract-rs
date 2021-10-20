<h1>dr-extract<br>
    <a href="https://github.com/PieKing1215/dr-extract-rs/actions/workflows/rust_build_test.yml"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/PieKing1215/dr-extract-rs/Rust%20Build+Test"></a>
</h1>

A WIP Rust library for parsing and extracting assets from [DELTARUNE](https://deltarune.com)'s data.win.

You must provide your own data.win file. I will not host it myself, nor will I host any extracted assets here.

It might work for other GameMaker games, but I have not tested any.

Based on the documentation here: https://pcy.ulyssis.be/undertale/unpacking-corrected<br>
(+ some adjustments for the newer version of GM)

Currently only the game metadata, audio metadata (no actual waveform data), spritesheets, and sprites are fully extractable.<br>
More specifically, these chunks are supported right now:
- GEN8
- OPTN
- SOND
- SPRT
- TPAG
- TXTR

Not supported right now:
- EXTN (unused)
- AGRP
- BGND
- PATH
- SCPT
- SHDR (unused)
- FONT
- TMLN
- OBJT
- ROOM
- DAFL (unused)
- CODE
- VARI
- FUNC
- STRG
- AUDO

## Usage
The script [examples/dump.rs](examples/dump.rs) is a simple example binary that uses the library to dump assets from a provided data.win.<br>
To run it, do `cargo run --release --example dump` and it will dump from `./data.win` into `./extract/`.

While this is neat and all, this is a LIBRARY, not just a tool for dumping to files. Everything is loaded into memory and can be accessed directly from your Rust code.

I want this library to be very controllable: you should be able to tell it exactly what to load and when to do it. This goal is a WIP, currently the flow goes like this:

```rust
// load and parse the file
// this currently parses all chunks, but doesn't
//   load any images or audio data into memory
// eventually, this will probably get split into a few functions to
//   allow you to control what chunks to parse
let mut data: dr_extract::DataWin = dr_extract::parse("data.win").unwrap();

// access some data (each chunk is an Option<> field inside data)
println!("{}", data.gen8.as_ref().unwrap().name);
println!("{}", data.sond.as_ref().unwrap().sounds[0].name);

// load the spritesheets' images
// normally the spritesheets inside data.txtr contain addresses to the data
// calling this function changes that into a image::DynamicImage
//   from https://github.com/image-rs/image
data.load_spritesheets().unwrap();

// now you can do this:
if let dr_extract::chunk::PNGState::Loaded { texture } = &data.txtr.as_ref().unwrap().spritesheets[0].png {
    // do whatever with the texture
    // ('texture' here is &image::DynamicImage)
}

// load the sprites' images (requires spritesheets to be loaded)
// same concept as with load_spritesheets
// note: there is currently no way to load individual sprites
//   this will be added in the future (eg. load_sprite("spr_dogcar"))
data.load_sprites().unwrap();

// now you can do this:
if let dr_extract::chunk::SpriteState::Loaded { textures } = &data.sprt.as_ref().unwrap().sprites[0].textures {
    // do whatever with the textures
    // ('textures' here is &Vec<image::DynamicImage> since it can be animated)
}
```

## License
TBD
