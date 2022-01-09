use termion::clear::All as ClearAll;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::style;
use std::io::Write;


use crate::character::Character;
use crate::character::resolve as resolve_character;
use crate::encyclopedia::Encyclopedia;
use crate::party::Party;


const OUTER_ROW: &'static str = " ============================== ";
const INNER_ROW: &'static str = " |                            | ";
const BOX_HEIGHT: u16 = 8;
const BOX_WIDTH: u16 = 32;

pub struct BattleCLI<R, W: Write> {
    pub stdin: R,
    pub stdout: W,
}

impl<R, W: Write> Drop for BattleCLI<R, W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        write!(self.stdout, "{}{}{}", ClearAll, style::Reset, Goto(1, 1)).unwrap();
    }
}
impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> BattleCLI<R, W> {
    fn clear(&mut self) {
        write!(self.stdout, "{}{}", ClearAll, Goto(1, 1)).unwrap();
    }
    fn _draw_boxes(&mut self, n: usize) {
        for _ in 0..n {
            write!(self.stdout, "{}", OUTER_ROW).unwrap();
        }
        write!(self.stdout, "\r\n").unwrap();
        for _ in 2..=BOX_HEIGHT {
            for _ in 0..n {
                write!(self.stdout, "{}", INNER_ROW).unwrap();
            }
            write!(self.stdout, "\r\n").unwrap();
        }
        for _ in 0..n {
            write!(self.stdout, "{}", OUTER_ROW).unwrap();
        }
        write!(self.stdout, "\r\n").unwrap();
    }
    fn draw_boxes(&mut self, num_baddies: usize, num_allies: usize) {
        self.clear();
        self._draw_boxes(num_baddies);
        write!(self.stdout, "\n\n\n").unwrap();
        self._draw_boxes(num_allies);
    }
    fn write_party_info(&mut self, p: &Party, y: u16, ch_enc: &Encyclopedia<Character>) {
/*
        for i in &p.formation {
            let c = p.group.get(i).unwrap();
            */
        for i in 0..p.len() {
            let c = resolve_character(p.get_character(i), ch_enc).unwrap();
            let (_, name) = c.whoami();
            let i: u16 = i.try_into().unwrap();
            write!(self.stdout, "{} {}", Goto(i*BOX_WIDTH + 3, y+1), name).unwrap();
            write!(self.stdout, "{} HP: ", Goto(i*BOX_WIDTH + 3, y+2)).unwrap(); // todo
        }
    }
    pub fn get_key(&mut self) {
        self.stdout.flush().unwrap();
        self.stdin.next().unwrap().unwrap();
    }
}

pub struct Battle<R, W: Write> {
    pub allies: Party,
    pub baddies: Party,
    pub cli: BattleCLI<R, W>,
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Battle<R, W> {
    pub fn run(&mut self, ch_enc: &Encyclopedia<Character>) {
        self.cli.clear();
        self.cli.draw_boxes(self.baddies.len(), self.allies.len());
        self.cli.write_party_info(&self.baddies, 1, ch_enc);
        self.cli.write_party_info(&self.allies, BOX_HEIGHT+5, ch_enc);
        self.cli.get_key(); // pause
    }
}
