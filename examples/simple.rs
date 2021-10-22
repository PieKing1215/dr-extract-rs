use dr_extract::{DataWin, DataWinReady, chunk::{PNGState, SpriteState, SpritesheetEntry}};


extern crate dr_extract;

fn main() {
    println!("Example \"simple\"...");
    
    // set the file up for reading
    // can also use prepare_bytes(Vec<u8>) instead
    let data_ready: DataWinReady = dr_extract::prepare_file("data.win", vec!["audiogroup1.dat"]).unwrap();

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
    match data.sprt.as_ref().unwrap().sprites.values().next().unwrap().textures {
        SpriteState::Loaded{..} => println!("sprite #0 image data is loaded"),
        SpriteState::Unloaded{..} => println!("sprite #0 image data is unloaded"),
    }

    // load image data for all sprites
    // alternatively, you could use data.load_sprite(String) to load individual sprites
    // requires the SPRT chunk to have been parsed already (parse_sprt)
    // requires load_spritesheets to have been called already
    data.load_sprites().unwrap();
    
    // now prints loaded
    match data.sprt.as_ref().unwrap().sprites.values().next().unwrap().textures {
        SpriteState::Loaded{..} => println!("sprite #0 image data is loaded"),
        SpriteState::Unloaded{..} => println!("sprite #0 image data is unloaded"),
    }
    // this allows you to read sprite images like this:
    match &data.sprt.as_ref().unwrap().sprites.get("spr_krisplace").unwrap().textures {
        SpriteState::Unloaded { texture_count, texture_addresses } => {},
        SpriteState::Loaded { textures } => {},
    }

    // can also load sounds like this:
    data.parse_sond().unwrap();
    data.parse_audo().unwrap();

    data.load_sounds().unwrap();
    // or load individual sounds with data.load_sound(String)

    println!("# of sounds = {}", data.sond.as_ref().unwrap().sounds.len());

    // now you can access the bytes of a sound file like this:
    let sound = data.sond.as_ref().unwrap().sounds.get("snd_heartshot_dr_b").unwrap();
    match sound.audio_data.as_ref().unwrap() {
        dr_extract::chunk::AudioType::Internal(bytes) => {
            println!("audio file is {} bytes", bytes.len());
        },
        dr_extract::chunk::AudioType::External => { /* sound.file contains the name of the external ogg file */},
    }

    println!("Done!");
}