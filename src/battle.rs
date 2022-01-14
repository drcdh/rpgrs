use termion::event::Key;

use crate::character::Character;
use crate::common::*;
use crate::encyclopedia::ActionEncyclopedia;
use crate::encyclopedia::CharacterEncyclopedia;
use crate::party::Party;
/*
enum BattleState {
    MENU,
    MESSAGE,
    TURN,
    QUIT,
}*/

pub struct Battle {
    pub allies: Party,
    pub baddies: Party,
    selections: Vec::<usize>, // selections made on parent menus
    text: Vec::<String>,
    current_pc: Option<usize>,
    action_enc: ActionEncyclopedia,
    ch_enc: CharacterEncyclopedia,
}

impl Battle {
    pub fn new(allies: Party, baddies: Party) -> Battle {
        let mut text = Vec::<String>::new();
        text.push("Go kick some ass!".to_string());
        text.push("Battle start!".to_string());
        Battle {
            allies,
            baddies,
            selections: Vec::<usize>::new(),
            text,
            current_pc: None,
            action_enc: ActionEncyclopedia::new("data/actions.json"), // fixme
            ch_enc: CharacterEncyclopedia::new("data/characters.json"), // fixme
        }
    }
    fn get_current_pc_actions(&self) -> Vec::<Vec::<Name>> {
        match self.current_pc {
            Some(i) => self.ch_enc.resolve(self.allies.get_character(i)).unwrap().get_action_options(&self.selections, &self.action_enc),
            None => Vec::<Vec::<Name>>::new(),
        }
    }
    pub fn handle_input(&mut self, key: Key) {
        if key == Key::Char('q') {
            // Handled in BattleCLI
        }
        let l = self.text.len();
        if l > 0 {
            self.pop_text();
        }
        if l > 1 {
            return;
        }
        if self.current_pc == None {
            self.current_pc = Some(0);
            let (_, next_pc) = self.ch_enc.resolve(self.allies.get_character(self.current_pc.unwrap())).unwrap().whoami();
            self.text.push(format!("It's {}'s turn!", next_pc));
            return;
        }
        self.make_selection(key);
    }
    pub fn get_text(&self) -> Option<&String> {
        let l = self.text.len();
        if l > 0 {
            return self.text.get(l-1);
        }
        None
    }
    fn pop_text(&mut self) {
        self.text.pop();
    }
    pub fn get_top_menu_options(&self) -> Option<Vec::<String>> {
        if self.text.len() > 0 {
            return None; // todo
        }
        match self.current_pc {
            Some(_) => self.get_current_pc_actions().get(self.selections.len()).cloned(),
            None => None,
        }
    }
    fn make_selection(&mut self, key: Key) {
        if let Key::Char(c) = key {
            if let Some(i) = c.to_digit(10) {
                self.selections.push((i as usize) - 1);
//                return;
            }
        }
        if let Some(a) = self.ch_enc.resolve(self.allies.get_character(self.current_pc.unwrap())).unwrap().get_action_selection(&self.selections, &self.action_enc) {
            // Clear the menu stack
            self.selections.clear();
            self.current_pc = None;
            self.text.push(a.copy_name());
            return;
        }
    }/*
    fn progress(&mut self) {
        self.current_pc = Some(0);
    }*/
}
