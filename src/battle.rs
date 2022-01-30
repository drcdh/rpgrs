use std::collections::VecDeque;

use termion::event::Key;

use crate::action::Action;
use crate::character::Character;
use crate::common::*;
use crate::effect::Effect;
use crate::encyclopedia::ActionEncyclopedia;
use crate::encyclopedia::CharacterEncyclopedia;
use crate::encyclopedia::EffectEncyclopedia;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::formula::eval_hit;
use crate::party::Party;


#[derive(PartialEq, Eq)]
pub enum PlayerIndex {
    Ally(usize),
    Baddy(usize),
}
/*
struct BattleEffect {
    actor: PlayerIndex,
    target: PlayerIndex,
    effect: IndexedOrLiteral<Effect>,
}
*/
pub struct Battle {
    pub allies: Party,
    pub baddies: Party,
    // selections made on parent menus and the highlighted option on
    // the top menu.
    pub selections: Vec::<usize>,
    text: VecDeque::<String>,
    current_pc: Option<PlayerIndex>,
    current_npc: Option<PlayerIndex>,
    pub targets: Vec::<PlayerIndex>,
    //effects: VecDeque::<BattleEffect>,
    action_enc: ActionEncyclopedia,
    ch_enc: CharacterEncyclopedia,
    effect_enc: EffectEncyclopedia,
    statblocks: StatBlockEncyclopedia,
}

impl Battle {
    pub fn new(allies: Party, baddies: Party) -> Battle {
        let mut text = VecDeque::<String>::new();
        text.push_back("Battle start!".to_string());
        text.push_back("Go kick some ass!".to_string());
        Battle {
            allies,
            baddies,
            selections: Vec::<usize>::new(),
            text,
            current_pc: None,
            current_npc: None,
            targets: Vec::<PlayerIndex>::new(),
            //effects: VecDeque::<BattleEffect>::new(),
            // FIXME: references should be supplied by the top-level Game object
            action_enc: ActionEncyclopedia::new("data/actions.json"),
            ch_enc: CharacterEncyclopedia::new("data/characters.json"),
            effect_enc: EffectEncyclopedia::new("data/effects.json"),
            statblocks: StatBlockEncyclopedia::new("data/stats.json"),
        }
    }
    fn next_turn(&mut self) {
        // Increment all characters' clocks while no one's turn is up
        loop {
            if let Some(i) = self.allies.get_ready_character() {
                self.current_pc = Some(PlayerIndex::Ally(i));
                self.selections.push(0);
                let next = self.ch_enc.resolve(self.get_current_pc().unwrap()).unwrap().copy_name();
                self.text.push_back(format!("It's {}'s turn!", next));
                return;
            }
            if let Some(i) = self.baddies.get_ready_character() {
                self.current_npc = Some(PlayerIndex::Baddy(i));
                let next = self.ch_enc.resolve(self.get_current_npc().unwrap()).unwrap().copy_name();
                self.text.push_back(format!("It's {}'s turn!", next));
                // TODO
                self.current_npc = None;
                return;
            }
            // Increment characters' clocks
            self.allies.increment_clocks(1, &self.ch_enc, &self.statblocks);
            self.baddies.increment_clocks(1, &self.ch_enc, &self.statblocks);
        }
    }
    fn handle_hit(&mut self, hit: &Hit, actor: &Character, target: &Character) -> String {
        let amount = match &hit.amount {
            HitAmt::Constant(v) => *v,
            HitAmt::Formula(f) => eval_hit(f, actor, target, &self.statblocks),
        };
        //target.take_hit(&Hit { pool: String::from(hit.pool.as_str()), amount: HitAmt::Constant(amount) });
        format!("{} took {} {} damage! ", target.copy_name(), amount, hit.pool)
    }
/*    fn handle_effect(&mut self) {
        if let Some(be) = self.effects.pop_front() {
            let actor: &Character = self.ch_enc.resolve(self.get_character(&Some(be.actor)).unwrap()).unwrap();
            let target: &Character = self.ch_enc.resolve(self.get_character(&Some(be.target)).unwrap()).unwrap();
            let mut effect_msg = String::from("TEST EFFECT: ");
            let effect = self.effect_enc.resolve(&be.effect).unwrap();
            for hit in &effect.hits {
                effect_msg.push_str(self.handle_hit(hit, actor, target).as_str());
            }
            self.text.push_back(effect_msg);
        }
    }*/
    fn get_current_pc_actions(&self) -> Vec::<Vec::<Name>> {
        let ns = self.selections.len()-1;
        let parent_menu_selections = &self.selections[..ns];
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
        if !self.text.is_empty() {
            self.pop_text();
            //self.handle_effect();
            if self.current_pc.is_none() && self.current_npc.is_none() {
                self.next_turn();
            }
            return;
        }
        self.make_selection(key);
    }
    pub fn get_text(&self) -> Option<&String> {
        self.text.front()
    }
    fn pop_text(&mut self) {
        self.text.pop_front();
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
    fn get_target_names(&self) -> Vec::<Name> {
        let mut target_names = Vec::<Name>::new();
        for i in &self.targets {
            if let PlayerIndex::Ally(i) = i {
                let t_name = self.ch_enc.resolve(self.allies.get_character(*i)).unwrap().copy_name();
                target_names.push(t_name);
            } else if let PlayerIndex::Baddy(i) = i {
                let t_name = self.ch_enc.resolve(self.baddies.get_character(*i)).unwrap().copy_name();
                target_names.push(t_name);
            }
        }
        target_names
    }
    fn play_pc_action(&mut self) {
        let a = self.get_selected_action().unwrap();
        eprintln!("Starting Action \'{}\'", a.copy_name());
        // Queue up the Action Effects
        for target in &self.targets {
            for effect in &a.effects {
                //self.effects.push_back(BattleEffect { actor: self.current_pc.unwrap(), target: *target, effect: *effect });
                for hit in &self.effect_enc.resolve(effect).unwrap().hits {
                    //self.handle_hit(hit, self.current_pc.unwrap(), target);
                }
            }
        }
        // Queue the Action message
        let pc_name = self.ch_enc.resolve(self.get_current_pc().unwrap()).unwrap().copy_name();
        let msg = a.get_message(&pc_name, &self.get_target_names());
        self.text.push_back(msg);
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
    }
}
