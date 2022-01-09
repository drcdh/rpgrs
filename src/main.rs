extern crate termion;

use std::io;
use std::io::{Read, Write};

use termion::clear::All as ClearAll;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::rpgrs::rpg::battle::{Battle, BattleCLI};

struct Menu<R: Read, W: Write> {
    options: Vec<String>,
    prompt: String,
    x: u16,
    y: u16,
    stdin: R,
    stdout: W,
}

impl<R: Read, W: Write> Menu<R, W> {
    pub fn show_and_get_answer(&mut self) -> String {
        let mut i = 0;
        for opt in &self.options {
            write!(self.stdout, "{}{}. {}", Goto(self.x, self.y + i), i+1, opt).unwrap();
            i += 1;
        }
        write!(self.stdout, "{}{} ", Goto(self.x, self.y + i), self.prompt).unwrap();
        self.stdout.flush().unwrap();
        self.stdin.read_line().unwrap().unwrap()
    }
}

fn main() {
    let termsize = termion::terminal_size().ok();
    let termwidth = termsize.map(|(w,_)| w - 2);
    let termheight = termsize.map(|(_,h)| h - 2);

    println!("Terminal width, height is ({}, {})", termwidth, termheight);

    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdin = io::stdin();
    let stdin = stdin.lock();

    // We go to raw mode to make the control over the terminal more fine-grained.
    let stdout = stdout.into_raw_mode().unwrap();

    let mut bcli = BattleCLI{ stdin, stdout };
    bcli.draw_windows(1, 1);

/*
    let allies = Party::new("Allies");
    let baddies = Party::new("Baddies");
    let mut battle = battle::Battle {allies, baddies}
*/
    
    //let answer = menu_test();
    //print!("{}Got answer {}", Goto(15, 15), answer);
    print!("{}{}{}", ClearAll, termion::style::Reset, Goto(1, 1));
}

fn menu_test() -> String {
    // Get and lock the stdios.
    let stdout = io::stdout();
    let stdout = stdout.lock();
    let stdin = io::stdin();
    //let stdin = stdin.lock();

    // We go to raw mode to make the control over the terminal more fine-grained.
    let stdout = stdout.into_raw_mode().unwrap();
    _menu_test(stdin, stdout)
}

fn _menu_test<R: Read, W: Write>(stdin: R, mut stdout: W) -> String {
    write!(stdout, "{}{}Oh hai.", ClearAll, Goto(1, 1)).unwrap();

    let mut menu = Menu {
        options: vec!["Blarg!".to_string(), "Booga?".to_string()],
        prompt: "What do?".to_string(),
        x: 40,
        y: 10,
        stdin,
        stdout,
    };
    menu.show_and_get_answer()
} 
