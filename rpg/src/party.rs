use crate::common::Id;
use crate::common::Name;
use crate::character;

type Group = Vec<character::Character>;
type Formation = Vec<usize>;

pub struct Party {
    id: Id,
    name: Name,
    group: Group,
    formation: Formation,
}

impl Party {
    pub fn new(name: Name) -> Party {
        Party { id: 0, name, group: Group::new(), formation: Formation::new(), }
    }
    pub fn add_character(&mut self, ch: character::Character) {
        self.group.push(ch);
        self.formation.push(self.group.len()-1);
    }
    pub fn remove_character(&mut self, id: Id) -> character::Character {
        let index = self.group.iter().position(|ch| ch.id == id).unwrap();
        let removed: character::Character = self.group.remove(index);
        self.formation.retain(|&i| i != index);
        removed
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
        assert_eq!(party.group.len(), 0);
        assert_eq!(party.formation.len(), 0);
    }
    #[test]
    fn add_remove_character_test() {
        let mut party = Party::new(String::from("Test"));
        let mog = character::Character::new(String::from("Mog"));
        party.add_character(mog);
        assert_eq!(party.group.len(), 1);
        assert_eq!(party.formation.len(), 1);
        assert_eq!(*party.formation.get(0).unwrap(), 0);
        let mog = party.remove_character(0);
        assert_eq!(mog.whoami(), "Mog");
        assert_eq!(party.group.len(), 0);
        assert_eq!(party.formation.len(), 0);
    }
}
