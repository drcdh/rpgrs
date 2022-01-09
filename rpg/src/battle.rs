extern crate termion;
use termion::clear::All as ClearAll;
use termion::cursor::Goto;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use std::io;
use std::io::{Read, Write};

use crate::party::Party;


const BOX_SEPARATION: u16 = 2;
const OUTER_ROW: &'static str = " ================== ";
const INNER_ROW: &'static str = " |                | ";
const BOX_HEIGHT: u16 = 8;

struct BattleCLI<R: Read, W: Write> {
    stdin: R,
    stdout: W,
}

impl<R: Read, W: Write> BattleCLI<R, W> {
    fn clear(&mut self) {
        write!(self.stdout, "{}{}", ClearAll, Goto(1, 1)).unwrap();
    }
    fn draw_boxes(&mut self, n: u16) {
        for _ in 0..n {
            write!(self.stdout, OUTER_ROW);
        }
        writeln!(self.out);
        for _ in 2..=BOX_HEIGHT {
            for _ in 0..n {
                write!(self.stdout, INNER_ROW);
            }
        }    
        writeln!(self.out);
        for _ in 0..n {
            write!(self.stdout, OUTER_ROW);
        }
        writeln!(self.stdout);
    }
    pub fn draw_windows(&mut self, num_baddies: u16, num_allies: u16) {
        self.clear();
        self.draw_boxes(num_baddies);
        write!(self.out, "\n\n\n");
        self.draw_boxes(num_allies);
        self.stdin.next().unwrap().unwrap();
    }
}

pub struct Battle {
    allies: &Party,
    baddies: &Party,
}

impl Battle {
}
