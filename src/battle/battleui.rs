use termion::event::Key;

use crate::battle::Battle;

pub trait BattleUI {
    fn refresh(&mut self, battle: &Battle);
    fn get_key(&mut self) -> Key;
}
