//! Example program that extracts assets from a data.win file in the working directory.
//! The extracted assets are placed in ./extract/

use std::{fs, io::Write, path::Path, time::Instant};

use image::GenericImageView;

extern crate dr_extract;

fn main() {
    println!("Example \"dump\"...");

    let mut data = dr_extract::prepare_file("data.win", vec!["audiogroup1.dat"]).expect("load_file failed")
        .fetch_chunks().expect("fetch_chunks failed");

    data.parse_gen8().expect("parse_gen8 failed");

    if let Some(gen8) = &data.gen8 {
        println!("Successfully parsed data.win: {} ({})", gen8.name, gen8.display_name);
    }

    // println!("gen8: {:?}", data.gen8);
    // println!("optn: {:?}", data.optn);
    // println!("sond: {:?}", data.sond);
    // println!("sprt: {:?}", data.sprt);
    // println!("tpag: {:?}", data.tpag);
    // println!("txtr: {:?}", data.txtr);

    println!("Parsing spritesheets...");
    data.parse_txtr().expect("parse_txtr failed");

    println!("Loading spritesheets...");
    let start = Instant::now();
    data.load_spritesheets().expect("Loading spritesheets failed");
    println!("Took {} ms", Instant::now().saturating_duration_since(start).as_millis());

    println!("Dumping spritesheets...");
    fs::create_dir_all("extract/spritesheet/").unwrap();
    if let Some(txtr) = &data.txtr {
        for (i, spr) in txtr.spritesheets.iter().enumerate() {
            match &spr.png {
                dr_extract::chunk::PNGState::Loaded { texture } => {
                    // println!("{}: {:?}", i, texture.dimensions());
    
                    texture.save(format!("extract/spritesheet/{}.png", i)).unwrap();
                }
                dr_extract::chunk::PNGState::Unloaded { .. } => {
                    println!("Spritesheet {} not loaded.", i);
                },
            }
        }
    }

    println!("Parsing sprites...");
    data.parse_sprt().expect("parse_sprt failed");

    println!("Loading sprites...");
    let start = Instant::now();
    data.load_sprites().expect("Loading sprites failed");
    // data.load_sprite("spr_kris_fall_smear").expect("Loading sprite failed");
    println!("Took {} ms", Instant::now().saturating_duration_since(start).as_millis());

    println!("Dumping sprites...");
    fs::create_dir_all("extract/sprite/").unwrap();
    if let Some(sprt) = &data.sprt {
        for (name, spr) in &sprt.sprites {
            match &spr.textures {
                dr_extract::chunk::SpriteState::Loaded { textures } => {
                    if textures.len() == 1 {
                        textures[0].save(format!("extract/sprite/{}.png", name)).unwrap();
                    }else{
                        for (i, tex) in textures.iter().enumerate() {
                            tex.save(format!("extract/sprite/{}_{}.png", name, i)).unwrap();
                        }
                    }
                }
                dr_extract::chunk::SpriteState::Unloaded { .. } => {},
            }
        }
    }

    println!("Parsing backgrounds...");
    data.parse_bgnd().expect("parse_bgnd failed");

    println!("Loading backgrounds...");
    let start = Instant::now();
    data.load_backgrounds().expect("Loading backgrounds failed");
    println!("Took {} ms", Instant::now().saturating_duration_since(start).as_millis());

    println!("Dumping backgrounds...");
    fs::create_dir_all("extract/background/").unwrap();
    if let Some(bgnd) = &data.bgnd {
        for (name, bg) in &bgnd.backgrounds {
            match &bg.texture {
                dr_extract::chunk::BackgroundState::Loaded { texture } => {
                    texture.save(format!("extract/background/{}.png", name)).unwrap();
                }
                dr_extract::chunk::BackgroundState::Unloaded { .. } => {},
            }
        }
    }

    println!("Parsing sounds...");
    data.parse_sond().expect("parse_sond failed");

    println!("Parsing audio data...");
    data.parse_audo().expect("parse_audo failed");

    println!("Loading sound data...");
    let start = Instant::now();
    data.load_sounds().expect("Loading sounds failed");
    println!("Took {} ms", Instant::now().saturating_duration_since(start).as_millis());

    println!("Dumping sounds...");
    fs::create_dir_all("extract/sound/").unwrap();
    if let Some(sond) = &data.sond {
        for (name, sound) in &sond.sounds {
            if let Some(audio) = &sound.audio_data {
                match audio {
                    dr_extract::chunk::AudioType::Internal(data) => {
                        let mut f = fs::OpenOptions::new().create(true).append(true).open(format!("extract/sound/{}.ogg", name)).expect("File open failed");
                        f.write_all(data).unwrap();
                    },
                    dr_extract::chunk::AudioType::External => {
                        println!("{} has external audio: {}", name, sound.file);
                    },
                }
            }
        }
    }
    
    println!("Parsing fonts...");
    data.parse_font().expect("parse_font failed");

    println!("Loading fonts...");
    let start = Instant::now();
    data.load_fonts().expect("Loading fonts failed");

    println!("Took {} ms", Instant::now().saturating_duration_since(start).as_millis());

    println!("Dumping fonts...");
    fs::create_dir_all("extract/font/").unwrap();
    if let Some(font) = &data.font {
        for (name, fnt) in &font.fonts {
            fs::create_dir_all(format!("extract/font/{}/", name)).unwrap();

            for (char, gly) in &fnt.glyphs {
                if let Some(tex) = &gly.texture {
                    if tex.width() > 0 && tex.height() > 0 {
                        tex.save(format!("extract/font/{}/{}.png", name, char)).unwrap();
                    } else {
                        println!("{} has 0 size", format!("extract/font/{}/{}.png", name, char));
                    }
                }
            }
        }
    }

    println!("Done!");
}