use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::common::*;
use crate::encyclopedia::SpriteEncyclopedia;
use crate::sprite::Sprite;

pub struct Map {
    pub dim: uXY,
    pub origin: XY,
    pub encoded_map: Vec<Vec<Id>>,
    pub sprite_code: SpriteEncyclopedia,
}

impl Map {
    pub fn from_files(encoded_map_filepath: &str, sprite_decoder_filename: &str) -> Map {
        let encoded_map_json = std::fs::read_to_string(&encoded_map_filepath).expect("Failed to read map");
        let encoded_map = serde_json::from_str::<Vec<Vec<Id>>>(&encoded_map_json).unwrap();
        let sprite_code = SpriteEncyclopedia::new(sprite_decoder_filename);
        Map {
            dim: (encoded_map[0].len().try_into().unwrap(), encoded_map.len().try_into().unwrap()),
            origin: (0, 0),
            encoded_map,
            sprite_code,
        }
    }
    pub fn decode_sprite(&self, code: Id) -> Option<&Sprite> {
        self.sprite_code.get(&code)
    }
    pub fn sprite_at_loc(&self, i: usize, j: usize) -> Option<&Sprite> {
        let code = self.encoded_map[j][i];
        self.sprite_code.get(&code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::fs;

    #[test]
    fn read_test() {
        Map::from_files("./data/maps/tower.encoded", "./data/maps/tower.sprites");
    }
/*    #[test]
    fn render_test() {
        let encoded_map = vec![vec![1; 3], vec![1, 0, 1], vec![1; 3]];
        let _sprite_code = HashMap::from([
            (
                0,
                Sprite {
                    frames: vec![' '],
                    period: 1,
                    offset: 0,
                },
            ),
            (
                1,
                Sprite {
                    frames: vec!['-'],
                    period: 1,
                    offset: 0,
                },
            ),
        ]);
        let sprite_code = SpriteEncyclopedia { en: _sprite_code };
        let test_map = Map::from_files("../data/maps/tower.encoded", "../data/maps/tower.sprites");
        /*        expected =
                    "------- \
                     -  x  - \
                     -------";
        */
        let expected = "---\n\
             - -\n\
             ---\n";
        let rendering = test_map.render((0, 0), 0, 0);
        assert_eq!(rendering, expected);
    }*/
}
