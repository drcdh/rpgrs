use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::common::*;


type Traits = Vec::<Name>;

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub struct Effect {
    id: Id,
    #[serde(default)]
    name: Name,
    #[serde(default)]
    hits: Hits,
    #[serde(default)]
    conditions: Hits,
    #[serde(default)]
    traits: Traits,
    #[serde(default = "Effect::default_msg")]
    msg: String,
}

impl Effect {
    pub fn new(id: Id, name: Name) -> Effect {
        Effect {
            id,
            name,
            hits: Hits::new(),
            conditions: Hits::new(),
            traits: Traits::new(),
            msg: String::from(""),
        }
    }
    pub fn default_msg() -> String {
        String::from("{:effect} was used on {:target}, and something happened maybe!")
    }
    pub fn whoami(&self) -> (Id, &str) {
        (self.id, &self.name[..])
    }
    pub fn apply<T: Target>(&self, target: &mut T) {
        for hit in &self.hits {
            let _v = target.take_hit(hit);
        }
        for cond in &self.conditions {
            let _success = target.take_condition(cond);
        }
        for tr in &self.traits {
        }
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut traits: String = self.traits.join(", ");
        if traits.len() > 0 {
            traits += ". ";
        }
        let mut conditions: String = "".to_string();
        for con in &self.conditions {
            conditions += &con.pool;
        }
        if conditions.len() > 0 {
            conditions = "Causes ".to_owned() + &conditions;
            conditions += ". ";
        }
        write!(f, "{}.{}: {}{}", self.id, self.name, traits, conditions)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::dummies::{DummyTarget, AdvancedDummyTarget};

    #[test]
    fn new_test() {
        let (id, name) = (0, "Thingamajig");
        let effect = Effect::new(id, String::from(name));
        assert_eq!(effect.whoami(), (id, name));
    }
    #[test]
    fn apply_test() {
        let mut t = AdvancedDummyTarget::new();
        let mut effect = Effect::new(0, "Test Effect".to_string());
        let (init_hp, _) = t.get_pool_vals("HP".to_string());
        let v = 10;
        let h = Hit { pool: String::from("HP".to_string()), amount: HitAmt::Constant(v) };
        effect.hits = vec![h];
        effect.apply(&mut t);
        let (hp, _) = t.get_pool_vals("HP".to_string());
        assert_eq!(hp, init_hp-v);
    }
}
