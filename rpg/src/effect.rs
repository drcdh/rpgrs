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
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.id, self.name)
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
        let effect = Effect::new(0, String::from("Potion"));
        println!(">>> Test effect: {}", effect);
    }
}
