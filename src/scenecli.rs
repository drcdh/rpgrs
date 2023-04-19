use termion::clear::All as ClearAll;
use termion::color;
use termion::cursor::Goto;
use termion::event::Key;
use termion::style;
use std::io::Write;

use crate::common::*;
use crate::map::Map;
use crate::scene::Scene;
use crate::scene::sceneui::SceneUI;

/*
const OUTER_ROW: &str = r" =================================== ";
const INNER_ROW: &str = r" |                                 | ";
const TURN_OUTER_ROW: &str = r" @=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=~=@ ";
const TURN_INNER_ROW: &str = r" !                                 ! ";
const TARGET_OUTER_ROW: &str = r" |\/\/\/\/\/\/\/\/|\/\/\/\/\/\/\/\/| ";
const TARGET_INNER_ROW: &str = r" >                                 < ";
const BOX_HEIGHT: u16 = 8;
const BOX_WIDTH: u16 = 37;
*/

pub struct SceneCLI<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> {
    pub stdin: R,
    pub stdout: W,
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> Drop for
SceneCLI<R, W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        write!(self.stdout, "{}{}{}", ClearAll, style::Reset, Goto(1, 1)).unwrap();
    }
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write> SceneUI for
SceneCLI<R, W> {
    fn refresh(&mut self, scene: &Scene) {
        self.clear();
        self.write_map(&scene.map, &scene.center);
        //self.write_objects(scene.objects);
        self.write_text(scene.get_text());
        //self.write_huds(scene);
    }
    fn get_key(&mut self) -> Key {
        self.stdout.flush().unwrap();
        self.stdin.next().unwrap().unwrap()
    }
}

impl<R: Iterator<Item=Result<Key, std::io::Error>>, W: Write>
SceneCLI<R, W> {
    fn clear(&mut self) {
        write!(self.stdout, "{}{}", ClearAll, Goto(1, 1)).unwrap();
    }
    fn write_map(&mut self, map: &Map, center: &XY) {
        write!(self.stdout, "{}{}", Goto(1, 1), map.render(center, 0, 0)).unwrap();
    }
    fn write_text(&mut self, text: Option<&String>) -> bool {
        if let Some(text) = text {
            write!(self.stdout, "{} >>> {}", Goto(1, 35), text).unwrap();
        }
        false
    }
}
