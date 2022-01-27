use termion::event::Key;

use crate::action::Action;
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
    pub targets: Vec::<usize>, // todo, just enemies for now
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
            targets: Vec::<usize>::new(),
            action_enc: ActionEncyclopedia::new("data/actions.json"), // fixme
            ch_enc: CharacterEncyclopedia::new("data/characters.json"), // fixme
        }
    }
    fn get_current_pc_actions(&self) -> Vec::<Vec::<Name>> {
        let ns = self.selections.len()-1;
        let parent_menu_selections = &self.selections[..ns];
        eprintln!("{:?}", self.selections);
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
    fn start_action(&mut self) {
        let a_str = self.get_selected_action().unwrap().copy_name();
        eprintln!("Starting Action \'{}\'", a_str);
        self.text.push(a_str); // todo, placeholder for applying the Action
        // Clear the menu stack
        self.selections.clear();
        self.current_pc = None;
        self.targets.clear();
        return;
    }
    fn get_selected_action(&self) -> Option<&Action> {
        let c = self.ch_enc.resolve(self.allies.get_character(self.current_pc.unwrap())).unwrap();
        c.get_action_selection(&self.selections[..], &self.action_enc)
    }
    fn next_menu(&mut self) {
        if let Some(a) = self.get_selected_action() {
            eprintln!("Targeting mode");
            // Enable targeting mode
            self.targets.push(0);
        } else {
            eprintln!("Next menu");
            self.selections.push(0);
        }
    }
    fn make_selection(&mut self, key: Key) {
        if self.targets.len() > 0 {
            if let Key::Char(c) = key {
                if c == '\n' {
                    self.start_action();
                }
            } else if key == Key::Left || key == Key::Right {
                if self.targets.len() < 2 {
                    let mut i = self.targets.pop().unwrap();
                    if key == Key::Left {
                        if i == 0 {
                            i = self.baddies.len()-1;
                        } else {
                            i -= 1;
                        }
                    } else if key == Key::Right {
                        i += 1;
                        if i >= self.baddies.len() { i = 0; }
                    }
                    self.targets.push(i);
                }
            } else if key == Key::Up {
                for i in 0..self.baddies.len() {
                    if !self.targets.contains(&i) {
                        self.targets.push(i);
                    }
                }
            } else if key == Key::Down {
                // Remove all but first element
                for _ in 1..self.targets.len() {
                    self.targets.pop();
                }
            }
            return;
        }
        if let Some(options) = self.get_top_menu_options() {
            if let Key::Char(c) = key {
                if c == '\n' {
                    self.next_menu();
                } else if let Some(i) = c.to_digit(10) {
                    if i > 0 {
                        let i = (i as usize) - 1;
                        if i < options.len() {
                            self.selections.pop();
                            self.selections.push(i);
                            self.next_menu();
                        }
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
