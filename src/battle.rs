use termion::event::Key;

use crate::action::Action;
use crate::character::Character;
use crate::common::*;
use crate::encyclopedia::ActionEncyclopedia;
use crate::encyclopedia::CharacterEncyclopedia;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::party::Party;
/*
enum BattleState {
    MENU,
    MESSAGE,
    TURN,
    QUIT,
}*/
#[derive(PartialEq, Eq)]
pub enum PlayerIndex {
    Ally(usize),
    Baddy(usize),
}

pub struct Battle {
    pub allies: Party,
    pub baddies: Party,
    // selections made on parent menus and the highlighted option on
    // the top menu.
    pub selections: Vec::<usize>,
    text: Vec::<String>,
    current_pc: Option<PlayerIndex>,
    current_npc: Option<PlayerIndex>,
    pub targets: Vec::<PlayerIndex>,
    action_enc: ActionEncyclopedia,
    ch_enc: CharacterEncyclopedia,
    statblocks: StatBlockEncyclopedia,
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
            current_npc: None,
            targets: Vec::<PlayerIndex>::new(),
            // FIXME: references should be supplied by the top-level Game object
            action_enc: ActionEncyclopedia::new("data/actions.json"),
            ch_enc: CharacterEncyclopedia::new("data/characters.json"),
            statblocks: StatBlockEncyclopedia::new("data/stats.json"),
        }
    }
    fn next_turn(&mut self) {
        // Increment all characters' clocks while no one's turn is up
        loop {
            if let Some(i) = self.allies.get_ready_character() {
                self.current_pc = Some(PlayerIndex::Ally(i));
                self.selections.push(0);
                let (_, next) = self.ch_enc.resolve(self.get_current_pc().unwrap()).unwrap().whoami();
                self.text.push(format!("It's {}'s turn!", next));
                return;
            }
            if let Some(i) = self.baddies.get_ready_character() {
                self.current_npc = Some(PlayerIndex::Baddy(i));
                let (_, next) = self.ch_enc.resolve(self.get_current_npc().unwrap()).unwrap().whoami();
                self.text.push(format!("It's {}'s turn!", next));
                // TODO
                self.current_npc = None;
                return;
            }
            // Increment characters' clocks
            self.allies.increment_clocks(1, &self.ch_enc, &self.statblocks);
            self.baddies.increment_clocks(1, &self.ch_enc, &self.statblocks);
        }
    }
    fn get_current_pc_actions(&self) -> Vec::<Vec::<Name>> {
        let ns = self.selections.len()-1;
        let parent_menu_selections = &self.selections[..ns];
//        eprintln!("{:?}", self.selections);
        match self.current_pc {
            Some(PlayerIndex::Ally(i)) => self.ch_enc.resolve(self.allies.get_character(i)).unwrap().get_action_options(parent_menu_selections, &self.action_enc),
            Some(PlayerIndex::Baddy(i)) => self.ch_enc.resolve(self.baddies.get_character(i)).unwrap().get_action_options(parent_menu_selections, &self.action_enc),
            None => Vec::<Vec::<Name>>::new(), // fixme??
        }
    }
    pub fn handle_input(&mut self, key: Key) {
        if key == Key::Char('q') {
            // Handled in BattleCLI
        }
        let l = self.text.len();
        if l > 0 {
            self.pop_text();
            if self.current_pc.is_none() && self.current_npc.is_none() {
                self.next_turn();
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
    fn get_character(&self, p_idx: &Option<PlayerIndex>) -> Option<&IndexedOrLiteral<Character>> {
        match p_idx {
            Some(PlayerIndex::Ally(i)) => Some(self.allies.get_character(*i)),
            Some(PlayerIndex::Baddy(i)) => Some(self.baddies.get_character(*i)),
            None => None,
        }
    }
    fn get_current_pc(&self) -> Option<&IndexedOrLiteral<Character>> {
        self.get_character(&self.current_pc)
    }
    fn get_current_npc(&self) -> Option<&IndexedOrLiteral<Character>> {
        self.get_character(&self.current_npc)
    }
    fn play_pc_action(&mut self) {
        let a = self.get_selected_action().unwrap();
        eprintln!("Starting Action \'{}\'", a.copy_name());
        let (_, cname) = self.ch_enc.resolve(self.get_current_pc().unwrap()).unwrap().whoami();
        let mut target_names = Vec::<&str>::new();
        for i in &self.targets {
            if let PlayerIndex::Ally(i) = i {
                let (_, ename) = self.ch_enc.resolve(self.allies.get_character(*i)).unwrap().whoami();
                target_names.push(ename);
            } else if let PlayerIndex::Baddy(i) = i {
                let (_, ename) = self.ch_enc.resolve(self.baddies.get_character(*i)).unwrap().whoami();
                target_names.push(ename);
            }
        }
        self.text.push(a.get_message(cname, &target_names));
        // Clear the menu stack
        self.selections.clear();
        self.current_pc = None;
        self.targets.clear();
    }
    fn get_selected_action(&self) -> Option<&Action> {
        let c = match self.current_pc.as_ref().unwrap() {
            PlayerIndex::Ally(i) => self.ch_enc.resolve(self.allies.get_character(*i)).unwrap(),
            PlayerIndex::Baddy(i) => self.ch_enc.resolve(self.baddies.get_character(*i)).unwrap(),
        };
        c.get_action_selection(&self.selections[..], &self.action_enc)
    }
    fn next_menu(&mut self) {
        if let Some(a) = self.get_selected_action() {
            eprintln!("Targeting mode");
            // Enable targeting mode
            // todo: check Action scope
            self.targets.push(PlayerIndex::Baddy(0));
        } else {
            eprintln!("Next menu");
            self.selections.push(0);
        }
    }
    fn change_member_selection(key: Key, i: &mut usize, l: usize) {
        if key == Key::Left {
            if *i == 0 {
                *i = l-1;
            } else {
                *i -= 1;
            }
        } else if key == Key::Right {
            *i += 1;
            if *i >= l { *i = 0; }
        }
    }
    fn make_selection(&mut self, key: Key) {
        if self.targets.len() > 0 {
            if let Key::Char(c) = key {
                if c == '\n' {
                    self.play_pc_action();
                }
            } else if key == Key::Left || key == Key::Right {
                if self.targets.len() < 2 {
                    if let PlayerIndex::Ally(mut i) = self.targets.first().unwrap() {
                        eprintln!("Selecting adjacent ally");
                        self.targets.pop();
                        Battle::change_member_selection(key, &mut i, self.allies.len());
                        self.targets.push(PlayerIndex::Ally(i));
                    } else if let PlayerIndex::Baddy(mut i) = self.targets.first().unwrap() {
                        eprintln!("Selecting adjacent baddie");
                        self.targets.pop();
                        Battle::change_member_selection(key, &mut i, self.baddies.len());
                        self.targets.push(PlayerIndex::Baddy(i));
                    }
                }
            } else if key == Key::Up {
                if matches!(self.targets.first(), Some(PlayerIndex::Baddy(_))) {
                    if self.targets.len() > 1 {
                        // Do nothing
                        eprintln!("All baddies already selected");
                    } else {
                        eprintln!("Selecting all baddies");
                        // Select all enemies
                        for i in 0..self.baddies.len() {
                            let b = PlayerIndex::Baddy(i);
                            if !self.targets.contains(&b) {
                                self.targets.push(b);
                            }
                        }
                    }
                } else if let PlayerIndex::Ally(mut i) = self.targets.first().unwrap() {
                    if self.targets.len() > 1 {
                        eprintln!("Deselecting whole party");
                        // Remove all but first element, the previous single-selected ally
                        for _ in 1..self.targets.len() {
                            self.targets.pop();
                        }
                    } else {
                        // Move up to enemy party
                        eprintln!("Moving selection to enemies");
                        self.targets.pop(); // Remove the singular ally target
                        if i >= self.baddies.len() { i = self.baddies.len()-1; }
                        self.targets.push(PlayerIndex::Baddy(i));
                    }
                }
            } else if key == Key::Down {
                if let PlayerIndex::Baddy(mut i) = self.targets.first().unwrap() {
                    if self.targets.len() > 1 {
                        eprintln!("Deselecting whole enemy party");
                        // Remove all but first element, the previous single-selected baddy
                        for _ in 1..self.targets.len() {
                            self.targets.pop();
                        }
                    } else {
                        // Move down to ally party
                        eprintln!("Moving selection to allies");
                        self.targets.pop(); // Remove the singlular enemy target
                        if i >= self.allies.len() { i = self.allies.len()-1; }
                        self.targets.push(PlayerIndex::Ally(i));
                    }
                } else if matches!(self.targets.first(), Some(PlayerIndex::Ally(_))) {
                    if self.targets.len() > 1 {
                        // Do nothing
                    } else {
                        // Select all allies
                        eprintln!("Selecting all allies");
                        for i in 0..self.allies.len() {
                            let a = PlayerIndex::Ally(i);
                            if !self.targets.contains(&a) {
                                self.targets.push(a);
                            }
                        }
                    }
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
