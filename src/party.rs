use core::slice::Iter;

use crate::common::*;
use crate::character::Character;
use crate::encyclopedia::CharacterEncyclopedia;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::item::Item;

type Group = Vec<Character>;
type Ordering = Vec<usize>;
type ItemPool = Vec<Item>;
type Clocks = Vec<u16>; // Following FFVI

pub struct Party {
    id: Id,
    name: Name,
    group: Group,
    formation: Ordering,
    items: ItemPool,
    pub clocks: Clocks,
}

impl Party {
    pub fn new(name: Name) -> Party {
        Party { id: 0, name, group: Group::new(), formation: Ordering::new(), items: ItemPool::new(), clocks: Clocks::new() }
    }
    pub fn whoami(&self) -> (Id, &str) {
        (self.id, &self.name[..])
    }
    pub fn add_character(&mut self, ch: Character) {
        self.group.push(ch);
        self.formation.push(self.group.len()-1);
        self.clocks.push(0);
    }
    pub fn add_clone(&mut self, iol_ch: &IndexedOrLiteral<Character>, ch_enc: &CharacterEncyclopedia) {
        self.group.push(ch_enc.clone_entry(iol_ch).unwrap());
        self.formation.push(self.group.len()-1);
        self.clocks.push(0);
    }
    pub fn remove_character(&mut self, id: Id) -> Option<Character> {
        if let Some(index) = self.group.iter().position(|ch| ch.matches(id) ) {
            let removed: Character = self.group.remove(index);
            self.formation.retain(|&i| i != index);
            self.clocks.remove(index);
            Some(removed)
        } else { None }
    }
    pub fn is_empty(&self) -> bool {
        self.group.is_empty()
    }
    pub fn len(&self) -> usize {
        self.group.len()
    }
    pub fn get_ch_by_pos(&self, i: usize) -> Option<&Character> {
        match self.formation.get(i) {
            Some(i) => self.group.get(*i),
            None => None,
        }
    }
    pub fn get_mut_ch_by_pos(&mut self, i: usize) -> Option<&mut Character> {
        match self.formation.get(i) {
            Some(i) => self.group.get_mut(*i),
            None => None,
        }
    }
    pub fn get_ready_ch_pos(&mut self) -> Option<usize> {
        for (i, c) in self.clocks.iter_mut().enumerate() {
            if *c == u16::MAX {
                *c = 0;
                return Some(i);
            }
        }
        None
    }
    pub fn increment_clocks(&mut self, dt: u16, statblocks: &StatBlockEncyclopedia) {
        for (ch, clk) in self.group.iter().zip(self.clocks.iter_mut()) {
            let dclk = ch.dclock(dt, statblocks);
            *clk = clk.saturating_add(dclk);
        }
    }
    pub fn items_iter(&self) -> Iter<Item> {
        self.items.iter()
    }
    pub fn all_down(&self) -> bool {
        for ch in self.group.iter() {
            if !ch.is_down() { return false; }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let party = Party::new(String::from("Test"));
        assert_eq!(party.whoami(), (0, "Test"));
        assert_eq!(party.id, 0);
        assert_eq!(party.name, String::from("Test"));
        assert!(party.is_empty());
        assert!(party.group.is_empty());
        assert!(party.formation.is_empty());
        assert!(party.clocks.is_empty());
        assert!(party.items_iter().collect::<Vec<_>>().is_empty());
    }
    #[test]
    fn add_remove_character_test() {
        let mut party = Party::new(String::from("Test"));
        let mog = Character::new(0, String::from("Mog"));
        party.add_character(mog);
        assert!(party.get_ch_by_pos(0).is_some());
        assert_eq!(party.len(), 1);
        assert_eq!(party.group.len(), 1);
        assert_eq!(party.formation.len(), 1);
        assert_eq!(party.clocks.len(), 1);
        assert_eq!(*party.formation.get(0).unwrap(), 0);
        let mog = party.remove_character(0).unwrap();
        assert_eq!(mog.whoami(), (0, "Mog"));
        assert!(party.get_ch_by_pos(0).is_none());
        assert!(party.is_empty());
        assert!(party.group.is_empty());
        assert!(party.formation.is_empty());
        assert!(party.clocks.is_empty());
    }
    #[test]
    fn clocks_test() {
        let mut party = Party::new(String::from("Test"));
        let mog = Character::new(0, String::from("Mog"));
        party.add_character(mog);
        assert_eq!(party.get_ready_ch_pos(), None);
        party.increment_clocks(u16::MAX, &StatBlockEncyclopedia::new("data/stats.json"));
        assert!(matches!(party.get_ready_ch_pos(), Some(_)));
    }
}
