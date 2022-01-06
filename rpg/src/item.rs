use std::collections::HashMap;

type Name = String;

pub struct Item {
    pub name: Name,
    pub power: i32,
    pub stamina_mod: i32,
    pub strength_mod: i32,
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

pub type Equipment = Option<Item>;
// todo pub type EquipmentSet = HashMap<Name, Equipment>;
pub type EquipmentSet = Vec<Equipment>;

pub fn equipment_power(equipment: &Equipment) -> i32 {
    match equipment {
        None => 0,
        Some(i) => i.power,
    }
}

pub fn equipment_mod(equipment: &Equipment, name: &Name) -> i32 {
    match equipment {
        None => 0,
        Some(i) => i.get_modifier(&name),
    }
}

pub fn create(name: String, power: i32, strength_mod: i32, stamina_mod: i32) -> Item {
    Item {name, power, strength_mod, stamina_mod}
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_test() {
        let empty: Equipment = None;
        let item: Equipment = Some(Item {
            name: String::from("Debug Stick"),
            power: 0,
            strength_mod: -1,
            stamina_mod: -2,
        });
        let STAMINA = String::from("Stamina");
        let STRENGTH = String::from("Strength");
        assert_eq!(equipment_power(&item), 0);
        assert_eq!(equipment_power(&empty), 0);
        assert_eq!(equipment_mod(&item, &STAMINA), -2);
        assert_eq!(equipment_mod(&empty, &STAMINA), 0);
        assert_eq!(equipment_mod(&item, &STRENGTH), -1);
        assert_eq!(equipment_mod(&empty, &STRENGTH), 0);
    }
}
