use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, BufRead, Write};
use std::path::Path;
use std::path::PathBuf;

use glob::glob;
use serde_json::json;

use rpgrs::common::*;
use rpgrs::encyclopedia::{_Encyclopedia, SpriteEncyclopedia};
use rpgrs::map::{EncodedMapLayer, Map};
use rpgrs::sprite::Sprite;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_layer(
    filepath: PathBuf,
    decoder: &mut _Encyclopedia::<Sprite>,
    encoder: &mut HashMap::<char, Id>,
    width: &mut usize
) -> EncodedMapLayer {
    println!("Reading from {}...", filepath.display());
    let mut encoded_layer = EncodedMapLayer::new();
    if let Ok(lines) = read_lines(filepath) {
        for line in lines {
            let mut encoded_row = Vec::<Id>::new();
            if let Ok(row) = line {
                if row.len() > *width {
                    *width = row.len();
                }
                for c in row.chars() {
                    if let Some(id) = encoder.get(&c) {
                        encoded_row.push(*id);
                    } else {
                        let id = encoder.len() as Id;
                        encoder.insert(c, id);
                        decoder.insert(id, Sprite::new_solid(c));
                        encoded_row.push(id);
                    }
                }
            }
            encoded_layer.layer.push(encoded_row);
        }
    }
    encoded_layer
}

fn write_encoded_layer(
    layer: &EncodedMapLayer,
    dir: &str,
    z: usize,
) {
    let path = format!("{}/{}.encodedmaplayer", dir, z);
    //let mut file = File::create(path).expect("Could not open file for EncodedMapLayer");
    let j = serde_json::to_string(layer).expect("Could not serialize EncodedMapLayer");
    fs::write(&path, j).expect("Could not write serialized EncodedMapLayer");
    println!("Wrote encoded layer {} to {}.", z, path);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let map_dir = &args[1];
//    let map_outpath = args[2];

    // Note use of internal type _Encyclopedia
    let mut decoder = _Encyclopedia::<Sprite>::new();
    let mut encoder = HashMap::<char, Id>::new();
    let mut width: usize = 0;
    let mut layers = Vec::<EncodedMapLayer>::new();
    let layers_glob = format!("{}/*.layer", map_dir);
    println!("Looking for map layers in {}", layers_glob);
    for entry in glob(&layers_glob).expect("Failed to read layer glob patter") {
        match entry {
            Ok(path) => layers.push(read_layer(path, &mut decoder, &mut encoder, &mut width)),
            Err(e) => println!("{:?}", e),
        }
    }
    for (z, layer) in layers.iter().enumerate() {
        write_encoded_layer(layer, &map_dir, z);
    }
/*    let map = Map {
        dim: (width, encoded_rows.len()),
        origin: (0, 0),
        encoded_map,
        sprite_code: SpriteEncyclopedia{en: decoder},
    };
    let map_json = json!(map);
    write!(map_json);*/
/*    for encoded_row in encoded_map.iter() {
        println!("{:?}", encoded_row);
    }*/
    let sprites_path = format!("{}/sprites.json", map_dir);
    let sprites_json = serde_json::to_string(&encoder).expect("Could not serialize encoder");
    fs::write(&sprites_path, sprites_json).expect("Could not write serialized encoder");
    println!("Wrote sprites encoder to {}.", sprites_path);

    let map = Map {
        dim: (width as uCoord, layers[0].layer.len() as uCoord),
        origin: (0, 0),
        layers,
        sprite_code: decoder,
    };
    let map_path = format!("{}/map.json", map_dir);
    let map_json = serde_json::to_string(&map).expect("Could not serialize map");
    fs::write(&map_path, map_json).expect("Could not write serialized map");
    println!("Wrote map to {}.", map_path);
}
