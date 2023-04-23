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

fn scenecli_test(display_size: uXY) {
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    // This is necessary to get individual keys without them being written first.
    let stdout = stdout.into_raw_mode().unwrap();

    let origin = (0, 0);
    let encoded_map = vec![
        vec![4,0,4,0,4,0,4],
        vec![0,4,2,1,3,4,0],
        vec![4,0,6,7,6,0,4],
        vec![0,4,6,7,6,4,0],
        vec![5,5,6,7,6,5,5],
    ];
    let _sprites = HashMap::from([
        (0, Sprite::new_solid(' ')),
        (1, Sprite::new_solid('-')),
        (2, Sprite::new_solid('(')),
        (3, Sprite::new_solid(')')),
        (4, Sprite::new_solid('~')),
        (5, Sprite::new_solid('=')),
        (6, Sprite::new_solid('|')),
        (7, Sprite::new_solid('.')),
    ]);
    let sprite_code = SpriteEncyclopedia { en: _sprites };
    let test_map = Map {
        dim: (7, 5),
        encoded_map,
        sprite_code,
        origin,
    };
    let tower_map = Map::from_files("./data/maps/tower.encoded", "./data/maps/tower.sprites");
    let mut scene = Scene::new(tower_map, (21, 27));
    let mut cli = SceneCLI {
        stdin: stdin.keys(),
        stdout,
        display_size,
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

    let display_size = (60, 20);
    scenecli_test(display_size);

    print!("{}{}{}", ClearAll, termion::style::Reset, Goto(1, 1));
    println!("Terminal width, height is ({}, {})", termwidth, termheight);
}
