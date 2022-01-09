use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::common::{Id, Name};
use crate::effect::Effect;

pub type Costs = HashMap::<String, u16>;

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
pub enum ActionEffect {
    Index(Id),
    Literal(Effect),
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub struct Action {
    id: Id,
    name: Name,
    #[serde(default)]
    costs: Costs,
    effects: Vec::<ActionEffect>,
    #[serde(default = "Action::default_scope")]
    scope: Scope,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub enum CharacterAction {
    Index(Id),
    Selection(Vec::<CharacterAction>),
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
