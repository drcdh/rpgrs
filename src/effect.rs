use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::common::{Id, Name};


#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
enum Hit {
    Constant(i64),
    Formula(String),
}

type Hits = HashMap::<String, Hit>;

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
    fn default_msg() -> String {
        String::from("{:effect} was used on {:target}, and something happened maybe!")
    }
    pub fn whoami(&self) -> (Id, &str) {
        (self.id, &self.name[..])
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut traits: String = self.traits.join(", ");
        if traits.len() > 0 {
            traits += ". ";
        }
        let mut conditions: String = self.conditions.keys().map(|s| &**s).collect::<Vec<_>>().join(", ");
        if conditions.len() > 0 {
            conditions = "Causes ".to_owned() + &conditions;
            conditions += ". ";
        }
        write!(f, "{}.{}: {}{}", self.id, self.name, traits, conditions)
    }
}

pub trait Target {
    fn feel_effect(&mut self, effect: &Effect);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let (id, name) = (0, "Thingamajig");
        let effect = Effect::new(id, String::from("Thingamajig"));
        assert_eq!(effect.whoami(), (id, name));
    }
}
