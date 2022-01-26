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
    // selections made on parent menus and the highlighted option on
    // the top menu.
    pub selections: Vec::<usize>,
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
        let ns = self.selections.len()-1;
        let parent_menu_selections = &self.selections[..ns];
        match self.current_pc {
            Some(i) => self.ch_enc.resolve(self.allies.get_character(i)).unwrap().get_action_options(parent_menu_selections, &self.action_enc),
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
            if self.current_pc == None {
                self.current_pc = Some(0);
                self.selections.push(0);
                let (_, next_pc) = self.ch_enc.resolve(self.allies.get_character(self.current_pc.unwrap())).unwrap().whoami();
                self.text.push(format!("It's {}'s turn!", next_pc));
            }
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
            Some(_) => self.get_current_pc_actions().get(self.selections.len()-1).cloned(),
            None => None,
        }
    }
    fn menu_enter(&mut self) {
        let ns = self.selections.len()-1;
        let parent_menu_selections = &self.selections[..ns];
        if let Some(a) = self.ch_enc.resolve(self.allies.get_character(self.current_pc.unwrap())).unwrap().get_action_selection(parent_menu_selections, &self.action_enc) {
            // Clear the menu stack
            self.selections.clear();
            self.current_pc = None;
            self.text.push(a.copy_name()); // todo, placeholder for applying the Action
            return;
        }
    }
    fn make_selection(&mut self, key: Key) {
        if let Some(options) = self.get_top_menu_options() {
            if let Key::Char(c) = key {
                if c == '\n' {
                    self.selections.push(0);
                    self.menu_enter();
                } else if let Some(i) = c.to_digit(10) {
                    let i = (i as usize) - 1;
                    if i >= 0 && i < options.len() {
                        self.selections.pop();
                        self.selections.push(i);
                        self.selections.push(0);
                        self.menu_enter();
                    }
                }
            } else if key == Key::Up || key == Key::Down {
                let mut i = self.selections.pop().unwrap();
                if key == Key::Up {
                    if i == 0 {
                        i = options.len()-1;
                    } else {
                        i -= 1;
                    }
                } else if key == Key::Down {
                    i += 1;
                    if i >= options.len() { i = 0; }
                }
                self.selections.push(i);
            }
        }
    }/*
    fn progress(&mut self) {
        self.current_pc = Some(0);
    }*/
}
