use termion::event::Key;

use crate::scene::Scene;

pub trait SceneUI {
    fn refresh(&mut self, scene: &Scene);
    fn get_key(&mut self) -> Key;
}
