<h1>dr-extract<br>
    <a href="https://github.com/PieKing1215/dr-extract-rs/actions/workflows/rust_build_test.yml"><img alt="GitHub Workflow Status" src="https://img.shields.io/github/workflow/status/PieKing1215/dr-extract-rs/Rust%20Build+Test"></a>
</h1>

A WIP Rust library for parsing and extracting assets from [DELTARUNE](https://deltarune.com)'s data.win.

This is not just a dumping tool, everything is loaded into memory and can be accessed directly from your Rust code.

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

While this is neat and all, this is a *library*, not just a tool for dumping to files.

I want this library to be very controllable: you should be able to tell it exactly what to load and when to do it.<br>This goal is a WIP: currently you control when to parse each individual chunk, and when to load assets (ie. image data) for individual chunks that have assets (currently TXTR, SPRT; eventually SOND + more?)

After a chunk is parsed, you can access the parsed data as a pretty simple set of structs. 

When you load the assets for TXTR/SPRT, the texture(s) are loaded into memory as `image::DynamicImage` from the [image crate](https://github.com/image-rs/image), and can be used by your program.

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
