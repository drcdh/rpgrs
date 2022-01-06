use crate::item;
use crate::item::EquipmentSet;
use crate::item::Item;
use crate::stats;


type Name = String;

pub struct Character {
    name: Name,

    base_stats: stats::BaseStats,
    stats: stats::DerivedStats,

    items: EquipmentSet,
}

impl Character {
    // `&self` is short for `self: &Self`
    // Here `Self` is short for `Character`
    pub fn whoami(&self) -> &str {
        &self.name[..]
    }
    pub fn get_stat(&self, name: Name) -> stats::Stat {
        match self.stats.get(&name) {
            Some(ds) => ds(&self.base_stats, &self.items),
            None => 0, // TODO!
        }
    }
    pub fn equip(&mut self, item: Item) {
        self.items.push(Some(item));
    }
}

pub fn create(name: Name) -> Character {
    let (base_stats, stats) = stats::generate_stats();
    Character {
        name,
        base_stats,
        stats,
        items: EquipmentSet::new(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instance_test() {
        let name = "Mog";
        let (base_stats, stats) = stats::generate_stats();
        let mog = Character {
            name: String::from(name),
            base_stats,
            stats,
            items: EquipmentSet::new(),
        };
        assert_eq!(mog.whoami(), name);
        assert_eq!(mog.get_stat(String::from("Strength")), 10);
        assert_eq!(mog.get_stat(String::from("Stamina")), 10);
        //assert_eq!(mog.get_stat(String::from("Moxie")), None);
    }
}
