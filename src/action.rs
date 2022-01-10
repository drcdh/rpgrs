use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::common::*;
use crate::effect::Effect;

pub type Costs = HashMap::<String, u32>;

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub enum Scope {
    None,
    Me,
    You,
    Ally,
    Enemy,
    One,
    MyAllies,
    Allies,
    Enemies,
    All,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub struct Action {
    id: Id,
    name: Name,
    #[serde(default)]
    costs: Costs,
    effects: Vec::<IndexedOrLiteral<Effect>>,
    #[serde(default = "Action::default_scope")]
    scope: Scope,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub enum CharacterAction {
    Index(Id),
    Selection(Vec::<CharacterAction>), // e.g. Magic, Techs, Dance, etc.
    Literal(Action),
    UseItem,
}

pub type CharacterActions = HashMap::<Name, CharacterAction>;

impl Action {
    fn default_scope() -> Scope {
        Scope::Enemy
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.id, self.name)
    }
}
