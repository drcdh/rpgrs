use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

use crate::common::*;

pub type _Encyclopedia<T> = HashMap<Id, T>;
pub struct Encyclopedia<T> {
    pub en: _Encyclopedia<T>,
}

fn _read_encyclopedia<T: Serialize + DeserializeOwned>(
    filename: &str,
) -> Result<_Encyclopedia<T>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let en = serde_json::from_reader(reader)?;
    Ok(en)
}

impl<T: Serialize + DeserializeOwned> Encyclopedia<T> {
    pub fn new(filename: &str) -> Encyclopedia<T> {
        Encyclopedia {
            en: _read_encyclopedia::<T>(filename).expect(&*format!("Failed to read encyclopedia from {filename}")),
        }
    }
}

impl<T> Encyclopedia<T> {
    pub fn is_empty(&self) -> bool {
        self.en.is_empty()
    }
    pub fn len(&self) -> usize {
        self.en.len()
    }
    pub fn get(&self, id: &Id) -> Option<&T> {
        self.en.get(id)
    }
    //pub fn iter(&self) -> Iter<'_, Id, T> { self.en.iter() }
    pub fn resolve<'a>(&'a self, iol: &'a IndexedOrLiteral<T>) -> Option<&'a T> {
        match iol {
            IndexedOrLiteral::<T>::Index(i) => self.en.get(i),
            IndexedOrLiteral::<T>::Literal(c) => Some(c),
        }
    }
    /*    pub fn resolve_mut<'a>(&'a mut self, iol: &'a IndexedOrLiteral::<T>) -> Option<&'a mut T> {
        match iol {
            IndexedOrLiteral::<T>::Index(i) => self.en.get_mut(&i),
            IndexedOrLiteral::<T>::Literal(c) => Some(c),
        }
    }*/
}

impl<T: Clone> Encyclopedia<T> {
    pub fn clone_entry(&self, iol: &IndexedOrLiteral<T>) -> Option<T> {
        self.resolve(iol).cloned()
    }
}

use crate::action::Action;
use crate::character::Character;
use crate::condition::Condition;
use crate::effect::Effect;
use crate::item::Item;
use crate::sprite::Sprite;
use crate::stats::StatBlock;

pub type ActionEncyclopedia = Encyclopedia<Action>;
pub type CharacterEncyclopedia = Encyclopedia<Character>;
pub type ConditionEncyclopedia = Encyclopedia<Condition>;
pub type EffectEncyclopedia = Encyclopedia<Effect>;
pub type ItemEncyclopedia = Encyclopedia<Item>;
pub type StatBlockEncyclopedia = Encyclopedia<StatBlock>;

pub type SpriteEncyclopedia = Encyclopedia<Sprite>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_action_encyclopedia_test() {
        assert!(!ActionEncyclopedia::new("data/actions.json").is_empty());
    }
    #[test]
    fn read_character_encyclopedia_test() {
        assert!(!CharacterEncyclopedia::new("data/characters.json").is_empty());
    }
    #[test]
    fn read_condition_encyclopedia_test() {
        assert!(!ConditionEncyclopedia::new("data/conditions.json").is_empty());
    }
    #[test]
    fn read_effect_encyclopedia_test() {
        assert!(!EffectEncyclopedia::new("data/effects.json").is_empty());
    }
    #[test]
    fn read_item_encyclopedia_test() {
        assert!(!ItemEncyclopedia::new("data/items.json").is_empty());
    }
    #[test]
    fn read_statblocks_encyclopedia_test() {
        assert!(!StatBlockEncyclopedia::new("data/stats.json").is_empty());
    }
}
