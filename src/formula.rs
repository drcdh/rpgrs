use std::collections::VecDeque;

use crate::character::Character;
use crate::common::{Formula, Name};
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::stats::{DerivedStat, Stat};

pub fn eval_stat(stat_name: Name, s: &DerivedStat, c: &Character) -> Stat {
    let mut tokens = s.split(' ').collect::<VecDeque<_>>();
    _eval_stat(&mut tokens, c, &stat_name)
}

fn _eval_stat(tokens: &mut VecDeque<&str>, c: &Character, stat_name: &Name) -> Stat {
    match tokens.pop_front() {
        Some("+") => _eval_stat(tokens, c, stat_name).saturating_add(_eval_stat(tokens, c, stat_name)),
        Some("-") => _eval_stat(tokens, c, stat_name).saturating_sub(_eval_stat(tokens, c, stat_name)),
        Some("*") => _eval_stat(tokens, c, stat_name).saturating_mul(_eval_stat(tokens, c, stat_name)),
        Some("/") => _eval_stat(tokens, c, stat_name).saturating_div(_eval_stat(tokens, c, stat_name)),
//        Some("^") => _eval_stat(tokens, c, stat_name).saturating_pow(_eval_stat(tokens, c, stat_name)),
        Some(term) => _eval_stat_term(term, c, stat_name),
        None => panic!("Ran out of tokens in _eval_stat"),
    }
}

fn _eval_stat_term(term: &str, c: &Character, base_stat_name: &Name) -> Stat {
    if let Ok(v) = term.parse::<Stat>() {
        return v;
    }
    let tokens = term.split('.').collect::<Vec<_>>();
    if let Some(item_attr) = c.get_item_attr(Name::from(tokens[0]), Name::from(tokens[1])) {
        return item_attr;
    }
    let slot_or_stat_name: Name = if tokens[0].is_empty() { Name::from(base_stat_name) } else { Name::from(tokens[0]) };
    let slot_or_stat_name = slot_or_stat_name.replace("-", " ");
    if tokens[1].is_empty() {
        return *c.get_base_stat(slot_or_stat_name).unwrap();
    }
    match tokens[1] {
        "AddMod" => c.sum_add_mods(slot_or_stat_name),
        "MultMod" => c.sum_mult_mods(slot_or_stat_name),
        "Power" => c.get_item_attr(slot_or_stat_name, String::from("Power")).unwrap(),
        m => panic!("Could not understand statistic attribute {}.", m),
    }
}

pub fn eval_hit(f: &Formula, actor: Option<&Character>, target: &Character, statblocks: &StatBlockEncyclopedia) -> Stat {
    let mut tokens = f.split(' ').collect::<VecDeque<_>>();
    _eval_hit(&mut tokens, actor, target, statblocks)
}

fn _eval_hit(tokens: &mut VecDeque<&str>, actor: Option<&Character>, target: &Character, statblocks: &StatBlockEncyclopedia) -> Stat {
    match tokens.pop_front() {
        Some("+") => _eval_hit(tokens, actor, target, statblocks).saturating_add(_eval_hit(tokens, actor, target, statblocks)),
        Some("-") => _eval_hit(tokens, actor, target, statblocks).saturating_sub(_eval_hit(tokens, actor, target, statblocks)),
        Some("*") => _eval_hit(tokens, actor, target, statblocks).saturating_mul(_eval_hit(tokens, actor, target, statblocks)),
        Some("/") => _eval_hit(tokens, actor, target, statblocks).saturating_div(_eval_hit(tokens, actor, target, statblocks)),
//        Some("^") => _eval_hit(tokens, actor, target, statblocks).saturating_pow(_eval_hit(tokens, actor, target, statblocks)),
        Some(term) => _eval_term(term, actor, target, statblocks),
        None => panic!("Ran out of tokens in _eval_hit"),
    }
}

fn _eval_term(term: &str, actor: Option<&Character>, target: &Character, statblocks: &StatBlockEncyclopedia) -> Stat {
    if let Ok(v) = term.parse::<Stat>() {
        return v;
    }
    let actor = actor.unwrap();
    // todo generalizations
    match term {
        "^Level" => actor.get_stat_val(String::from("Level"), 1, statblocks),
        "^Offense" => actor.get_stat_val(String::from("Offense"), 0, statblocks),
        "^Strength" => actor.get_stat_val(String::from("Strength"), 0, statblocks),
        "^Magic" => actor.get_stat_val(String::from("Magic"), 0, statblocks),
        "$Offense" => target.get_stat_val(String::from("Offense"), 0, statblocks),
        "$Defense" => target.get_stat_val(String::from("Defense"), 0, statblocks),
        "$Magic-Defense" => target.get_stat_val(String::from("Magic Defense"), 0, statblocks),
        t => panic!("Could not understand token {}.", t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_stat_term_test() {
        let term = "1";
        let c = Character::from_json(r#"{"id": 0, "name": "Test"}"#);
        assert_eq!(_eval_stat_term(term, &c, &String::new()), 1);
    }
    #[test]
    fn eval_hit_test() {
        let f = Formula::from("+ 1 - ^Offense / $Offense 2");
        let c = Character::from_json(r#"{"id": 0, "name": "Test"}"#);
        let statblocks = StatBlockEncyclopedia::new("data/stats.json");
        let offense: Stat = c.get_stat_val(String::from("Offense"), 0, &statblocks);
        let expected: Stat = 1 + offense - offense/2;
        let evaluated = eval_hit(&f, Some(&c), &c, &statblocks);
        assert_eq!(evaluated, expected);
    }
    #[test]
    fn eval_stat_test() {
        let ds = DerivedStat::from("+ .AddMod * .MultMod + Weapon.power .");
        let c = Character::from_json(r#"{"id": 0, "name": "Test"}"#);
        //let statblocks = StatBlockEncyclopedia::new("data/stats.json");
        let offense: Stat = *c.get_base_stat(String::from("Offense")).unwrap();
        let expected: Stat = offense + 10; // todo, see Character::get_item_attr
        let evaluated = eval_stat(Name::from("Offense"), &ds, &c);
        assert_eq!(evaluated, expected);
    }
    #[test]
    #[should_panic]
    fn incomplete_hit_formula_test() {
        let f = Formula::from("+ 1");
        let c = Character::from_json(r#"{"id": 0, "name": "Test"}"#);
        let statblocks = StatBlockEncyclopedia::new("data/stats.json");
        eval_hit(&f, Some(&c), &c, &statblocks);
    }
    #[test]
    #[should_panic]
    fn bad_hit_formula_test() {
        let f = Formula::from("+ 1 $Moxie");
        let c = Character::from_json(r#"{"id": 0, "name": "Test"}"#);
        let statblocks = StatBlockEncyclopedia::new("data/stats.json");
        eval_hit(&f, Some(&c), &c, &statblocks);
    }
    #[test]
    #[should_panic]
    fn incomplete_stat_formula_test() {
        let ds = DerivedStat::from("+ 1");
        let c = Character::from_json(r#"{"id": 0, "name": "Test"}"#);
        eval_stat(Name::new(), &ds, &c);
    }
    #[test]
    #[should_panic]
    fn bad_stat_formula_test() {
        let ds = DerivedStat::from("+ 1 Moxie.");
        let c = Character::from_json(r#"{"id": 0, "name": "Test"}"#);
        eval_stat(Name::new(), &ds, &c);
    }
    #[test]
    #[should_panic]
    fn bad_attr_in_stat_formula_test() {
        let ds = DerivedStat::from("+ 1 Moxie.Blarg");
        let c = Character::from_json(r#"{"id": 0, "name": "Test"}"#);
        eval_stat(Name::new(), &ds, &c);
    }
}
