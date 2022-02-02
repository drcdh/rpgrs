use std::collections::VecDeque;

use termion::event::Key;

use crate::action::Action;
use crate::character::Character;
use crate::common::*;
use crate::effect::Effect;
use crate::encyclopedia::ActionEncyclopedia;
use crate::encyclopedia::EffectEncyclopedia;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::formula::eval_hit;
use crate::party::Party;


#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub enum PlayerIndex {
    Ally(usize),
    Baddy(usize),
}

struct TargetedEffect {
    actor: PlayerIndex,
    target: PlayerIndex,
    effect: Effect,
}
impl TargetedEffect {
    fn new(actor: &PlayerIndex, target: &PlayerIndex, effect: &IndexedOrLiteral<Effect>, effect_enc: &EffectEncyclopedia) -> TargetedEffect {
        TargetedEffect {
            actor: actor.clone(),
            target: target.clone(),
            effect: effect_enc.clone_entry(effect).unwrap(),
        }
    }
}

pub struct Battle {
    pub allies: Party,
    pub baddies: Party,

    pub selections: Vec::<usize>,
    text: VecDeque::<String>,
    current_pc_idx: Option<PlayerIndex>,
    current_npc_idx: Option<PlayerIndex>,
    pub targets: Vec::<PlayerIndex>,
    effects: VecDeque::<TargetedEffect>,

    action_enc: ActionEncyclopedia,
    effect_enc: EffectEncyclopedia,
    statblocks: StatBlockEncyclopedia,
}

impl Battle {
    pub fn new(allies: Party, baddies: Party) -> Battle {
        let mut text = VecDeque::<String>::new();
        text.push_back("Battle start!".to_string());
        Battle {
            allies,
            baddies,
            selections: Vec::<usize>::new(),
            text,
            current_pc_idx: None,
            current_npc_idx: None,
            targets: Vec::<PlayerIndex>::new(),
            effects: VecDeque::<TargetedEffect>::new(),
            // FIXME: references should be supplied by the top-level Game object
            action_enc: ActionEncyclopedia::new("data/actions.json"),
            effect_enc: EffectEncyclopedia::new("data/effects.json"),
            statblocks: StatBlockEncyclopedia::new("data/stats.json"),
        }
    }
    fn next_turn(&mut self) {
        // Increment all characters' clocks while no one's turn is up
        loop {
            if let Some(i) = self.allies.get_ready_ch_pos() {
                self.current_pc_idx = Some(PlayerIndex::Ally(i));
                self.selections.push(0);
                let next = self.get_current_pc().unwrap().copy_name();
                self.text.push_back(format!("It's {}'s turn!", next));
                return;
            }
            if let Some(i) = self.baddies.get_ready_ch_pos() {
                self.current_npc_idx = Some(PlayerIndex::Baddy(i));
                let next = self.get_current_npc().unwrap().copy_name();
                self.text.push_back(format!("It's {}'s turn!", next));
                // TODO
                self.current_npc_idx = None;
                return;
            }
            // Increment characters' clocks
            self.allies.increment_clocks(1, &self.statblocks);
            self.baddies.increment_clocks(1, &self.statblocks);
        }
    }
    fn handle_hit(&self, hit: &Hit, actor: &Character, target: &Character) -> String {
        let amount = match &hit.amount {
            HitAmt::Constant(v) => *v,
            HitAmt::Formula(f) => eval_hit(f, Some(actor), target, &self.statblocks),
        };
        //target.take_hit(&Hit { pool: String::from(hit.pool.as_str()), amount: HitAmt::Constant(amount) });
        format!("{} took {} {} damage! ", target.copy_name(), amount, hit.pool)
    }
    fn handle_effect(&mut self) {
        if let Some(be) = self.effects.pop_front() {
            let actor: &Character = self.get_character(&Some(be.actor)).unwrap();
            let target: &Character = self.get_character(&Some(be.target)).unwrap();
            let mut effect_msg = String::from("TEST EFFECT: ");
            for hit in &be.effect.hits {
                effect_msg.push_str(self.handle_hit(hit, actor, target).as_str());
            }
            self.text.push_back(effect_msg);
        }
    }
    fn get_current_pc_actions(&self) -> Vec::<Vec::<Name>> {
        let ns = self.selections.len()-1;
        let parent_menu_selections = &self.selections[..ns];
        match self.current_pc_idx {
            Some(PlayerIndex::Ally(i)) => self.allies.get_ch_by_pos(i).unwrap().get_action_options(parent_menu_selections, &self.action_enc),
            Some(PlayerIndex::Baddy(i)) => self.baddies.get_ch_by_pos(i).unwrap().get_action_options(parent_menu_selections, &self.action_enc),
            None => Vec::<Vec::<Name>>::new(), // fixme??
        }
    }
    pub fn handle_input(&mut self, key: Key) {
        if key == Key::Char('q') {
            // Handled in BattleCLI
        }
        if !self.text.is_empty() {
            self.pop_text();
            self.handle_effect();
            if self.current_pc_idx.is_none() && self.current_npc_idx.is_none() {
                self.next_turn();
            }
            return;
        }
        self.make_selection(key);
    }
    pub fn get_text(&self) -> Option<&String> {
        self.text.front()
    }
    fn pop_text(&mut self) -> Option<String> {
        self.text.pop_front()
    }
    pub fn get_top_menu_options(&self) -> Option<Vec::<String>> {
        if !self.text.is_empty() {
            return None; // todo
        }
        match self.current_pc_idx {
            Some(_) => self.get_current_pc_actions().get(self.selections.len()-1).cloned(),
            None => None,
        }
    }
    fn get_character(&self, p_idx: &Option<PlayerIndex>) -> Option<&Character> {
        match p_idx {
            Some(PlayerIndex::Ally(i)) => self.allies.get_ch_by_pos(*i),
            Some(PlayerIndex::Baddy(i)) => self.baddies.get_ch_by_pos(*i),
            None => None,
        }
    }
    fn get_mut_character(&mut self, p_idx: &Option<PlayerIndex>) -> Option<&mut Character> {
        match p_idx {
            Some(PlayerIndex::Ally(i)) => self.allies.get_mut_ch_by_pos(*i),
            Some(PlayerIndex::Baddy(i)) => self.baddies.get_mut_ch_by_pos(*i),
            None => None,
        }
    }
    fn get_current_pc(&self) -> Option<&Character> {
        self.get_character(&self.current_pc_idx)
    }
    fn get_current_npc(&self) -> Option<&Character> {
        self.get_character(&self.current_npc_idx)
    }
    fn get_target_names(&self) -> Vec::<Name> {
        let mut target_names = Vec::<Name>::new();
        for i in &self.targets {
            if let PlayerIndex::Ally(i) = i {
                let t_name = self.allies.get_ch_by_pos(*i).unwrap().copy_name();
                target_names.push(t_name);
            } else if let PlayerIndex::Baddy(i) = i {
                let t_name = self.baddies.get_ch_by_pos(*i).unwrap().copy_name();
                target_names.push(t_name);
            }
        }
        target_names
    }
    fn play_pc_action(&mut self) {
        if let Some(actor) = self.current_pc_idx.as_ref() {
            let mut teffects = VecDeque::<TargetedEffect>::new();
            if let Some(a) = self.get_selected_action() {
                eprintln!("Starting Action \'{}\'", a.copy_name());
                // Queue up the Action Effects
                for target in &self.targets {
                    for effect in &a.effects {
                        let te = TargetedEffect::new(
                                actor,
                                target,
                                effect,
                                &self.effect_enc,
                            );
                        teffects.push_back(te);
                    }
                }
                // Queue the Action message
                let pc_name = self.get_current_pc().unwrap().copy_name();
                let msg = a.get_message(&pc_name, &self.get_target_names());
                self.text.push_back(msg);
            }
            self.effects.append(&mut teffects);
            // Clear the menu stack
            self.selections.clear();
            self.current_pc_idx = None;
            self.targets.clear();
        }
    }
    fn get_selected_action(&self) -> Option<&Action> {
        match self.get_current_pc() {
            Some(c) => c.get_action_selection(&self.selections[..], &self.action_enc),
            None => None,
        }
    }
    fn next_menu(&mut self) {
        if let Some(a) = self.get_selected_action() {
            eprintln!("Targeting mode");
            // Enable targeting mode
            // todo: check Action scope
            let _ = a;
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
        if !self.targets.is_empty() {
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

#[cfg(test)]
impl Battle {
    fn force_turn(&mut self, pi: PlayerIndex) {
        match pi {
            PlayerIndex::Ally(i) =>
                self.current_pc_idx = Some(PlayerIndex::Ally(i)),
            PlayerIndex::Baddy(i) =>
                self.current_npc_idx = Some(PlayerIndex::Baddy(i)),
        }
    }
}

#[cfg(test)]
pub mod tests;
