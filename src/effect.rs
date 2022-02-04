use std::fmt;

use serde::{Serialize, Deserialize};

use crate::character::Character;
use crate::common::*;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::formula;

type Traits = Vec::<Name>;

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Effect {
    id: Id,
    #[serde(default)]
    name: Name,
    #[serde(default)]
    pub hits: Hits,
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
    pub fn actor_affect_target(&self, actor: &Character, target: &Character, statblocks: &StatBlockEncyclopedia) -> Hits {
        let mut hits = Hits::new();
        for hit in &self.hits {
            let amount: i32 = match &hit.amount {
                HitAmt::Constant(v) => *v,
                HitAmt::Formula(f) => formula::eval_hit(f, Some(actor), target, statblocks),
            };
//            let v: i32 = target.hit_pool(&hit.pool, amount);
            hits.push(Hit { pool: hit.pool.clone(), amount: HitAmt::Constant(amount) });
        }
        hits
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut traits: String = self.traits.join(", ");
        if !traits.is_empty() {
            traits += ". ";
        }
        let mut conditions: String = "".to_string();
        for con in &self.conditions {
            conditions += &con.pool;
        }
        if !conditions.is_empty() {
            conditions = "Causes ".to_owned() + &conditions;
            conditions += ". ";
        }
        write!(f, "{}.{}: {}{}", self.id, self.name, traits, conditions)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::character::Character;

    #[test]
    fn new_test() {
        let (id, name) = (0, "Thingamajig");
        let effect = Effect::new(id, String::from(name));
        assert_eq!(effect.whoami(), (id, name));
    }
    #[test]
    fn actor_affect_test() {
        let statblocks = StatBlockEncyclopedia::new("data/stats.json");
        let c = Character::new(0, String::from("Test Character"));
        let t = Character::new(1, String::from("Test Target Character"));
        let mut effect = Effect::new(0, "Test Effect".to_string());
//        let init_hp = t.get_pool_vals("HP".to_string()).unwrap().0;
        let v = 1;
        let f = format!("{}", v);
        effect.hits = vec![
            Hit { pool: String::from("HP"), amount: HitAmt::Constant(v) },
            Hit { pool: String::from("HP"), amount: HitAmt::Formula(f) },
        ];
        let hits = effect.actor_affect_target(&c, &t, &statblocks);
        assert_eq!(effect.hits[0].pool, hits[0].pool);
        assert_eq!(effect.hits[1].pool, hits[1].pool);
        assert_eq!(hits[0].amount, HitAmt::Constant(v));
        assert_eq!(hits[1].amount, HitAmt::Constant(v));
//        let hp = t.get_pool_vals("HP".to_string()).unwrap().0;
//        assert_eq!(hp, init_hp - 2);
    }
}
