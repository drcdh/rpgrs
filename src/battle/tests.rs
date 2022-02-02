use super::*;
use crate::encyclopedia::CharacterEncyclopedia;

fn get_test_battle(num_allies: usize, num_baddies: usize) -> Battle {
    let ch_enc = CharacterEncyclopedia::new("data/characters.json");
    let test_dummy_iol = IndexedOrLiteral::Index(0);
    let mut allies = Party::new(String::from("Allies"));
    let mut baddies = Party::new(String::from("Baddies"));
    for _ in 0..num_allies {
        allies.add_clone(&test_dummy_iol, &ch_enc);
    }
    for _ in 0..num_baddies {
        baddies.add_clone(&test_dummy_iol, &ch_enc);
    }
    Battle::new(allies, baddies)
}

#[test]
fn new_test() {
    let mut battle = get_test_battle(0, 0);
    assert!(battle.get_character(&None).is_none());
    assert!(battle.get_mut_character(&None).is_none());
    assert!(battle.get_top_menu_options().is_none());
    assert!(battle.get_text().is_some());
    assert!(battle.pop_text().is_some());
    assert!(battle.get_text().is_none());
    assert!(battle.pop_text().is_none());
    assert!(battle.get_current_npc().is_none());
    assert!(battle.get_current_pc().is_none());
//    assert!(!battle.get_current_pc_actions().is_empty());
    assert!(battle.get_top_menu_options().is_none());
    assert!(battle.get_selected_action().is_none());
    assert!(battle.get_target_names().is_empty());
}

#[test]
fn get_character_test() {
    let mut battle = get_test_battle(2, 2);
    assert!(battle.get_character(&Some(PlayerIndex::Ally(0))).is_some());
    assert!(battle.get_character(&Some(PlayerIndex::Ally(1))).is_some());
    assert!(battle.get_character(&Some(PlayerIndex::Ally(2))).is_none());
    assert!(battle.get_character(&Some(PlayerIndex::Baddy(0))).is_some());
    assert!(battle.get_character(&Some(PlayerIndex::Baddy(1))).is_some());
    assert!(battle.get_character(&Some(PlayerIndex::Baddy(2))).is_none());
    assert!(battle.get_mut_character(&Some(PlayerIndex::Ally(0))).is_some());
    assert!(battle.get_mut_character(&Some(PlayerIndex::Ally(1))).is_some());
    assert!(battle.get_mut_character(&Some(PlayerIndex::Ally(2))).is_none());
    assert!(battle.get_mut_character(&Some(PlayerIndex::Baddy(0))).is_some());
    assert!(battle.get_mut_character(&Some(PlayerIndex::Baddy(1))).is_some());
    assert!(battle.get_mut_character(&Some(PlayerIndex::Baddy(2))).is_none());
}

#[test]
fn turn_start_test() {
    let mut battle = get_test_battle(1, 1);
    while battle.pop_text().is_some() { }
    battle.force_turn(PlayerIndex::Ally(0));
    battle.selections.push(0);
    assert!(battle.get_current_pc().is_some());
//    assert!(!battle.get_current_pc_actions().is_empty());
    assert!(battle.get_top_menu_options().is_some());
    battle.force_turn(PlayerIndex::Baddy(0));
    assert!(battle.get_current_npc().is_some());
}

#[test]
fn next_turn_test() {
    let mut battle = get_test_battle(1, 1);
    battle.next_turn();
    assert!(battle.get_current_npc().is_some() || battle.get_current_pc().is_some());
}
