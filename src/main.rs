use std::collections::HashMap;
use std::io;
use std::io::{Read, Write};

use termion::clear::All as ClearAll;
use termion::cursor::Goto;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
//use termion::style;

use rpgrs::battle::Battle;
use rpgrs::battlecli::BattleCLI;
use rpgrs::common::*;
use rpgrs::encyclopedia::CharacterEncyclopedia;
use rpgrs::encyclopedia::SpriteEncyclopedia;
use rpgrs::map::Map;
use rpgrs::party::Party;
use rpgrs::scene::Scene;
use rpgrs::scenecli::SceneCLI;
use rpgrs::sprite::Sprite;

fn bcli_test<R: Read, W: Write>(stdin: R, stdout: W, ch_enc: &CharacterEncyclopedia) {
    let mut allies = Party::new("Allies".to_string());
    allies.add_clone(&IndexedOrLiteral::Index(515), ch_enc);
    allies.add_clone(&IndexedOrLiteral::Index(521), ch_enc);
    allies.add_clone(&IndexedOrLiteral::Index(619), ch_enc);
    allies.add_clone(&IndexedOrLiteral::Index(8330), ch_enc);

    let mut baddies = Party::new("Baddies".to_string());
    baddies.add_clone(&IndexedOrLiteral::Index(101), ch_enc);
    baddies.add_clone(&IndexedOrLiteral::Index(102), ch_enc);
    baddies.add_clone(&IndexedOrLiteral::Index(102), ch_enc);
    baddies.add_clone(&IndexedOrLiteral::Index(101), ch_enc);

    let mut battle = Battle::new(allies, baddies);
    let mut cli = BattleCLI {
        stdin: stdin.keys(),
        stdout,
    };
    battle.run(&mut cli);
}
fn bcli_test_boss<R: Read, W: Write>(stdin: R, stdout: W, ch_enc: &CharacterEncyclopedia) {
    let mut allies = Party::new("Allies".to_string());
    allies.add_clone(&IndexedOrLiteral::Index(515), ch_enc);
    allies.add_clone(&IndexedOrLiteral::Index(521), ch_enc);
    allies.add_clone(&IndexedOrLiteral::Index(619), ch_enc);
    allies.add_clone(&IndexedOrLiteral::Index(1007), ch_enc);

    let mut baddies = Party::new("Baddies".to_string());
    baddies.add_clone(&IndexedOrLiteral::Index(103), ch_enc);

    let mut battle = Battle::new(allies, baddies);
    let mut cli = BattleCLI {
        stdin: stdin.keys(),
        stdout,
    };
    battle.run(&mut cli);
}

fn easy_fight(phonebook: &CharacterEncyclopedia) {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    // We go to raw mode to make the control over the terminal more fine-grained.
    let stdout = stdout.into_raw_mode().unwrap();

    bcli_test(stdin, stdout, &phonebook);
}

fn boss_fight(phonebook: &CharacterEncyclopedia) {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    // We go to raw mode to make the control over the terminal more fine-grained.
    let stdout = stdout.into_raw_mode().unwrap();

    bcli_test_boss(stdin, stdout, &phonebook);
}

fn scenecli_test() {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    // This is necessary to get individual keys without them being written first.
    let stdout = stdout.into_raw_mode().unwrap();

    let encoded_map = vec![vec![1; 3], vec![1, 0, 1], vec![1; 3]];
    let _sprites = HashMap::from([(0, Sprite::new_solid(' ')), (1, Sprite::new_solid('-'))]);
    let sprite_code = SpriteEncyclopedia { en: _sprites };
    let test_map = Map {
        encoded_map,
        sprite_code,
    };
    let mut scene = Scene::new(test_map);
    let mut cli = SceneCLI {
        stdin: stdin.keys(),
        stdout,
    };
    scene.run(&mut cli);
}

fn main() {
    let phonebook = CharacterEncyclopedia::new("data/characters.json");

    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w, _)| w - 2).unwrap();
    let termheight = termsize.map(|(_, h)| h - 2).unwrap();

    easy_fight(&phonebook);
    boss_fight(&phonebook);

    scenecli_test();

    print!("{}{}{}", ClearAll, termion::style::Reset, Goto(1, 1));
    println!("Terminal width, height is ({}, {})", termwidth, termheight);
}
