use std::collections::VecDeque;

use crate::character::Character;
use crate::common::Formula;
use crate::encyclopedia::StatBlockEncyclopedia;
use crate::stats::Stat;

pub fn eval_formula(f: &Formula, actor: &Character, target: &Character, statblocks: &StatBlockEncyclopedia) -> Stat {
    let mut tokens = f.split(' ').collect::<VecDeque<_>>();
    _eval_formula(&mut tokens, actor, target, statblocks)
}

fn _eval_formula(tokens: &mut VecDeque<&str>, actor: &Character, target: &Character, statblocks: &StatBlockEncyclopedia) -> Stat {
    match tokens.pop_front() {
        Some("+") => _eval_formula(tokens, actor, target, statblocks).saturating_add(_eval_formula(tokens, actor, target, statblocks)),
        Some("-") => _eval_formula(tokens, actor, target, statblocks).saturating_sub(_eval_formula(tokens, actor, target, statblocks)),
        Some("*") => _eval_formula(tokens, actor, target, statblocks).saturating_mul(_eval_formula(tokens, actor, target, statblocks)),
        Some("/") => _eval_formula(tokens, actor, target, statblocks).saturating_div(_eval_formula(tokens, actor, target, statblocks)),
//        Some("^") => _eval_formula(tokens, actor, target, statblocks).saturating_pow(_eval_formula(tokens, actor, target, statblocks)),
        Some(term @ String) => _eval_term(term, actor, target, statblocks),
        None => panic!("Ran out of tokens in _eval_formula"),
    }
}

fn _eval_term(term: &str, actor: &Character, target: &Character, statblocks: &StatBlockEncyclopedia) -> Stat {
    if let Ok(v) = term.parse::<Stat>() {
        return v;
    }
    // todo generalizations
    match term {
        "^Offense" => actor.get_stat_val(String::from("Offense"), 0, statblocks).into(),
        "$Offense" => target.get_stat_val(String::from("Offense"), 0, statblocks).into(),
        t @ _ => panic!("Could not understand token {}.", t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_formula_test() {
        let f = Formula::from("+ 1 - ^Offense / $Offense 2");
        let c = Character::from_json(r#"{"id": 0, "name": "Test"}"#);
        let statblocks = StatBlockEncyclopedia::new("data/stats.json");
        let offense: Stat = c.get_stat_val(String::from("Offense"), 0, &statblocks);
        let expected: Stat = 1 + offense - offense/2;
        let evaluated = eval_formula(&f, &c, &c, &statblocks);
        assert_eq!(evaluated, expected);
    }
}
