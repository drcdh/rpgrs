use std::error::Error;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::common::*;
use crate::encyclopedia::_Encyclopedia;
use crate::sprite::Sprite;

pub type _EncodedMapLayer = Vec<Vec<Id>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncodedMapLayer {
    pub layer: _EncodedMapLayer,
}
impl EncodedMapLayer {
    pub fn new() -> EncodedMapLayer {
        EncodedMapLayer {
            layer: _EncodedMapLayer::new(),
        }
    }
}
pub type MapLayers = Vec::<EncodedMapLayer>;
#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub dim: uXY,
    pub origin: XY,
    pub layers: MapLayers,
//    pub encoded_map: _EncodedMapLayer,
    pub sprite_code: _Encyclopedia::<Sprite>,
}

impl Map {
/*    pub fn from_files(encoded_map_filepath: &str, sprite_decoder_filename: &str) -> Map {
        let encoded_map_json = std::fs::read_to_string(&encoded_map_filepath).expect("Failed to read map");
        let encoded_map = serde_json::from_str::<_EncodedMapLayer>(&encoded_map_json).unwrap();
        let sprite_code = SpriteEncyclopedia::new(sprite_decoder_filename);
        Map {
            dim: (encoded_map[0].len().try_into().unwrap(), encoded_map.len().try_into().unwrap()),
            origin: (0, 0),
            encoded_map,
            sprite_code,
        }
    }*/
    pub fn from_json(filename: &str) -> Map {
        let file = File::open(filename).expect("Could not open serialized map file");
        let reader = BufReader::new(file);
        let map = serde_json::from_reader(reader).expect("Could not deserialize map");
        map
    }
    pub fn decode_sprite(&self, code: Id) -> Option<&Sprite> {
        self.sprite_code.get(&code)
    }
    pub fn sprite_at_loc(&self, i: usize, j: usize, k: usize) -> Option<&Sprite> {
        let code = self.layers[k].layer[j][i];
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
        Map::from_json("./data/maps/tower/map.json");
    }

    #[test]
    fn deserialize_layer_test() {
        let data = "\
        { \"layer\": [\
            [1, 2, 3],\
            [4, 5, 6]\
        ]}";
        let l: EncodedMapLayer = serde_json::from_str(data).expect("EncodedMapLayer JSON not well-formatted");
        assert_eq!(l.layer[0][2], 3);
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
