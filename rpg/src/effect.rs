use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::common::{Id, Name};

#[derive(Serialize, Deserialize, Debug)]
enum Hit {
    Constant(i64),
    Formula(String),
}

type Hits = HashMap::<String, Hit>;
type Traits = Vec::<Name>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Effect {
    id: Id,
    name: Name,
    hits: Hits,
    traits: Traits,
}

impl Effect {
    pub fn new(id: Id, name: Name) -> Effect {
        Effect { id, name, hits: Hits::new(), traits: Traits::new() }
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.id, self.name)
    }
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
