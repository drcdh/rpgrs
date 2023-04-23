use std::fmt::Write as StringWrite;
use std::io::Write;
use termion::clear::All as ClearAll;
use termion::color;
use termion::cursor::Goto;
use termion::event::Key;
use termion::style;

use crate::common::*;
use crate::map::Map;
use crate::scene::sceneui::SceneUI;
use crate::scene::Scene;
use crate::sprite::Sprite;

pub struct SceneCLI<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> {
    pub stdin: R,
    pub stdout: W,
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> Drop for SceneCLI<R, W> {
    fn drop(&mut self) {
        // When done, restore the defaults to avoid messing with the terminal.
        write!(self.stdout, "{}{}{}", ClearAll, style::Reset, Goto(1, 1)).unwrap();
    }
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> SceneUI for SceneCLI<R, W> {
    fn refresh(&mut self, scene: &Scene) {
        self.clear();
        //self.write_map(&scene.map, &scene.center);
        //self.write_objects(scene.objects);
        self.render(&scene.map, &scene.center);
        self.write_text(scene.get_text());
        //self.write_huds(scene);
    }
    fn get_key(&mut self) -> Key {
        self.stdout.flush().unwrap();
        self.stdin.next().unwrap().unwrap()
    }
}

impl<R: Iterator<Item = Result<Key, std::io::Error>>, W: Write> SceneCLI<R, W> {
    fn clear(&mut self) {
        write!(self.stdout, "{}{}", ClearAll, Goto(1, 1)).unwrap();
    }
    fn write_text(&mut self, text: Option<&String>) -> bool {
        if let Some(text) = text {
            write!(self.stdout, "{} >>> {}", Goto(1, 35), text).unwrap();
        }
        false
    }
    fn render(&mut self, map: &Map, center: &XY) {
        for (j, row) in map.encoded_map.iter().enumerate() {
            for (i, code) in row.iter().enumerate() {
                let sprite = map.decode_sprite(*code).unwrap();
                let x: Coord = <usize as TryInto<Coord>>::try_into(i).unwrap() + 1;
                let y: Coord = <usize as TryInto<Coord>>::try_into(j).unwrap() + 1;
                write!(self.stdout, "{}{}", Goto(x, y), sprite.draw(0)).unwrap();
            }
        }
    }
}
