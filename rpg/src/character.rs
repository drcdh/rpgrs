use std::fmt;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::action::{CharacterAction, CharacterActions};
use crate::common::{Id, Name};
use crate::item::Item;
use crate::stats::{BaseStats, Stat, generate_stats};


type Items = Vec::<Id>; // todo, allow literals in JSON with CharacterItem

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    id: Id,
    name: Name,
    #[serde(default = "Character::default_base_stats")]
    base_stats: BaseStats,
    #[serde(default = "Character::default_stats")]
    stats: Id,
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
        let (base_stats, stats) = generate_stats();
        Character {
            id,
            name,
            base_stats,
            stats: 0, //stats,
            actions: CharacterActions::new(),
            items: Items::new(),
            //equips: item::generate_equipment_set(),
        }
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
    pub fn get_stat(&self, name: Name) -> Stat {
        let (_bsset, dsset) = generate_stats();
        match dsset.get(&name) { //self.stats.get(&name) {
            Some(ds) => 1,//ds(&self.base_stats, &self.equips),
            None => 0, // TODO!
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
        let name = "Mog";
        let mog = Character::new(
            0,
            String::from(name),
        );
        assert_eq!(mog.whoami(), (0, "Mog"));
    }
    #[test]
    fn from_json_test() {
        let mog_json = r#"{
            "id": 0,
            "name": "Mog",
            "base_stats": {
                "Strength": 10,
                "Stamina": 12
            },
            "stats": 0
        }"#;
        let mog = Character::from_json(mog_json);
        assert_eq!(mog.whoami(), (0, "Mog"));
    }
}
