use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::common::*;
use crate::effect::Effect;
use crate::encyclopedia::ActionEncyclopedia;

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
    Menu(ActionMenu),
    Literal(Action),
    UseItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
pub struct ActionMenu {
    prompt: Name,
    options: Vec::<CharacterAction>,
}
impl ActionMenu {
    // Used for serde(default) in Character
    pub fn new() -> ActionMenu {
        let mut ca = Vec::<CharacterAction>::new();
        ca.push(CharacterAction::Index(0));
        ca.push(CharacterAction::UseItem);
        ActionMenu { prompt: "ROOT".to_string(), options: ca }
    }
    pub fn get_prompt(&self) -> &Name {
        &self.prompt
    }
    pub fn get_prompts(&self, act_en: &ActionEncyclopedia) -> Vec::<Name> {
        let mut pr = Vec::<Name>::new();
        for ca in &self.options {
            pr.push(
                match ca {
                    CharacterAction::Index(id) => act_en.get(&id).unwrap().copy_name(),
                    CharacterAction::Menu(m) => m.prompt.clone(),
                    CharacterAction::Literal(a) => a.name.clone(),
                    CharacterAction::UseItem => "Item".to_string(),
                }
            );
        }
        pr
    }
    pub fn get_option(&self, opt: usize) -> Option::<&CharacterAction> {
        self.options.get(opt)
    }
}

impl Action {
    fn default_scope() -> Scope {
        Scope::Enemy
    }
    pub fn copy_name(&self) -> Name {
        self.name.clone()
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}.{}", self.id, self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_prompts_test() {
        let act_en = ActionEncyclopedia::new("data/actions.json");
        let spell_id: Id = 0;
        let magic = vec![CharacterAction::Index(spell_id)];
        let magic_menu = ActionMenu { prompt: "Magic".to_string(), options: magic };
        assert_eq!(magic_menu.get_prompts(&act_en)[0], act_en.get(&spell_id).unwrap().name);
        let attack_id: Id = 0;
        let options = vec![
                CharacterAction::Index(attack_id),
                CharacterAction::Menu(magic_menu),
                CharacterAction::UseItem,
        ];
        let am = ActionMenu {
            prompt: "ROOT".to_string(),
            options,
        };
        let prompts = am.get_prompts(&act_en);
        assert_eq!(prompts[0], "Attack");
        assert_eq!(prompts[1], "Magic");
        assert_eq!(prompts[2], "Item");
    }
}
