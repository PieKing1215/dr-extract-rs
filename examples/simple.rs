use dr_extract::{DataWin, DataWinReady, chunk::{PNGState, SpriteState, SpritesheetEntry}};


extern crate dr_extract;

fn main() {
    println!("Example \"simple\"...");
    
    // set the file up for reading
    // can also use prepare_bytes(Vec<u8>) instead
    let data_ready: DataWinReady = dr_extract::prepare_file("data.win").unwrap();

    // load the chunk names and positions
    let mut data: DataWin = data_ready.fetch_chunks().unwrap();

    // parse a chunk and print some of its data
    data.parse_gen8().unwrap();
    println!("name = {}, display_name = {}", data.gen8.as_ref().unwrap().name, data.gen8.as_ref().unwrap().display_name);

    // parse the spritesheet chunk
    data.parse_txtr().unwrap();
    println!("# of spritesheets = {}", data.txtr.as_ref().unwrap().spritesheets.len());

    // prints unloaded
    match data.txtr.as_ref().unwrap().spritesheets[0].png {
        PNGState::Loaded{..} => println!("sheet #0 image data is loaded"),
        PNGState::Unloaded{..} => println!("sheet #0 image data is unloaded"),
    }

    // load spritesheet image data
    // requires the TXTR chunk to have been parsed already (parse_txtr)
    data.load_spritesheets().unwrap();
    
    // now prints loaded
    match data.txtr.as_ref().unwrap().spritesheets[0].png {
        PNGState::Loaded{..} => println!("sheet #0 image data is loaded"),
        PNGState::Unloaded{..} => println!("sheet #0 image data is unloaded"),
    }

    // parse the sprites chunk
    data.parse_sprt().unwrap();
    println!("# of sprites = {}", data.sprt.as_ref().unwrap().sprites.len());

    // prints unloaded
    match data.sprt.as_ref().unwrap().sprites[0].textures {
        SpriteState::Loaded{..} => println!("sprite #0 image data is loaded"),
        SpriteState::Unloaded{..} => println!("sprite #0 image data is unloaded"),
    }

    // load sprite image data
    // requires the SPRT chunk to have been parsed already (parse_sprt)
    // requires load_spritesheets to have been called already
    data.load_sprites().unwrap();
    
    // now prints loaded
    match data.sprt.as_ref().unwrap().sprites[0].textures {
        SpriteState::Loaded{..} => println!("sprite #0 image data is loaded"),
        SpriteState::Unloaded{..} => println!("sprite #0 image data is unloaded"),
    }

    println!("Done!");
}