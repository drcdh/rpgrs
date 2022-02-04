use std::collections::VecDeque;
use rand::Rng;

use termion::event::Key;

use crate::action::{Action, Costs, Scope};
use crate::character::Character;
use crate::common::*;
use crate::effect::Effect;
use crate::encyclopedia::ActionEncyclopedia;
use crate::encyclopedia::EffectEncyclopedia;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::party::Party;


#[derive(PartialEq, Eq)]
#[derive(Clone)]
pub enum PlayerIndex {
    Ally(usize),
    Baddy(usize),
}

struct TargetedEffect {
    actor_pi: PlayerIndex,
    target_pi: PlayerIndex,
    effect: Effect,
}
impl TargetedEffect {
    fn new(actor_pi: &PlayerIndex, target_pi: &PlayerIndex, effect_iol: &IndexedOrLiteral<Effect>, effect_enc: &EffectEncyclopedia) -> TargetedEffect {
        TargetedEffect {
            actor_pi: actor_pi.clone(),
            target_pi: target_pi.clone(),
            effect: effect_enc.clone_entry(effect_iol).unwrap(),
        }
    }
}
struct TargetedHit {
    target_pi: PlayerIndex,
    pool: Name,
    amount: i32,
}

pub struct Battle {
    pub allies: Party,
    pub baddies: Party,

    ended: bool,
    pub selections: Vec::<usize>,
    text: VecDeque::<String>,
    current_pc_idx: Option<PlayerIndex>,
    current_npc_idx: Option<PlayerIndex>,
    pub targets: Vec::<PlayerIndex>,
    effects: VecDeque::<TargetedEffect>,
    hits: VecDeque::<TargetedHit>,

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
            ended: false,
            selections: Vec::<usize>::new(),
            text,
            current_pc_idx: None,
            current_npc_idx: None,
            targets: Vec::<PlayerIndex>::new(),
            effects: VecDeque::<TargetedEffect>::new(),
            hits: VecDeque::<TargetedHit>::new(),
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
                self.play_npc_action();
                self.current_npc_idx = None;
                return;
            }
            // Increment characters' clocks
            self.allies.increment_clocks(1, &self.statblocks);
            self.baddies.increment_clocks(1, &self.statblocks);
        }
    }
    fn handle_hit(&mut self) {
        if let Some(th) = self.hits.pop_front() {
            let name = self.get_character(&Some(th.target_pi.clone())).unwrap().copy_name();
            let v = self.get_mut_character(&Some(th.target_pi)).unwrap().hit_pool(&th.pool, th.amount);
            if v > 0 {
                self.text.push_back(format!("{} took {} {} damage!", name, v, th.pool));
            } else if v == 0 {
                self.text.push_back(format!("No effect on {}...", name));
            } else {
                self.text.push_back(format!("{} was healed for {} {}!", name, -v, th.pool));
            }
        }
    }
    fn handle_effect(&mut self) {
        if let Some(te) = self.effects.pop_front() {
            let actor = self.get_ch_by_pi(&te.actor_pi);
            let target = self.get_ch_by_pi(&te.target_pi);
            let hits = te.effect.actor_affect_target(actor, target, &self.statblocks);
            for hit in hits {
                let target_pi = te.target_pi.clone();
                let pool = hit.pool;
                if let HitAmt::Constant(amount) = hit.amount {
                    self.hits.push_back(TargetedHit { target_pi, pool, amount });
                } else { panic!(); }
            }
        }
    }
    fn get_current_pc_actions(&self) -> Vec::<Vec::<Name>> {
        if self.selections.is_empty() {
            return Vec::<Vec::<Name>>::new();
        }
        let ns = self.selections.len()-1;
        let parent_menu_selections = &self.selections[..ns];
        match self.current_pc_idx {
            Some(PlayerIndex::Ally(i)) => self.allies.get_ch_by_pos(i).unwrap().get_action_options(parent_menu_selections, &self.action_enc),
            Some(PlayerIndex::Baddy(i)) => self.baddies.get_ch_by_pos(i).unwrap().get_action_options(parent_menu_selections, &self.action_enc),
            None => Vec::<Vec::<Name>>::new(), // fixme??
        }
    }
    pub fn handle_input(&mut self, key: Key) -> bool {
        if key == Key::Char('q') {
            // Handled in BattleCLI
        }
        if !self.text.is_empty() {
            self.pop_text();
            self.handle_effect();
            self.handle_hit();
            if !self.check_end_game() && self.text.is_empty() && self.current_pc_idx.is_none() && self.current_npc_idx.is_none() {
                self.next_turn();
            }
            return false;
        }
        self.make_selection(key);
        self.ended
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
    pub fn get_menu_selections(&self) -> (Vec<Vec<String>>, Vec<usize>) {
        (self.get_current_pc_actions(), self.selections.clone())
    }
    fn get_ch_by_pi(&self, p_idx: &PlayerIndex) -> &Character {
        match p_idx {
            PlayerIndex::Ally(i) => self.allies.get_ch_by_pos(*i).unwrap(),
            PlayerIndex::Baddy(i) => self.baddies.get_ch_by_pos(*i).unwrap(),
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
    fn _get_target_names(&self, targets: &[PlayerIndex]) -> Vec::<Name> {
        let mut target_names = Vec::<Name>::new();
        for i in targets {
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
    fn get_target_names(&self) -> Vec::<Name> {
        self._get_target_names(&self.targets)
    }
    fn play_pc_action(&mut self) {
        if let Some(actor) = self.current_pc_idx.as_ref() {
            let mut costs = Costs::new();
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
                costs = a.costs.clone();
                // Queue the Action message
                let msg = a.get_message(&self.get_current_pc().unwrap().copy_name(), &self.get_target_names());
                self.text.push_back(msg);
            }
            let pi = self.current_pc_idx.clone();
            self.get_mut_character(&pi).unwrap().spend_costs(costs);
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
            let actor = self.get_current_pc().unwrap();
            if actor.can_afford_action_costs(a) {
                if a.scope == Scope::Enemy {
                    self.targets = vec![PlayerIndex::Baddy(self.baddies.get_nth_up_pos(0))];
                } else if a.scope == Scope::Ally {
                    self.targets = vec![PlayerIndex::Ally(self.baddies.get_nth_up_pos(0))];
                /*} else if a.scope == Scope::You {
                    if let PlayerIndex::Ally(mut i) = self.current_pc_idx {
                        Battle::change_member_selection(Key::Right, &mut i, self.allies.len());
                        self.targets = vec![PlayerIndex::Ally(i)];
                    }*/
                } else if a.scope == Scope::Enemies {
                    self.targets = (0..self.baddies.len()).map(PlayerIndex::Baddy).collect::<Vec<_>>();
                } else if a.scope == Scope::Allies {
                    self.targets = (0..self.allies.len()).map(PlayerIndex::Ally).collect::<Vec<_>>();
                } else if a.scope == Scope::All {
                    self.targets.append(&mut (0..self.baddies.len()).map(PlayerIndex::Baddy).collect::<Vec<_>>());
                    self.targets.append(&mut (0..self.allies.len()).map(PlayerIndex::Ally).collect::<Vec<_>>());
                } else {
                    panic!("PC Action scopes other than Enemy, Ally, Enemies, Allies, and All not implemented yet.");
                }
           } else {
               self.text.push_back(String::from("Can't afford that action :-/"));
           }
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
                        /* todo check for something like Scope::OneOrAllEnemies
                        eprintln!("Selecting all baddies");
                        // Select all enemies
                        for i in 0..self.baddies.len() {
                            let b = PlayerIndex::Baddy(i);
                            if !self.targets.contains(&b) {
                                self.targets.push(b);
                            }
                        }*/
                    }
                } else if let PlayerIndex::Ally(mut i) = self.targets.first().unwrap() {
                    /* todo check for something like Scope::OnePlayer etc.
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
                    }*/
                }
            } else if key == Key::Down {
                if let PlayerIndex::Baddy(mut i) = self.targets.first().unwrap() {
                    /* ditto
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
                    }*/
                } else if matches!(self.targets.first(), Some(PlayerIndex::Ally(_))) {
                    if self.targets.len() > 1 {
                        // Do nothing
                    } else {
                        /*
                        // Select all allies
                        eprintln!("Selecting all allies");
                        for i in 0..self.allies.len() {
                            let a = PlayerIndex::Ally(i);
                            if !self.targets.contains(&a) {
                                self.targets.push(a);
                            }
                        }*/
                    }
                }
            } else if key == Key::Esc {
                self.targets.clear();
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
            } else if key == Key::Esc {
                if self.selections.len() > 1 {
                    self.selections.pop();
                } // todo switch to next playable character
            }
        }
    }
    fn play_npc_action(&mut self) {
        if let Some(actor_pi) = self.current_npc_idx.as_ref() {
            let mut teffects = VecDeque::<TargetedEffect>::new();
            let actor = self.get_current_npc().unwrap();
            if let Some(a) = actor.get_random_action(&self.action_enc) {
                let targets = self.get_random_targets(actor_pi, &a.scope);
                let target_names = self._get_target_names(&targets);
                for target_pi in targets {
                    for effect in &a.effects {
                        let te = TargetedEffect::new(
                                actor_pi,
                                &target_pi,
                                effect,
                                &self.effect_enc,
                            );
                        teffects.push_back(te);
                    }
                }
                // Queue the Action message
                let msg = a.get_message(&actor.copy_name(), &target_names);
                self.text.push_back(msg);
            }
            self.effects.append(&mut teffects);
            self.current_npc_idx = None;
        }
    }
    fn get_random_targets(&self, actor_pi: &PlayerIndex, scope: &Scope) -> Vec::<PlayerIndex> {
        // todo Assuming actor_pi is PlayerIndex::Baddy
        let na = self.allies.get_num_up();
        let nb = self.baddies.get_num_up();
        let mut rng = rand::thread_rng();
        match scope {
            Scope::Enemy => vec![PlayerIndex::Ally(self.allies.get_nth_up_pos(rng.gen_range(0..na)))],
            Scope::Ally => vec![PlayerIndex::Baddy(self.baddies.get_nth_up_pos(rng.gen_range(0..nb)))],
            Scope::Enemies => (0..na).map(|i| PlayerIndex::Ally(self.allies.get_nth_up_pos(i))).collect::<Vec<_>>(),
            Scope::Allies => (0..nb).map(|i| PlayerIndex::Baddy(self.baddies.get_nth_up_pos(i))).collect::<Vec<_>>(),
            _ => panic!("NPC Action scopes other than Enemy and Ally not implemented yet."),
        }
    }
    pub fn is_player_down(&self, pi: &PlayerIndex) -> bool {
        match pi {
            PlayerIndex::Ally(i) => self.allies.get_ch_by_pos(*i).unwrap().is_down(),
            PlayerIndex::Baddy(i) => self.baddies.get_ch_by_pos(*i).unwrap().is_down(),
        }
    }
    fn reset(&mut self) {
        //self.text.clear();
        self.current_npc_idx = None;
        self.current_pc_idx = None;
        self.selections.clear();
        self.effects.clear();
        self.hits.clear();
    }
    pub fn check_end_game(&mut self) -> bool {
        if self.ended { return true; }
        if self.effects.is_empty() && self.hits.is_empty() {
            if self.allies.all_down() {
                self.reset();
                self.text.push_back(String::from("LOooOoSER!"));
                self.ended = true;
            } else if self.baddies.all_down() {
                self.reset();
                self.text.push_back(String::from("A winner is you!"));
                self.text.push_back(String::from("You'd probably earn some experience points now."));
                self.text.push_back(String::from("You'd probably find some phat loot now."));
                self.ended = true;
            }
        }
        self.ended
    }
    pub fn is_pc_turn(&self, pi: &PlayerIndex) -> bool {
        match &self.current_pc_idx {
            Some(cpi) => cpi == pi,
            None => false,
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
