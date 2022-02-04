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
use rpgrs::party::Party;


fn bcli_test<R: Read, W: Write>(stdin: R, stdout: W, ch_enc: &CharacterEncyclopedia) {
    let mut allies = Party::new("Allies".to_string());
    allies.add_clone(&IndexedOrLiteral::Index(515), ch_enc);
    allies.add_clone(&IndexedOrLiteral::Index(521), ch_enc);
    allies.add_clone(&IndexedOrLiteral::Index(619), ch_enc);

    let mut baddies = Party::new("Baddies".to_string());
    baddies.add_clone(&IndexedOrLiteral::Index(101), ch_enc);
    baddies.add_clone(&IndexedOrLiteral::Index(102), ch_enc);
    baddies.add_clone(&IndexedOrLiteral::Index(101), ch_enc);

    let battle = Battle::new(allies, baddies);
    let mut cli = BattleCLI {
        stdin: stdin.keys(),
        stdout,
        battle,
    };
    cli.run();
}

fn main() {
    let phonebook = CharacterEncyclopedia::new("data/characters.json");

    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w - 2).unwrap();
    let termheight = termsize.map(|(_,h)| h - 2).unwrap();

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    // We go to raw mode to make the control over the terminal more fine-grained.
    let stdout = stdout.into_raw_mode().unwrap();

    bcli_test(stdin, stdout, &phonebook);
/*
    let allies = Party::new("Allies");
    let baddies = Party::new("Baddies");
    let mut battle = battle::Battle {allies, baddies}
*/
    
    //let answer = menu_test();
    //print!("{}Got answer {}", Goto(15, 15), answer);
    print!("{}{}{}", ClearAll, termion::style::Reset, Goto(1, 1));
    println!("Terminal width, height is ({}, {})", termwidth, termheight);
}
