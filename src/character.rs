use std::fmt;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::action::{CharacterAction, CharacterActions};
use crate::common::{Id, Name};
use crate::item::Item;
use crate::stats::{BaseStats, Stat, DerivedStat, DerivedStats, StatBlock};

use crate::encyclopedia::Encyclopedia;
use crate::encyclopedia::read_encyclopedia;


type CharacterStats = Id; // todo, allow literals in JSON with enum
type Items = Vec::<Id>; // todo, allow literals in JSON with CharacterItem

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    id: Id,
    name: Name,
    #[serde(default = "Character::default_base_stats")]
    base_stats: BaseStats,
    #[serde(default = "Character::default_stats")]
    stats: CharacterStats,
    #[serde(default = "Character::default_actions")]
    actions: CharacterActions,
    #[serde(default)]
    items: Items,
    // equips: item::EquipmentSet,
}

impl Character {
    fn default_actions() -> CharacterActions {
        let mut ca = CharacterActions::new();
        ca.insert(String::from("Attack"), CharacterAction::Index(0));
        ca.insert(String::from("Item"), CharacterAction::UseItem);
        ca
    }
    fn default_base_stats() -> BaseStats {
        let mut bs = BaseStats::new();
        bs.insert(String::from("Strength"), 10);
        bs.insert(String::from("Stamina"), 10);
        bs.insert(String::from("Magic"), 10);
        bs.insert(String::from("Speed"), 10);
        bs
    }
    fn default_stats() -> Id {
        0
    }
    pub fn new(id: Id, name: Name) -> Character {
        Character {
            id,
            name,
            base_stats: BaseStats::new(),
            stats: 0, // DerivedStats::new(),
            actions: CharacterActions::new(),
            items: Items::new(),
            //equips: item::generate_equipment_set(),
        }
    }
    pub fn matches(&self, id: Id) -> bool {
        self.id == id
    }
    fn from_json(data: &str) -> Character {
        let c: Character = serde_json::from_str(data).expect("Character JSON was not well-formatted");
        c
    }
    // `&self` is short for `self: &Self`
    // Here `Self` is short for `Character`
    pub fn whoami(&self) -> (Id, &str) {
        (self.id, &self.name[..])
    }
    pub fn get_base_stat(&self, name: Name) -> Option<&Stat> {
        self.base_stats.get(&name)
    }
    pub fn get_stat<'a>(&self, name: Name, statblocks: &'a Encyclopedia<StatBlock>) -> Option<&'a DerivedStat> {
        match statblocks.get(&self.stats) {
            Some(statblock) => statblock.get_stat(name),
            None => None,
        }
    }
    /*
    pub fn equip_to_slot(&mut self, item: item::Item, slot: String) -> Option<item::Item> {
        let prev_equip = self.equips.remove(&slot).unwrap();
        self.equips.insert(slot, Some(item));
        prev_equip
    }
    pub fn unequip_from_slot(&mut self, slot: String) -> Option<item::Item> {
        let prev_equip = self.equips.remove(&slot).unwrap();
        self.equips.insert(slot, None);
        prev_equip
    }*/
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.id, self.name)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let (id, name) = (0, "Mog");
        let mog = Character::new(
            id,
            String::from(name),
        );
        assert!(mog.matches(id));
        assert_eq!(mog.whoami(), (id, "Mog"));
    }
    #[test]
    fn from_json_test() {
        let (id, name) = (0, "Mog");
        let mog_json = format!("{{\"id\": {}, \"name\": \"{}\"}}", id, String::from(name));
        let mog = Character::from_json(&mog_json);
        assert!(mog.matches(id));
        assert_eq!(mog.whoami(), (id, name));
    }
    #[test]
    fn serde_defaults_test() {
        let (id, name) = (0, "Mog");
        let mog_json = format!("{{\"id\": {}, \"name\": \"{}\"}}", id, String::from(name));
        let mog = Character::from_json(&mog_json);
        assert_eq!(mog.actions, Character::default_actions());
        assert_eq!(mog.base_stats, Character::default_base_stats());
        assert_eq!(mog.stats, Character::default_stats())
    }
    #[test]
    fn get_base_stat_test() {
        let mog = Character::from_json(r#"{"id": 0, "name": "Mog"}"#);
        assert!(mog.get_base_stat(String::from("Strength")).is_some());
        assert!(mog.get_base_stat(String::from("Moxie")).is_none());
    }
    #[test]
    fn get_stat_test() {
        let statblocks = read_encyclopedia::<StatBlock>("data/stats.json");
        let mog = Character::from_json(r#"{"id": 0, "name": "Mog"}"#);
        assert!(mog.get_stat(String::from("Strength"), &statblocks).is_some());
        assert!(mog.get_stat(String::from("Offense"), &statblocks).is_some());
        assert!(mog.get_stat(String::from("Moxie"), &statblocks).is_none());
    }
}