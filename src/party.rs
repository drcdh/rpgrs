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
    pub fn len(&self) -> usize {
        self.group.len()
    }
    pub fn get_ch_by_pos(&self, i: usize) -> &Character {
        &self.group[self.formation[i]]
    }
    pub fn get_mut_ch_by_pos(&mut self, i: usize) -> &mut Character {
        &mut self.group[self.formation[i]]
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_test() {
        let party = Party::new(String::from("Test"));
        assert_eq!(party.id, 0);
        assert_eq!(party.name, String::from("Test"));
        assert_eq!(party.len(), 0);
        assert_eq!(party.group.len(), 0);
        assert_eq!(party.formation.len(), 0);
        assert_eq!(party.clocks.len(), 0);
    }
    #[test]
    fn add_remove_character_test() {
        let mut party = Party::new(String::from("Test"));
        let mog = Character::new(0, String::from("Mog"));
        party.add_character(mog);
        assert_eq!(party.len(), 1);
        assert_eq!(party.group.len(), 1);
        assert_eq!(party.formation.len(), 1);
        assert_eq!(party.clocks.len(), 1);
        assert_eq!(*party.formation.get(0).unwrap(), 0);
        if let Some(mog) = party.remove_character(0) {
            assert_eq!(mog.whoami(), (0, "Mog"));
        }
        assert_eq!(party.len(), 0);
        assert_eq!(party.group.len(), 0);
        assert_eq!(party.formation.len(), 0);
        assert_eq!(party.clocks.len(), 0);
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
