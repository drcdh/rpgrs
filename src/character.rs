use rand::Rng;
use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json;

use crate::action::{Action, ActionMenu, CharacterAction, Costs};
use crate::common::*;
use crate::condition::{Condition, TargetConditions};
use crate::encyclopedia::ActionEncyclopedia;
use crate::encyclopedia::ConditionEncyclopedia;
use crate::encyclopedia::EffectEncyclopedia;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::formula::eval_stat;
use crate::stats::{BaseStats, DerivedStat, Stat};

type CharacterStats = Id; // todo, allow literals in JSON with enum
type Items = Vec<Id>; // todo, allow literals in JSON with CharacterItem

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pool {
    pub name: Name,
    pub current: i32,
    pub maximum: i32,
}

impl fmt::Display for Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}/{}", self.name, self.current, self.maximum)
    }
}

type Pools = HashMap<Name, Pool>;

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    #[serde(default = "Character::default_pools")]
    pools: Pools,
    #[serde(default)]
    pub conditions: TargetConditions,
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
    fn default_pools() -> Pools {
        let mut pools = Pools::new();
        pools.insert(
            String::from("HP"),
            Pool {
                name: String::from("HP"),
                current: 10,
                maximum: 10,
            },
        );
        pools.insert(
            String::from("MP"),
            Pool {
                name: String::from("MP"),
                current: 5,
                maximum: 5,
            },
        );
        pools
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
            pools: Character::default_pools(),
            conditions: TargetConditions::new(),
        }
    }
    pub fn matches(&self, id: Id) -> bool {
        self.id == id
    }
    pub fn from_json(data: &str) -> Character {
        let c: Character =
            serde_json::from_str(data).expect("Character JSON was not well-formatted");
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
    pub fn get_stat<'s>(
        &self,
        name: Name,
        statblocks: &'s StatBlockEncyclopedia,
    ) -> Option<&'s DerivedStat> {
        match statblocks.get(&self.stats) {
            Some(statblock) => statblock.get_stat(name),
            None => None,
        }
    }
    pub fn get_stat_val(
        &self,
        name: Name,
        default: Stat,
        statblocks: &StatBlockEncyclopedia,
    ) -> Stat {
        let base_stat = Name::from(&name);
        match self.get_stat(name, statblocks) {
            Some(formula) => eval_stat(base_stat, formula, self),
            None => default,
        }
    }
    pub fn get_pool_vals(&self, name: String) -> Option<(i32, i32)> {
        self.pools
            .get(&name)
            .map(|pool| (pool.current, pool.maximum))
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
            "Armor" => Some(10),
            _ => None,
        }
    }
    pub fn get_action_options(
        &self,
        selections: &[usize],
        act_en: &ActionEncyclopedia,
    ) -> Vec<Vec<Name>> {
        // Start with the root CharacterActions (e.g. Attack, Magic, Item)
        // This needs to be a mutable reference for the loop below to work.
        let mut menu: &ActionMenu = &self.actions; // ROOT ActionMenu
        let mut result = vec![menu.get_prompts(act_en)];
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
    pub fn get_action_selection<'a>(
        &'a self,
        selections: &[usize],
        action_enc: &'a ActionEncyclopedia,
    ) -> Option<&'a Action> {
        // Start with the root CharacterActions (e.g. Attack, Magic, Item)
        let mut menu: &ActionMenu = &self.actions; // ROOT ActionMenu
        for s in selections {
            let ca: &CharacterAction = menu.get_option(*s).unwrap();
            if let CharacterAction::Index(id) = ca {
                return action_enc.get(id);
            }
            if let CharacterAction::Literal(a) = ca {
                return Some(a);
            }
            if let CharacterAction::Menu(m) = ca {
                menu = m;
                continue;
            }
        }
        None
    }
    pub fn get_random_action<'a>(
        &'a self,
        action_enc: &'a ActionEncyclopedia,
    ) -> Option<&'a Action> {
        // TODO: Check for unusable actions and reroll in case
        let mut menu: &ActionMenu = &self.actions; // ROOT ActionMenu
        let mut rng = rand::thread_rng();
        loop {
            let s = rng.gen_range(0..menu.len());
            let ca: &CharacterAction = menu.get_option(s).unwrap();
            if let CharacterAction::Index(id) = ca {
                return action_enc.get(id);
            }
            if let CharacterAction::Literal(a) = ca {
                return Some(a);
            }
            if let CharacterAction::Menu(m) = ca {
                menu = m;
                continue;
            }
        }
        None
    }
    pub fn dclock(
        &mut self,
        dt: u16,
        conditions: &ConditionEncyclopedia,
        statblocks: &StatBlockEncyclopedia,
    ) -> u16 {
        // FIXME This is almost certainly error-prone ???
        if self.is_down() {
            0
        } else {
            self.experience_conditions(dt, conditions);
            dt.saturating_mul(
                u16::try_from(self.get_stat_val(String::from("Speed"), 0, statblocks))
                    .ok()
                    .unwrap(),
            )
        }
    }
    fn experience_conditions(&mut self, dt: u16, conditions: &ConditionEncyclopedia) {
        let mut expired_condition_indices = Vec::<usize>::new();
        for (idx, tcon) in self.conditions.iter_mut().enumerate() {
            let con = conditions
                .get(&tcon.condition_id)
                .expect("missing condition in encyclopedia");
            for (reff, reff_cd) in con
                .repeat_effects
                .iter()
                .zip(tcon.repeat_effect_countdowns.iter_mut())
            {
                *reff_cd = reff_cd.saturating_sub(dt);
                if *reff_cd == 0 {
                    // TODO reff.rep: IndexOrLiteral<Effect>
                    *reff_cd = reff.period;
                }
            }
            for (rhit, rhit_cd) in con
                .repeat_hits
                .iter()
                .zip(tcon.repeat_hit_countdowns.iter_mut())
            {
                *rhit_cd = rhit_cd.saturating_sub(dt);
                if *rhit_cd == 0 {
                    // TODO rhit.rep: Hit
                    *rhit_cd = rhit.period;
                }
            }
            tcon.duration += dt;
            if let Some(max_duration) = con.duration {
                if tcon.duration >= max_duration {
                    expired_condition_indices.insert(idx, 0);
                }
            }
        }
        for idx in expired_condition_indices.iter() {
            // todo
        }
    }
    pub fn sum_add_mods(&self, stat_name: Name) -> Stat {
        let _ = stat_name;
        0 // todo
    }
    pub fn sum_mult_mods(&self, stat_name: Name) -> Stat {
        let _ = stat_name;
        1 // todo
    }
    pub fn can_afford_action_costs(&self, action: &Action) -> bool {
        for (pool, cost) in action.costs_iter() {
            let pool = self
                .pools
                .get(pool)
                .expect("Character does not have Pool for Action cost");
            if pool.current < *cost {
                return false;
            }
        }
        true
    }
    pub fn spend_costs(&mut self, costs: Costs) {
        for (pool, cost) in costs.iter() {
            let mut pool = self
                .pools
                .get_mut(pool)
                .expect("Character does not have Pool for Action cost");
            pool.current -= *cost;
        }
    }
    pub fn spend_action_costs(&mut self, action: &Action) {
        for (pool, cost) in action.costs_iter() {
            let mut pool = self
                .pools
                .get_mut(pool)
                .expect("Character does not have Pool for Action cost");
            pool.current -= *cost;
        }
    }
    pub fn use_action_on(
        &mut self,
        action: &Action,
        target: &Character,
        effect_enc: &EffectEncyclopedia,
        statblocks: &StatBlockEncyclopedia,
    ) -> Hits {
        self.spend_action_costs(action);
        let mut hits = Hits::new();
        for effect in &action.effects {
            hits.append(
                &mut effect_enc
                    .resolve(effect)
                    .unwrap()
                    .actor_affect_target(self, target, statblocks),
            );
        }
        hits
    }
    pub fn is_down(&self) -> bool {
        //todo critical_pools attribute
        self.get_pool_vals(String::from("HP")).unwrap().0 <= 0
    }
    pub fn take_condition(&mut self, cond: &Condition) -> bool {
        // TODO
        true
    }
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}.{}", self.id, self.name).ok();
        for (_, pool) in &self.pools {
            writeln!(f, "  {}", pool).ok();
        }
        Ok(())
    }
}

impl Target for Character {
    fn hit_pool(&mut self, pool: &String, amount: i32) -> i32 {
        if let Some(mut affected_pool) = self.pools.get_mut(pool) {
            let curr = affected_pool.current;
            let v = amount;
            if curr - v < 0 {
                affected_pool.current = 0;
            } else {
                affected_pool.current = curr - v;
            }
            if affected_pool.current > affected_pool.maximum {
                affected_pool.current = affected_pool.maximum;
            }
            return v;
        }
        0 // todo None (so a zero isn't displayed)
    }
    /*    fn take_condition(&mut self, cond: &Condition) -> bool {
        // TODO
        true
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let (id, name) = (10, "Mog");
        let mog = Character::new(id, String::from(name));
        assert!(mog.matches(id));
        assert_eq!(mog.whoami(), (id, "Mog"));
    }
    #[test]
    fn from_json_test() {
        let (id, name) = (10, "Mog");
        let mog_json = format!("{{\"id\": {}, \"name\": \"{}\"}}", id, String::from(name));
        let mog = Character::from_json(&mog_json);
        assert!(mog.matches(id));
        assert_eq!(mog.whoami(), (id, name));
    }
    #[test]
    fn serde_defaults_test() {
        let (id, name) = (10, "Mog");
        let mog_json = format!("{{\"id\": {}, \"name\": \"{}\"}}", id, String::from(name));
        let mog = Character::from_json(&mog_json);
        assert_eq!(mog.actions, ActionMenu::new());
        assert_eq!(mog.base_stats, Character::default_base_stats());
        assert_eq!(mog.stats, Character::default_stats())
    }
    #[test]
    fn get_base_stat_test() {
        let mog = Character::from_json(r#"{"id": 10, "name": "Mog"}"#);
        assert!(mog.get_base_stat(String::from("Strength")).is_some());
        assert!(mog.get_base_stat(String::from("Moxie")).is_none());
    }
    #[test]
    fn get_stat_test() {
        use crate::encyclopedia::StatBlockEncyclopedia;
        let statblocks = StatBlockEncyclopedia::new("data/stats.json");
        let mog = Character::from_json(r#"{"id": 10, "name": "Mog"}"#);
        assert!(mog
            .get_stat(String::from("Strength"), &statblocks)
            .is_some());
        assert!(mog.get_stat(String::from("Offense"), &statblocks).is_some());
        assert!(mog.get_stat(String::from("Moxie"), &statblocks).is_none());
    }
    #[test]
    fn get_action_options_test() {
        use crate::encyclopedia::ActionEncyclopedia;
        use crate::encyclopedia::CharacterEncyclopedia;
        let actions = ActionEncyclopedia::new("data/actions.json");
        let characters = CharacterEncyclopedia::new("data/characters.json");
        let mog = characters.get(&10).unwrap();
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
        let mog = characters.get(&10).unwrap();
        let selections: Vec<usize> = vec![1, 1]; // "Dance" -> "Desert Lullaby"
        let selected_action = mog.get_action_selection(&selections, &actions).unwrap();
        assert_eq!(selected_action.copy_name(), "Desert Lullaby");
        let selections: Vec<usize> = vec![1, 0]; // "Dance" -> "Water Harmony"
        let selected_action = mog.get_action_selection(&selections, &actions).unwrap();
        assert_eq!(selected_action.copy_name(), "Water Harmony");
    }
    #[test]
    #[should_panic]
    fn bad_pool_test() {
        let c = Character::new(0, String::from("new character"));
        c.get_pool_vals(String::from("ZZ")).unwrap();
    }
}
