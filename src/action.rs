use std::collections::HashMap;
use std::fmt;

use serde::{Serialize, Deserialize};

use crate::common::*;
use crate::effect::Effect;
use crate::encyclopedia::ActionEncyclopedia;

pub type Costs = HashMap::<String, u32>;
type Effects = Vec::<IndexedOrLiteral<Effect>>;

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
#[derive(Clone, Copy)]
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
#[derive(Clone)]
pub struct Action {
    id: Id,
    name: Name,
    #[serde(default)]
    costs: Costs,
    pub effects: Effects,
    #[serde(default = "Action::default_scope")]
    scope: Scope,
    // A format string that may use {:actor}, {:target}, or {:targets}.
    #[serde(default = "Action::default_message")]
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub enum CharacterAction {
    Index(Id),
    Menu(ActionMenu),
    Literal(Action),
    UseItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
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
    fn default_message() -> String {
        String::from("{:actor} did something to {:target}.")
    }
    pub fn copy_name(&self) -> Name {
        self.name.clone()
    }
    pub fn get_message(&self, actor: &str, target_names: &Vec::<Name>) -> String {
        let ntargets = target_names.len();
        let mut targets = target_names[..ntargets-1].join(", ");
        if ntargets == 1 {
            targets = target_names.get(0).unwrap().to_string();
        } else if ntargets == 2 {
            targets = format!("{} and {}", targets, target_names.get(ntargets-1).unwrap());
        } else if ntargets > 2 {
            targets = format!("{}, and {}", targets, target_names.get(ntargets-1).unwrap());
        }
        String::from(&str::replace(&str::replace(&self.message, "{:actor}", actor), "{:targets}", &targets))
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
        assert_eq!(am.get_prompt(), "ROOT");
        let prompts = am.get_prompts(&act_en);
        assert_eq!(prompts[0], "Attack");
        assert_eq!(prompts[1], "Magic");
        assert_eq!(prompts[2], "Item");
    }

    #[test]
    fn get_message_test() {
        let act = Action {
            id: 0,
            name: String::from("TestAction"),
            costs: Costs::new(),
            effects: Effects::new(),
            scope: Scope::All,
            message: String::from("TEST {:actor} {:targets} TEST"),
        };
        let actor = "A";
        let mut targets: Vec::<Name> = vec![String::from("1")];
        assert_eq!(act.get_message(actor, &targets), "TEST A 1 TEST");
        targets.push(String::from("2"));
        assert_eq!(act.get_message(actor, &targets), "TEST A 1 and 2 TEST");
        targets.push(String::from("3"));
        assert_eq!(act.get_message(actor, &targets), "TEST A 1, 2, and 3 TEST");
    }
}
