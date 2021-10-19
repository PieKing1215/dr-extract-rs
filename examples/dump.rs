//! Example program that extracts assets from a data.win file in the working directory.
//! The extracted assets are placed in ./extract/

use std::fs;

extern crate dr_extract;

fn main() {
    println!("Hello example!");

    let mut data = dr_extract::parse("data.win").expect("failed");

    if let Some(gen8) = &data.gen8 {
        println!("Successfully parsed data.win: {} ({})", gen8.name, gen8.display_name);
    }

    // println!("gen8: {:?}", data.gen8);
    // println!("optn: {:?}", data.optn);
    // println!("sond: {:?}", data.sond);
    // println!("sprt: {:?}", data.sprt);
    // println!("tpag: {:?}", data.tpag);
    // println!("txtr: {:?}", data.txtr);

    println!("Loading spritesheets...");
    data.load_spritesheets().expect("Loading spritesheets failed");

    println!("Dumping spritesheets...");
    fs::create_dir_all("extract/spritesheet/").unwrap();
    if let Some(txtr) = &data.txtr {
        for (i, spr) in txtr.spritesheets.iter().enumerate() {
            match &spr.png {
                dr_extract::PNGState::Loaded { texture } => {
                    // println!("{}: {:?}", i, texture.dimensions());
    
                    texture.save(format!("extract/spritesheet/{}.png", i)).unwrap();
                }
                dr_extract::PNGState::Unloaded { .. } => {
                    println!("Spritesheet {} not loaded.", i);
                },
            }
        }
    }

    println!("Loading sprites...");
    data.load_sprites().expect("Loading sprites failed");

    println!("Dumping sprites...");
    fs::create_dir_all("extract/sprite/").unwrap();
    if let Some(sprt) = &data.sprt {
        for spr in &sprt.sprites {
            match &spr.textures {
                dr_extract::SpriteState::Loaded { textures } => {
                    // println!("{}: {:?}", spr.name, textures.len());
    
                    for (i, tex) in textures.iter().enumerate() {
                        tex.save(format!("extract/sprite/{}_{}.png", spr.name, i)).unwrap();
                    }
                }
                dr_extract::SpriteState::Unloaded { .. } => {
                    println!("Sprite {} not loaded.", spr.name);
                },
            }
        }
    }
}