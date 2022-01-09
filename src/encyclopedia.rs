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

fn _read_encyclopedia<T: Serialize + DeserializeOwned>(filename: &str) -> Result<Encyclopedia<T>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let en = serde_json::from_reader(reader)?;
    Ok(en)
}

pub fn read_encyclopedia<T: Serialize + DeserializeOwned>(filename: &str) -> Encyclopedia<T> {
    _read_encyclopedia::<T>(filename).expect(format!("Failed to read encyclopedia from {}", filename).as_str())
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::action::Action;
    use crate::character::Character;
    use crate::effect::Effect;
    use crate::item::Item;
    use crate::stats::StatBlock;

    #[test]
    fn read_action_encyclopedia_test() {
        let filename = "data/actions.json";
        let actions = read_encyclopedia::<Action>(filename);
        assert_ne!(actions.len(), 0);
    }
    #[test]
    fn read_character_encyclopedia_test() {
        let filename = "data/characters.json";
        let characters = read_encyclopedia::<Character>(filename);
        assert_ne!(characters.len(), 0);
    }
    #[test]
    fn read_effect_encyclopedia_test() {
        let filename = "data/effects.json";
        let effects = read_encyclopedia::<Effect>(filename);
        assert_ne!(effects.len(), 0);
    }
    #[test]
    fn read_item_encyclopedia_test() {
        let filename = "data/items.json";
        let items = read_encyclopedia::<Item>(filename);
        assert_ne!(items.len(), 0);
    }
    #[test]
    fn read_statblocks_encyclopedia_test() {
        let filename = "data/stats.json";
        let statblocks = read_encyclopedia::<StatBlock>(filename);
        assert_ne!(statblocks.len(), 0);
    }
}
