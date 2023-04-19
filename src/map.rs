use std::fmt::Write;

use crate::common::*;
use crate::encyclopedia::SpriteCode;

pub struct Map {
    encoded_map: Vec::<Vec::<Id>>,
    sprite_code: SpriteCode,
}

impl Map {
    pub fn render(&self, center: &XY, width: u8, height: u8) -> String {
        let mut rendering = String::new();
        for row in self.encoded_map.iter() {
            for code in row.iter() {
                let sprite = self.sprite_code.get(code).unwrap();
                write!(&mut rendering, "{}", sprite).unwrap();
            }
            write!(&mut rendering, "\n").unwrap();
        }
        rendering
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
            (0, ' '),
            (1, '-'),
        ]);
        let sprite_code = SpriteCode {
            en: _sprite_code,
        };
        let test_map = Map {
            encoded_map,
            sprite_code,
        };
/*        expected =
            "------- \
             -  x  - \
             -------";
*/
        let expected =
            "---\n\
             - -\n\
             ---\n";
        let rendering = test_map.render((0, 0), 0, 0);
        assert_eq!(rendering, expected);
    }
}
