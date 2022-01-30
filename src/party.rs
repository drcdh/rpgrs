use crate::common::*;
use crate::character::Character;
use crate::encyclopedia::CharacterEncyclopedia;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::item::Item;

// todo: use HashSet and/or make OrderedSet struct
type Group = Vec<IndexedOrLiteral<Character>>;
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
    pub fn add_character(&mut self, ch: IndexedOrLiteral<Character>) {
        self.group.push(ch);
        self.formation.push(self.group.len()-1);
        self.clocks.push(0);
    }
    pub fn remove_character(&mut self, id: Id) -> IndexedOrLiteral<Character> {
        let index = self.group.iter().position(|ch| match ch { IndexedOrLiteral::Index(i) => *i == id, IndexedOrLiteral::Literal(ch) => ch.matches(id) }).unwrap();
        let removed: IndexedOrLiteral<Character> = self.group.remove(index);
        self.formation.retain(|&i| i != index);
        self.clocks.remove(index);
        removed
    }
    pub fn len(&self) -> usize {
        self.group.len()
    }
    pub fn get_character(&self, i: usize) -> &IndexedOrLiteral<Character> {
        &self.group[self.formation[i]]
    }
    pub fn get_ready_character(&mut self) -> Option<usize> {
        for (i, c) in self.clocks.iter_mut().enumerate() {
            if *c == u16::MAX {
                *c = 0;
                return Some(i);
            }
        }
        None
    }
    pub fn increment_clocks(&mut self, dt: u16, ch_enc: &CharacterEncyclopedia, statblocks: &StatBlockEncyclopedia) {
        for (ch, cl) in self.group.iter().zip(self.clocks.iter_mut()) {
            let ch = ch_enc.resolve(ch).unwrap();
            let dc = ch.dclock(dt, statblocks);
            *cl = cl.saturating_add(dc);
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
        party.add_character(IndexedOrLiteral::Literal(mog));
        assert_eq!(party.len(), 1);
        assert_eq!(party.group.len(), 1);
        assert_eq!(party.formation.len(), 1);
        assert_eq!(party.clocks.len(), 1);
        assert_eq!(*party.formation.get(0).unwrap(), 0);
        assert!(matches!(party.get_character(0), IndexedOrLiteral::Literal(_)));
        if let IndexedOrLiteral::Literal(mog) = party.remove_character(0) {
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
        party.add_character(IndexedOrLiteral::Literal(mog));
        assert_eq!(party.get_ready_character(), None);
        party.increment_clocks(u16::MAX, &CharacterEncyclopedia::new("data/characters.json"), &StatBlockEncyclopedia::new("data/stats.json"));
        assert!(matches!(party.get_ready_character(), Some(_)));
    }
}
