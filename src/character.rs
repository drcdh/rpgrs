use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};
use serde_json;

use crate::action::{Action, ActionMenu, CharacterAction};
use crate::common::*;
use crate::encyclopedia::ActionEncyclopedia;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::formula::eval_stat;
use crate::stats::{BaseStats, Stat, DerivedStat};

type CharacterStats = Id; // todo, allow literals in JSON with enum
type Items = Vec::<Id>; // todo, allow literals in JSON with CharacterItem

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Pool {
    pub name: Name,
    pub current: i32,
    pub maximum: i32,
}

type Pools = HashMap::<Name, Pool>;

#[derive(Serialize, Deserialize, Debug)]
#[derive(Clone)]
pub struct Character {
    id: Id,
    name: Name,
    #[serde(default = "Character::default_base_stats")]
    base_stats: BaseStats,
    #[serde(default = "Character::default_stats")]
    stats: CharacterStats,
    #[serde(default = "ActionMenu::new")]
    actions: ActionMenu,
    #[serde(default)]
    items: Items,
    // equips: item::EquipmentSet,
    #[serde(default)]
    pools: Pools,
    #[serde(default)]
    conditions: Vec::<Name>,
}

impl Character {
    fn default_base_stats() -> BaseStats {
        let mut bs = BaseStats::new();
        bs.insert(String::from("Offense"), 10);
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
            base_stats: Character::default_base_stats(),
            stats: Character::default_stats(),
            actions: ActionMenu::new(),
            items: Items::new(),
            //equips: item::generate_equipment_set(),
            pools: Pools::new(),
            conditions: Vec::<Name>::new(),
        }
    }
    pub fn matches(&self, id: Id) -> bool {
        self.id == id
    }
    pub fn from_json(data: &str) -> Character {
        let c: Character = serde_json::from_str(data).expect("Character JSON was not well-formatted");
        c
    }
    // `&self` is short for `self: &Self`
    // Here `Self` is short for `Character`
    pub fn whoami(&self) -> (Id, &str) {
        (self.id, &self.name[..])
    }
    pub fn copy_name(&self) -> Name {
        self.name.clone()
    }
    pub fn get_base_stat(&self, name: Name) -> Option<&Stat> {
        self.base_stats.get(&name)
    }
    pub fn get_stat<'s>(&self, name: Name, statblocks: &'s StatBlockEncyclopedia) -> Option<&'s DerivedStat> {
        match statblocks.get(&self.stats) {
            Some(statblock) => statblock.get_stat(name),
            None => None,
        }
    }
    pub fn get_stat_val<'s>(&self, name: Name, default: Stat, statblocks: &'s StatBlockEncyclopedia) -> Stat {
        let base_stat = Name::from(&name);
        match self.get_stat(name, statblocks) {
            Some(formula) => eval_stat(base_stat, formula, self),
            None => default,
        }
    }
    pub fn get_pool_vals(&self, name: String) -> (i32, i32) {
        match self.pools.get(&name) {
            Some(pool) => (pool.current, pool.maximum),
            None => (0, 0)
        }
    }
    pub fn get_pools(&self) -> &Pools {
        &self.pools
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
    pub fn get_item_attr(&self, slot: Name, attr: Name) -> Option<Stat> {
        let _ = attr;
        match slot.as_str() {
            "Weapon" => Some(10),
            _ => None,
        }
    }
    pub fn get_action_options(&self, selections: &[usize], act_en: &ActionEncyclopedia) -> Vec::<Vec::<Name>> {
        // Start with the root CharacterActions (e.g. Attack, Magic, Item)
        // This needs to be a mutable reference for the loop below to work.
        let mut menu: &ActionMenu = &self.actions; // ROOT ActionMenu
        let mut result = Vec::<Vec::<Name>>::new();
        result.push(menu.get_prompts(act_en));
        for s in selections {
            let ca: &CharacterAction = menu.get_option(*s).unwrap();
            if let CharacterAction::Menu(m) = ca {
                result.push(m.get_prompts(act_en));
                menu = m; // Here is why menu is a mutable reference
            } else {
                panic!("A non-Menu CharacterAction was somehow selected. Should have called get_action_selection :-(");
            }
        }
        result
    }
    pub fn get_action_selection<'a>(&'a self, selections: &[usize], action_enc: &'a ActionEncyclopedia) -> Option<&'a Action> {
        // Start with the root CharacterActions (e.g. Attack, Magic, Item)
        let mut menu: &ActionMenu = &self.actions; // ROOT ActionMenu
        for s in selections {
            let ca: &CharacterAction = menu.get_option(*s).unwrap();
            if let CharacterAction::Index(id) = ca {
                return action_enc.get(&id);
            }
            if let CharacterAction::Literal(a) = ca {
                return Some(&a);
            }
            if let CharacterAction::Menu(m) = ca {
                menu = m;
                continue;
            }
        }
        None
    }
    pub fn dclock(&self, dt: u16, statblocks: &StatBlockEncyclopedia) -> u16 {
        // FIXME This is almost certainly error-prone
        dt.saturating_mul(u16::try_from(self.get_stat_val(String::from("Speed"), 0, statblocks)).ok().unwrap())
    }
    pub fn sum_add_mods(&self, stat_name: Name) -> Stat {
        let _ = stat_name;
        0 // todo
    }
    pub fn sum_mult_mods(&self, stat_name: Name) -> Stat {
        let _ = stat_name;
        1 // todo
    }
}


impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.id, self.name)
    }
}

impl Target for Character {
    fn take_hit(&mut self, hit: &Hit) -> i32 {
        let mut affected_pool = self.pools.get_mut(&hit.pool).expect(format!("Character \"{}\" does not have pool \"{}\"", self.name, hit.pool).as_str());
        if let HitAmt::Constant(v) = hit.amount {
            let curr = affected_pool.current;
            if curr-v < 0 {
                affected_pool.current = 0;
            } else {
                affected_pool.current = curr - v;
            }
            return v;
        }
        panic!();
    }
    fn take_condition(&mut self, hit: &Hit) -> bool {
        self.conditions.push(hit.pool.clone());
        true // fixme
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
        assert_eq!(mog.actions, ActionMenu::new());
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
        use crate::encyclopedia::StatBlockEncyclopedia;
        let statblocks = StatBlockEncyclopedia::new("data/stats.json");
        let mog = Character::from_json(r#"{"id": 0, "name": "Mog"}"#);
        assert!(mog.get_stat(String::from("Strength"), &statblocks).is_some());
        assert!(mog.get_stat(String::from("Offense"), &statblocks).is_some());
        assert!(mog.get_stat(String::from("Moxie"), &statblocks).is_none());
    }
    #[test]
    fn get_action_options_test() {
        use crate::encyclopedia::ActionEncyclopedia;
        use crate::encyclopedia::CharacterEncyclopedia;
        let actions = ActionEncyclopedia::new("data/actions.json");
        let characters = CharacterEncyclopedia::new("data/characters.json");
        let mog = characters.get(&0).unwrap();
        let mut selections = Vec::<usize>::new();
        let mog_menus = mog.get_action_options(&selections, &actions);
        let mut expected_menus = vec![vec!["Attack", "Dance", "Magic", "Item"]];
        assert_eq!(&mog_menus, &expected_menus);
        selections.push(1); // select "Dance"
        expected_menus.push(vec!["Water Harmony", "Desert Lullaby"]);
        let mog_menus = mog.get_action_options(&selections, &actions);
        assert_eq!(mog_menus, expected_menus);        
    }
    #[test]
    fn get_action_selection_test() {
        use crate::encyclopedia::ActionEncyclopedia;
        use crate::encyclopedia::CharacterEncyclopedia;
        let actions = ActionEncyclopedia::new("data/actions.json");
        let characters = CharacterEncyclopedia::new("data/characters.json");
        let mog = characters.get(&0).unwrap();
        let selections: Vec::<usize> = vec![1, 1]; // "Dance" -> "Desert Lullaby"
        let selected_action = mog.get_action_selection(&selections, &actions).unwrap();
        assert_eq!(selected_action.copy_name(), "Desert Lullaby");
        let selections: Vec::<usize> = vec![1, 0]; // "Dance" -> "Water Harmony"
        let selected_action = mog.get_action_selection(&selections, &actions).unwrap();
        assert_eq!(selected_action.copy_name(), "Water Harmony");
    }
}


pub mod dummies {
    use super::*;

    pub struct DummyTarget {
    }
    pub struct AdvancedDummyTarget {
        name: Name,
        pools: Pools,
        conditions: Vec::<Name>,
    }
    impl DummyTarget {
        pub fn new() -> DummyTarget {
            DummyTarget{}
        }
    }
    impl Target for DummyTarget {
        fn take_hit(&mut self, hit: &Hit) -> i32 { match &hit.amount { HitAmt::Constant(v) => *v, HitAmt::Formula(_s) => 0 } }
        fn take_condition(&mut self, _hit: &Hit) -> bool { true }
    }
    impl AdvancedDummyTarget {
        pub fn new() -> AdvancedDummyTarget {
            let conditions = Vec::<Name>::new();
            let mut pools = Pools::new();
            pools.insert(String::from("HP"), Pool { name: String::from("HP"), current: 100, maximum: 100 });
            AdvancedDummyTarget { name: String::from("Advanced Test Dummy"), conditions, pools }
        }
        pub fn get_pool_vals(&self, name: String) -> (i32, i32) {
            match self.pools.get(&name) {
                Some(pool) => (pool.current, pool.maximum),
                None => (0, 0)
            }
        }
    }
    impl Target for AdvancedDummyTarget {
        fn take_hit(&mut self, hit: &Hit) -> i32 {
            let mut affected_pool = self.pools.get_mut(&hit.pool).expect(format!("Target \"{}\" does not have pool \"{}\"", self.name, hit.pool).as_str());
            if let HitAmt::Constant(v) = hit.amount {
                let curr = affected_pool.current;
                if curr-v < 0 {
                    affected_pool.current = 0;
                } else {
                    affected_pool.current = curr - v;
                }
                return v;
            }
            panic!();
        }
        fn take_condition(&mut self, hit: &Hit) -> bool {
            self.conditions.push(hit.pool.clone());
            true // fixme
        }
    }
} // mod dummies
