<h1>dr-extract<br>
    <a href="https://github.com/PieKing1215/dr-extract-rs/actions/workflows/rust_build_test.yml"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/PieKing1215/dr-extract-rs/Rust%20Build+Test"></a>
</h1>

A WIP Rust library for flexibly parsing and extracting assets from [DELTARUNE](https://deltarune.com)'s data.win.

This is not just a dumping tool, everything is loaded into memory and can be accessed directly from your Rust code.

You must provide your own data.win & audiogroup1.dat file. I will not host them myself, nor will I host any extracted assets here.

It might work for other GameMaker 2 games, but I have not tested anything other than the DELTARUNE Chapter 1&2 Demo.

Based on the documentation here: https://pcy.ulyssis.be/undertale/unpacking-corrected<br>
(+ some adjustments for the newer version of GM)

Currently the game metadata, sounds, fonts, spritesheets, and sprites are fully extractable.<br>
More specifically, these chunks are supported right now:
- GEN8
- OPTN
- SOND
- SPRT
- TPAG
- TXTR
- AUDO
- FONT

Not supported right now:
- EXTN (unused)
- AGRP
- BGND
- PATH
- SCPT
- SHDR (unused)
- TMLN
- OBJT
- ROOM
- DAFL (unused)
- CODE
- VARI
- FUNC
- STRG

## Usage
The script [examples/dump.rs](examples/dump.rs) is a simple example binary that uses the library to dump assets from a provided data.win & audiogroup1.dat.<br>
To run it, do `cargo run --release --example dump` and it will dump from `./data.win` & `./audiogroup1.dat` into `./extract/`.

While this is neat and all, this is a *library*, not just a tool for dumping to files.

I want this library to be very controllable: you should be able to tell it exactly what to load and when to do it.<br>This goal is a WIP: currently you control when to parse each individual chunk, and when to load assets (ie. image/audio data) for individual chunks that have assets (currently TXTR, SPRT, SOND, FONT; eventually more?). For sprites and sounds, you can also choose to load the image/audio data for only certain sprites/sounds (by name).

After a chunk is parsed, you can access the parsed data as a pretty simple set of structs. 

When you load the assets for TXTR/SPRT/FONT, the texture(s) are loaded into memory as `image::DynamicImage` from the [image crate](https://github.com/image-rs/image), and can be used by your program.

When you load the assets for SOND/AUDO, the audio data is loaded into memory as a `Vec<u8>` if the audio is embedded in the data.win, otherwise you can use the sound's `file` field to locate the external file. The `Vec<u8>` is the raw file data for the embedded file, so you can literally just dump the bytes directly to an .ogg file, or you can use a library to parse the audio in-memory.

See [examples/simple.rs](examples/simple.rs) for an example of the logic flow.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
