use termion::clear::All as ClearAll;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::style;
use std::io::Write;

use crate::battle::Battle;
use crate::character::Character;
use crate::encyclopedia::CharacterEncyclopedia;
use crate::party::Party;

const OUTER_ROW: &'static str = " ============================== ";
const INNER_ROW: &'static str = " |                            | ";
const BOX_HEIGHT: u16 = 8;
const BOX_WIDTH: u16 = 32;

pub struct BattleCLI<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> {
    pub stdin: R,
    pub stdout: W,
    pub battle: Battle,
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Drop for
BattleCLI<R, W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        write!(self.stdout, "{}{}{}", ClearAll, style::Reset, Goto(1, 1)).unwrap();
    }
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write>
BattleCLI<R, W> {
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
    fn write_baddies_info(&mut self, ch_enc: &CharacterEncyclopedia) {
        let p = &self.battle.baddies;
        for i in 0..p.len() {
            let c = ch_enc.resolve(p.get_character(i)).unwrap();
            let (_, name) = c.whoami();
            let i: u16 = i.try_into().unwrap();
            write!(self.stdout, "{} {}", Goto(i*BOX_WIDTH + 3, 2), name).unwrap();
            write!(self.stdout, "{} HP: ", Goto(i*BOX_WIDTH + 3, 3)).unwrap(); // todo
        }
    }
    fn write_allies_info(&mut self, ch_enc: &CharacterEncyclopedia) {
        let p = &self.battle.allies;
        for i in 0..p.len() {
            let c = ch_enc.resolve(p.get_character(i)).unwrap();
            let (_, name) = c.whoami();
            let i: u16 = i.try_into().unwrap();
            write!(self.stdout, "{} {}", Goto(i*BOX_WIDTH + 3, BOX_HEIGHT+4+2), name).unwrap();
            write!(self.stdout, "{} HP: ", Goto(i*BOX_WIDTH + 3, BOX_HEIGHT+4+3)).unwrap(); // todo
        }
    }
    pub fn get_key(&mut self) {
        self.stdout.flush().unwrap();
        self.stdin.next().unwrap().unwrap();
    }
    pub fn run(&mut self, ch_enc: &CharacterEncyclopedia) {
        self.clear();
        self.draw_boxes(self.battle.baddies.len(), self.battle.allies.len());
        self.write_baddies_info(ch_enc);
        self.write_allies_info(ch_enc);
        self.get_key(); // pause
    }
}
