use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use serde_json::json;

use rpgrs::common::*;
use rpgrs::encyclopedia::_Encyclopedia;
use rpgrs::map::Map;
use rpgrs::sprite::Sprite;


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let map_filepath = &args[1];
//    let map_outpath = args[2];

    // Note use of internal type _Encyclopedia
    let mut decoder = _Encyclopedia::<Sprite>::new();
    let mut encoder = HashMap::<char, Id>::new();

    let mut encoded_map = Vec::<Vec::<Id>>::new();
    let mut width = 0;
    if let Ok(lines) = read_lines(map_filepath) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let mut encoded_row = Vec::<Id>::new();
            if let Ok(row) = line {
                if row.len() > width {
                    width = row.len();
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
            encoded_map.push(encoded_row);
        }
    }
/*    let map = Map {
        dim: (width, encoded_rows.len()),
        origin: (0, 0),
        encoded_map,
        sprite_code: SpriteEncyclopedia{en: decoder},
    };
    let map_json = json!(map);
    write!(map_json);*/
    for encoded_row in encoded_map.iter() {
        println!("{:?}", encoded_row);
    }
    let sprites_json = json!(decoder);
    print!("{}", sprites_json);
}
