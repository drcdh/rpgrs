use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::action::Action;
use crate::character::Character;
use crate::effect::Effect;
use crate::item::Item;


#[derive(Serialize, Deserialize, Debug)]
pub struct Encyclopedia<T: Serialize> {
    listing: Vec::<T>,
}

//type Encyclopedia<T: Serialize + DeserializeOwned> = HashMap::<String, Vec::<T>>;

impl<T: fmt::Display + Serialize> fmt::Display for Encyclopedia<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(&self.listing).ok().unwrap())
    }
}

pub type ActionEncyclopedia = Encyclopedia<Action>;
pub fn get_action_encyclopedia(filename: &str) -> Result<ActionEncyclopedia, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let en = serde_json::from_reader(reader)?;
    Ok(en)
}

pub type CharacterEncyclopedia = Encyclopedia<Character>;
pub fn get_character_encyclopedia(filename: &str) -> Result<CharacterEncyclopedia, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let en = serde_json::from_reader(reader)?;
    Ok(en)
}

pub type EffectEncyclopedia = Encyclopedia<Effect>;
pub fn get_effect_encyclopedia(filename: &str) -> Result<EffectEncyclopedia, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let en = serde_json::from_reader(reader)?;
    Ok(en)
}

pub type ItemEncyclopedia = Encyclopedia<Item>;
pub fn get_item_encyclopedia(filename: &str) -> Result<ItemEncyclopedia, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let en = serde_json::from_reader(reader)?;
    Ok(en)
}
/*
pub fn get_encyclopedia<T: Serialize + DeserializeOwned>(filename: &str) -> Result<Encyclopedia<T>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let en = serde_json::from_reader(reader)?;
    Ok(en)
}
*/
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn action_encyclopedia_test() {
        let filename = "data/actions.json";
        let actions = get_action_encyclopedia(filename).expect("Could not parse encyclopedia file");
        println!(">>> ACTIONS <<<\n{}", actions);
    }
    #[test]
    fn character_encyclopedia_test() {
        let filename = "data/characters.json";
        let characters = get_character_encyclopedia(filename).expect("Could not parse encyclopedia file");
        println!(">>> CHARACTERS <<<\n{}", characters);
    }
    #[test]
    fn effect_encyclopedia_test() {
        let filename = "data/effects.json";
        let effects = get_effect_encyclopedia(filename).expect("Could not parse encyclopedia file");
        println!(">>> EFFECTS <<<\n{}", effects);
    }
    #[test]
    fn item_encyclopedia_test() {
        let filename = "data/items.json";
        let items = get_item_encyclopedia(filename).expect("Could not parse encyclopedia file");
        println!(">>> ITEMS <<<\n{}", items);
    }
}
