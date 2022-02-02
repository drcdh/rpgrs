use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::common::*;
use crate::effect::Effect;


#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    id: Id,
    #[serde(default)]
    pub name: Name,
    #[serde(default)]
    pub power: i32,
    #[serde(default)]
    pub stamina_mod: i32,
    #[serde(default)]
    pub strength_mod: i32,
    effect: IndexedOrLiteral<Effect>,
}

impl Item {
    pub fn get_modifier(&self, name: &Name) -> i32 {
        match name.as_str() {
            "Strength" => self.strength_mod,
            "Stamina" => self.stamina_mod,
            _ => 0,
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.id, self.name)
    }
}

pub type EquipmentSlot = Option<Item>;
pub type EquipmentSet = HashMap<Name, EquipmentSlot>;

pub fn equipment_power(equipment: &EquipmentSlot) -> i32 {
    match equipment {
        None => 0,
        Some(i) => i.power,
    }
}

pub fn equipment_mod(equipment: &EquipmentSlot, name: &Name) -> i32 {
    match equipment {
        None => 0,
        Some(i) => i.get_modifier(name),
    }
}

pub fn generate_equipment_set() -> EquipmentSet {
    let mut equips = EquipmentSet::new();
    equips.insert(String::from("Weapon"), None::<Item>);
    equips.insert(String::from("Shield"), None::<Item>);
    equips
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_test() {
        let empty: EquipmentSlot = None;
        let item: EquipmentSlot = Some(Item {
            id: 255,
            name: String::from("Debug Stick"),
            power: 1,
            strength_mod: -1,
            stamina_mod: -2,
            effect: IndexedOrLiteral::<Effect>::Index(2), // Attack
        });
        assert_eq!(equipment_power(&item), 1);
        assert_eq!(equipment_power(&empty), 0);
        assert_eq!(equipment_mod(&item, &String::from("Stamina")), -2);
        assert_eq!(equipment_mod(&empty, &String::from("Stamina")), 0);
        assert_eq!(equipment_mod(&item, &String::from("Strength")), -1);
        assert_eq!(equipment_mod(&empty, &String::from("Strength")), 0);
        assert_eq!(equipment_mod(&item, &String::from("Moxie")), 0);
        assert_eq!(equipment_mod(&empty, &String::from("Moxie")), 0);
    }
}
