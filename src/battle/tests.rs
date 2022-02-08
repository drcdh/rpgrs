use super::*;
use crate::encyclopedia::CharacterEncyclopedia;
use crate::encyclopedia::EffectEncyclopedia;

fn get_test_parties(num_allies: usize, num_baddies: usize) -> (Party, Party) {
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
    (allies, baddies)
}

fn get_test_battle(num_allies: usize, num_baddies: usize) -> Battle {
    let (mut allies, mut baddies) = get_test_parties(num_allies, num_baddies);
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

#[test]
fn handle_effect_test() {
    let effect_enc = EffectEncyclopedia::new("data/effects.json");
    let (allies, baddies) = get_test_parties(2, 2);
    let mut effects = VecDeque::<TargetedEffect>::new();
    effects.push_back(TargetedEffect {
        actor_pi: PlayerIndex::Ally(0),
        target_pi: PlayerIndex::Baddy(0),
        effect: effect_enc.get(&735730).unwrap().clone(),
    });
    let mut battle = Battle {
            allies,
            baddies,
            ended: false,
            selections: Vec::<usize>::new(),
            text: VecDeque::<String>::new(),
            current_pc_idx: None,
            current_npc_idx: None,
            targets: Vec::<PlayerIndex>::new(),
            effects,
            hits: VecDeque::<TargetedHit>::new(),
            // FIXME: references should be supplied by the top-level Game object
            action_enc: ActionEncyclopedia::new("data/actions.json"),
            effect_enc: EffectEncyclopedia::new("data/effects.json"),
            statblocks: StatBlockEncyclopedia::new("data/stats.json"),
    };
    battle.handle_effect();
    assert!(!battle.hits.is_empty());
}

#[test]
fn handle_hit_test() {
    let (allies, baddies) = get_test_parties(2, 2);
    let mut hits = VecDeque::<TargetedHit>::new();
    hits.push_back(TargetedHit {
        target_pi: PlayerIndex::Baddy(0),
        pool: String::from("HP"),
        amount: 5i32,
    });
    let mut battle = Battle {
            allies,
            baddies,
            ended: false,
            selections: Vec::<usize>::new(),
            text: VecDeque::<String>::new(),
            current_pc_idx: None,
            current_npc_idx: None,
            targets: Vec::<PlayerIndex>::new(),
            effects: VecDeque::<TargetedEffect>::new(),
            hits,
            // FIXME: references should be supplied by the top-level Game object
            action_enc: ActionEncyclopedia::new("data/actions.json"),
            effect_enc: EffectEncyclopedia::new("data/effects.json"),
            statblocks: StatBlockEncyclopedia::new("data/stats.json"),
    };
    battle.handle_hit();
    assert!(!battle.text.is_empty());
}
