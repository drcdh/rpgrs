use std::fmt;

use crate::common::Id;
use crate::common::Name;

pub enum Scope {
    None,
    Me,
    Ally,
    Enemy,
    One,
    Allies,
    Enemies,
    All,
}

pub struct Effect {
    id: Id,
    name: Name,
    scope: Scope,
}

impl Effect {
    pub fn new(id: Id, name: Name, scope: Scope) -> Effect {
        Effect { id, name, scope }
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
        let effect = Effect::new(0, String::from("Potion"), Scope::One);
        println!(">>> Test effect: {}", effect);
    }
}
