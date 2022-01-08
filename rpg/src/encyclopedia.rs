use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json;

use crate::common::Id;


pub type Encyclopedia<T> = HashMap::<Id, T>;

pub fn get_encyclopedia<T: Serialize + DeserializeOwned>(filename: &str) -> Result<Encyclopedia<T>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let en = serde_json::from_reader(reader)?;
    Ok(en)
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;
    use crate::character::Character;
    use crate::effect::Effect;
    use crate::item::Item;

    #[test]
    fn action_encyclopedia_test() {
        let filename = "data/actions.json";
        let actions = get_encyclopedia::<Action>(&filename).expect(format!("Could not parse {}", &filename).as_str());
        assert_ne!(actions.len(), 0);
    }
    #[test]
    fn character_encyclopedia_test() {
        let filename = "data/characters.json";
        let characters = get_encyclopedia::<Character>(&filename).expect(format!("Could not parse {}", &filename).as_str());
        //println!(">>> CHARACTERS <<<\n{}", characters);
        assert_ne!(characters.len(), 0);
    }
    #[test]
    fn effect_encyclopedia_test() {
        let filename = "data/effects.json";
        let effects = get_encyclopedia::<Effect>(&filename).expect(format!("Could not parse {}", &filename).as_str());
        //println!(">>> EFFECTS <<<\n{}", effects);
        assert_ne!(effects.len(), 0);
    }
    #[test]
    fn item_encyclopedia_test() {
        let filename = "data/items.json";
        let items = get_encyclopedia::<Item>(&filename).expect(format!("Could not parse {}", &filename).as_str());
//        println!(">>> ITEMS <<<\n{}", items);
        assert_ne!(items.len(), 0);
    }
}
