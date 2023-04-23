use std::io::Write;
use termion::clear::All as ClearAll;
use termion::color;
use termion::cursor::Goto;
use termion::event::Key;
use termion::style;

use crate::battle::battleui::BattleUI;
use crate::battle::{Battle, PlayerIndex};
use crate::party::Party;

const OUTER_ROW: &str = r" =================================== ";
const INNER_ROW: &str = r" |                                 | ";
const TURN_OUTER_ROW: &str = r" @=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=@ ";
const TURN_INNER_ROW: &str = r" !                                 ! ";
const TARGET_OUTER_ROW: &str = r" |\/\/\/\/\/\/\/\/|\/\/\/\/\/\/\/\/| ";
const TARGET_INNER_ROW: &str = r" >                                 < ";
const BOX_HEIGHT: u16 = 8;
const BOX_WIDTH: u16 = 37;

pub struct BattleCLI<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> {
    pub stdin: R,
    pub stdout: W,
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> Drop for BattleCLI<R, W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        write!(self.stdout, "{}{}{}", ClearAll, style::Reset, Goto(1, 1)).unwrap();
    }
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> BattleUI for BattleCLI<R, W> {
    fn refresh(&mut self, battle: &Battle) {
        self.clear();
        self.draw_boxes(battle.baddies.len(), battle.allies.len(), battle);
        self.write_baddies_info(&battle.baddies);
        self.write_allies_info(&battle.allies);
        self.write_text(battle.get_text());
        self.write_menus(battle);
    }
    fn get_key(&mut self) -> Key {
        self.stdout.flush().unwrap();
        self.stdin.next().unwrap().unwrap()
    }
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> BattleCLI<R, W> {
    fn clear(&mut self) {
        write!(self.stdout, "{}{}", ClearAll, Goto(1, 1)).unwrap();
    }
    fn write_text(&mut self, text: Option<&String>) -> bool {
        if let Some(text) = text {
            write!(self.stdout, "{} >>> {}", Goto(1, 35), text).unwrap();
            return true;
        }
        false
    }
    fn write_menus(&mut self, battle: &Battle) {
        if battle.get_top_menu_options().is_some() {
            // fixme
            let (menus, selections) = battle.get_menu_selections();
            if selections.is_empty() {
                return;
            }
            let depth = if battle.targets.is_empty() {
                selections.len()
            } else {
                selections.len() + 1
            };
            for (im, (m, s)) in menus.iter().zip(selections.iter()).enumerate() {
                let menu_width = m.iter().max_by_key(|&s| s.len()).unwrap().len() + 11;
                write!(
                    self.stdout,
                    "{}{}{}",
                    Goto((1 + im * 8) as u16, 25),
                    color::Fg(color::AnsiValue::grayscale(
                        23u8.saturating_sub(5 * (depth - im - 1) as u8)
                    )),
                    "#".repeat(menu_width)
                )
                .unwrap();
                for (i, opt) in m.iter().enumerate() {
                    let opt_str = if i == *s { " -> " } else { "    " };
                    let opt_str = format!("# {}{}. {}", opt_str, i + 1, opt);
                    write!(
                        self.stdout,
                        "{}{}",
                        Goto((1 + im * 8) as u16, (26 + i) as u16),
                        opt_str
                    )
                    .unwrap();
                }
                write!(
                    self.stdout,
                    "{}{}{}",
                    Goto((1 + im * 8) as u16, (26 + m.len()) as u16),
                    "#".repeat(menu_width),
                    color::Fg(color::Reset)
                )
                .unwrap();
            }
            write!(self.stdout, "{} >>> Pick your next action! ", Goto(1, 35)).unwrap();
        }
    }
    fn _draw_boxes(&mut self, n: usize, baddies: bool, battle: &Battle) {
        // todo: for now this overwrites all character info
        for i in 0..n {
            let pi = if baddies {
                PlayerIndex::Baddy(i)
            } else {
                PlayerIndex::Ally(i)
            };
            let mut row = if battle.targets.contains(&pi) {
                String::from(TARGET_OUTER_ROW)
            } else if battle.is_pc_turn(&pi) {
                String::from(TURN_OUTER_ROW)
            } else {
                String::from(OUTER_ROW)
            };
            if battle.is_player_down(&pi) {
                row = format!(
                    "{}{}{}",
                    color::Fg(color::Red),
                    row,
                    color::Fg(color::Reset)
                );
            }
            write!(self.stdout, "{}", row).unwrap();
        }
        write!(self.stdout, "\r\n").unwrap();
        for _ in 2..=BOX_HEIGHT {
            for i in 0..n {
                let pi = if baddies {
                    PlayerIndex::Baddy(i)
                } else {
                    PlayerIndex::Ally(i)
                };
                let mut row = if battle.targets.contains(&pi) {
                    String::from(TARGET_INNER_ROW)
                } else if battle.is_pc_turn(&pi) {
                    String::from(TURN_INNER_ROW)
                } else {
                    String::from(INNER_ROW)
                };
                if battle.is_player_down(&pi) {
                    row = format!(
                        "{}{}{}",
                        color::Fg(color::Red),
                        row,
                        color::Fg(color::Reset)
                    );
                }
                write!(self.stdout, "{}", row).unwrap();
            }
            write!(self.stdout, "\r\n").unwrap();
        }
        for i in 0..n {
            let pi = if baddies {
                PlayerIndex::Baddy(i)
            } else {
                PlayerIndex::Ally(i)
            };
            let mut row = if battle.targets.contains(&pi) {
                String::from(TARGET_OUTER_ROW)
            } else if battle.is_pc_turn(&pi) {
                String::from(TURN_OUTER_ROW)
            } else {
                String::from(OUTER_ROW)
            };
            if battle.is_player_down(&pi) {
                row = format!(
                    "{}{}{}",
                    color::Fg(color::Red),
                    row,
                    color::Fg(color::Reset)
                );
            }
            write!(self.stdout, "{}", row).unwrap();
        }
        write!(self.stdout, "\r\n").unwrap();
    }
    fn draw_boxes(&mut self, num_baddies: usize, num_allies: usize, battle: &Battle) {
        //        self.clear();
        self._draw_boxes(num_baddies, true, battle);
        write!(self.stdout, "\n\n\n").unwrap();
        self._draw_boxes(num_allies, false, battle); // fixme
    }
    fn write_baddies_info(&mut self, p: &Party) {
        for i in 0..p.len() {
            let c = p.get_ch_by_pos(i).unwrap();
            let name = c.whoami().1;
            let i: u16 = i.try_into().unwrap();
            write!(self.stdout, "{} {}", Goto(i * BOX_WIDTH + 3, 2), name).unwrap();
            for (j, (_, pool)) in c.get_pools().iter().enumerate() {
                let j: u16 = j.try_into().unwrap();
                write!(
                    self.stdout,
                    "{} {:>4}: {:4} / {:4}",
                    Goto(i * BOX_WIDTH + 3, 4 + j),
                    pool.name,
                    pool.current,
                    pool.maximum
                )
                .unwrap();
            }
            //write!(self.stdout, "{} {}", Goto(i*BOX_WIDTH + 3, 7), p.clocks.get(i as usize).unwrap()).unwrap();
        }
    }
    fn write_allies_info(&mut self, p: &Party) {
        for i in 0..p.len() {
            // todo: ch_iter
            let c = p.get_ch_by_pos(i).unwrap();
            let name = c.whoami().1;
            let i: u16 = i.try_into().unwrap();
            write!(
                self.stdout,
                "{} {}",
                Goto(i * BOX_WIDTH + 3, BOX_HEIGHT + 4 + 2),
                name
            )
            .unwrap();
            for (j, (_, pool)) in c.get_pools().iter().enumerate() {
                let j: u16 = j.try_into().unwrap();
                write!(
                    self.stdout,
                    "{} {:>4}: {:4} / {:4}",
                    Goto(i * BOX_WIDTH + 3, BOX_HEIGHT + 4 + 4 + j),
                    pool.name,
                    pool.current,
                    pool.maximum
                )
                .unwrap();
            }
            //write!(self.stdout, "{} {}", Goto(i*BOX_WIDTH + 3, BOX_HEIGHT+4+7), p.clocks.get(i as usize).unwrap()).unwrap();
        }
    }
}
