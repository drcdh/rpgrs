use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_json;
use serde_json::Value;

use crate::common::{Id, Name};
use crate::item;
use crate::stats;

type Actions = serde_json::Map<Name, Value>;// HashMap::<Name, Id>;
type Items = Vec::<Value>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    pub id: Id, // todo, make private (impacts party.rs)
    name: Name,

    base_stats: stats::BaseStats,
    stats: Id, // stats::DerivedStats,
    actions: Actions,
    items: Items, //Vec::<item::Item>,
    // equips: item::EquipmentSet,
}

impl Character {
    pub fn new(id: Id, name: Name) -> Character {
        let (base_stats, stats) = stats::generate_stats();
        Character {
            id,
            name,
            base_stats,
            stats: 0, //stats,
            actions: Actions::new(),
            items: Items::new(),
            //equips: item::generate_equipment_set(),
        }
    }
    fn from_json(data: &str) -> Character {
        let c: Character = serde_json::from_str(data).expect("Character JSON was not well-formatted");
        c
/*
        let id = json["id"].as_u64().expect("No id in JSON");
        let name = json["name"].as_str().expect("No name in JSON").to_string();
        let (mut base_stats, stats) = stats::generate_stats();
        if let Some(new_base_stats) = json["base_stats"].as_array() {
            // update base_stats
        }
        let actions: Actions = *json["actions"].as_object().expect("No actions in JSON");
        let items = *json["items"].as_array().expect("No items in JSON");
        Character {
            id,
            name,
            base_stats,
            stats: 0, // todo
            actions,
            items,
        }
*/
    }
    // `&self` is short for `self: &Self`
    // Here `Self` is short for `Character`
    pub fn whoami(&self) -> &str {
        &self.name[..]
    }
    pub fn get_stat(&self, name: Name) -> stats::Stat {
        let (_bsset, dsset) = stats::generate_stats();
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
        assert_eq!(mog.id, 0);
        assert_eq!(mog.whoami(), name);
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
            "stats": 0,
            "actions": {
                "Attack": 0,
                "Dance": 30,
                "Magic": 1,
                "Item": 2
            },
            "items": [
                0,0,5
            ]
        }"#;
        let mog = Character::from_json(mog_json);//.expect("Mog JSON was not well-formatted");
        assert_eq!(mog.id, 0);
        assert_eq!(mog.whoami(), "Mog");
    }
}
