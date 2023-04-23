use std::fmt::Write;

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
    fn render_test() {
        let encoded_map = vec![vec![1; 3], vec![1, 0, 1], vec![1; 3]];
        let _sprite_code = HashMap::from([
            (
                0,
                Sprite {
                    frames: vec![' '],
                    period: 1,
                },
            ),
            (
                1,
                Sprite {
                    frames: vec!['-'],
                    period: 1,
                },
            ),
        ]);
        let sprite_code = SpriteEncyclopedia { en: _sprite_code };
        let test_map = Map {
            encoded_map,
            sprite_code,
        };
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
    }
}
