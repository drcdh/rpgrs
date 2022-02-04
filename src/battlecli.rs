use termion::clear::All as ClearAll;
use termion::color;
use termion::cursor::Goto;
use termion::event::Key;
use termion::style;
use std::io::Write;

use crate::battle::{Battle, PlayerIndex};

const OUTER_ROW: &str = r" ============================== ";
const INNER_ROW: &str = r" |                            | ";
const TARGET_OUTER_ROW: &str = r" |\/\/\/\/\/\/\/\/\/\/\/\/\/\/| ";
const TARGET_INNER_ROW: &str = r" >                            < ";
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
    fn refresh(&mut self) {
        self.clear();
        self.draw_boxes(self.battle.baddies.len(), self.battle.allies.len());
        self.write_baddies_info();
        self.write_allies_info();
        self.write_text();
        self.write_menu();
    }
    fn write_text(&mut self) -> bool {
        if let Some(text) = self.battle.get_text() {
            write!(self.stdout, "{} >>> {}", Goto(1, 30), text).unwrap();
        }
        false
    }
    pub fn write_menu(&mut self) {
        if let Some(options) = self.battle.get_top_menu_options() {
            for (i, opt) in options.iter().enumerate() {
                let mut sel_str = "    ";
                if i == *self.battle.selections.last().unwrap() {
                    sel_str = " -> ";
                }
                write!(self.stdout, "{}{}{}. {}", Goto(1, (30 + i) as u16), sel_str, i+1, opt).unwrap();
            }
            write!(self.stdout, "{} >>> Pick your next action! ", Goto(1, (30 + options.len() + 2) as u16)).unwrap();
        }
    }
    fn _draw_boxes(&mut self, n: usize, baddies: bool) {
        // todo: for now this overwrites all character info
        for i in 0..n {
            let pi = if baddies { PlayerIndex::Baddy(i) } else { PlayerIndex::Ally(i) };
            let mut row = if self.battle.targets.contains(&pi) { String::from(TARGET_OUTER_ROW) } else { String::from(OUTER_ROW) };
            if self.battle.is_player_down(&pi) {
                row = format!("{}{}{}", color::Fg(color::Red), row, color::Fg(color::Reset));
            }
            write!(self.stdout, "{}", row).unwrap();
        }
        write!(self.stdout, "\r\n").unwrap();
        for _ in 2..=BOX_HEIGHT {
            for i in 0..n {
                let pi = if baddies { PlayerIndex::Baddy(i) } else { PlayerIndex::Ally(i) };
                let mut row = if self.battle.targets.contains(&pi) { String::from(TARGET_INNER_ROW) } else { String::from(INNER_ROW) };
                if self.battle.is_player_down(&pi) {
                    row = format!("{}{}{}", color::Fg(color::Red), row, color::Fg(color::Reset));
                }
                write!(self.stdout, "{}", row).unwrap();
            }
            write!(self.stdout, "\r\n").unwrap();
        }
        for i in 0..n {
            let pi = if baddies { PlayerIndex::Baddy(i) } else { PlayerIndex::Ally(i) };
            let mut row = if self.battle.targets.contains(&pi) { String::from(TARGET_OUTER_ROW) } else { String::from(OUTER_ROW) };
            if self.battle.is_player_down(&pi) {
                row = format!("{}{}{}", color::Fg(color::Red), row, color::Fg(color::Reset));
            }
            write!(self.stdout, "{}", row).unwrap();
        }
        write!(self.stdout, "\r\n").unwrap();
    }
    fn draw_boxes(&mut self, num_baddies: usize, num_allies: usize) {
//        self.clear();
        self._draw_boxes(num_baddies, true);
        write!(self.stdout, "\n\n\n").unwrap();
        self._draw_boxes(num_allies, false); // fixme
    }
    fn write_baddies_info(&mut self) {
        let p = &self.battle.baddies;
        for i in 0..p.len() {
            let c = p.get_ch_by_pos(i).unwrap();
            let name = c.whoami().1;
            let i: u16 = i.try_into().unwrap();
            write!(self.stdout, "{} {}", Goto(i*BOX_WIDTH + 3, 2), name).unwrap();
            for (j, (_, pool)) in c.get_pools().iter().enumerate() {
                let j: u16 = j.try_into().unwrap();
                write!(self.stdout, "{} {:>4}: {:4} / {:4}", Goto(i*BOX_WIDTH + 3, 4+j), pool.name, pool.current, pool.maximum).unwrap();
            }
            //write!(self.stdout, "{} {}", Goto(i*BOX_WIDTH + 3, 7), p.clocks.get(i as usize).unwrap()).unwrap();
        }
    }
    fn write_allies_info(&mut self) {
        let p = &self.battle.allies;
        for i in 0..p.len() { // todo: ch_iter
            let c = p.get_ch_by_pos(i).unwrap();
            let name = c.whoami().1;
            let i: u16 = i.try_into().unwrap();
            write!(self.stdout, "{} {}", Goto(i*BOX_WIDTH + 3, BOX_HEIGHT+4+2), name).unwrap();
            for (j, (_, pool)) in c.get_pools().iter().enumerate() {
                let j: u16 = j.try_into().unwrap();
                write!(self.stdout, "{} {:>4}: {:4} / {:4}", Goto(i*BOX_WIDTH + 3, BOX_HEIGHT+4+4+j), pool.name, pool.current, pool.maximum).unwrap();
            }
            //write!(self.stdout, "{} {}", Goto(i*BOX_WIDTH + 3, BOX_HEIGHT+4+7), p.clocks.get(i as usize).unwrap()).unwrap();
        }
    }
    pub fn get_key(&mut self) -> Key {
        self.stdout.flush().unwrap();
        self.stdin.next().unwrap().unwrap()
    }
    pub fn run(&mut self) {
        loop {
            self.refresh();
            let key = self.get_key();
            if key == Key::Char('q') {
                break;
            }
            if let Key::Char(_c) = key {
                // Collect it as entropy
//                self.rand.write_u8(c as u8);
            }
            if self.battle.handle_input(key) {
                break;
            }
        }
    }
}
