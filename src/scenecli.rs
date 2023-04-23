use std::convert::TryFrom;
//use std::fmt::Write as StringWrite;
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
    pub display_size: uXY,
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
        self.render(&scene.map, &scene.focus, scene.ticker);
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
            return true;
        }
        false
    }
    fn render(&mut self, map: &Map, focus: &XY, ticker: u8) {
        let beat = <u8 as TryInto<usize>>::try_into(ticker).unwrap();
        let t_ = (3u16, 3u16);
        let mut t = (0, 0);
        while t.1 <= self.display_size.1 {
            t.1 += 1;
            let y = (t.1 as Coord) + focus.1 - map.origin.1 - 1;
            t.0 = 0;
            while t.0 <= self.display_size.0 {
                t.0 += 1;
                let x = (t.0 as Coord) + focus.0 - map.origin.0 - 1;
                if x < 0 || x >= (map.dim.0 as Coord) ||
                   y < 0 || y >= (map.dim.1 as Coord) {
                    write!(self.stdout, "{}{}", Goto(t_.0 + t.0, t_.1 + t.1), "`").unwrap(); // Print void
                    continue;
                }
                let i = x as usize;
                let j = y as usize;
                let sprite = map.sprite_at_loc(i, j).unwrap();
                write!(self.stdout, "{}{}", Goto(t_.0 + t.0, t_.1 + t.1), sprite.draw(beat)).unwrap();
            }
//            write!(self.stdout, "\r\n").unwrap();
        }
        /*        for (j, row) in map.encoded_map.iter().enumerate() {
            for (i, code) in row.iter().enumerate() {
                let sprite = map.decode_sprite(*code).unwrap();
                let x: Coord = self.focus.0 - focus.0 + <usize as TryInto<Coord>>::try_into(i).unwrap() + 1;
                let y: Coord = self.focus.1 - focus.1 + <usize as TryInto<Coord>>::try_into(j).unwrap() + 1;
                write!(self.stdout, "{}{}", Goto(x, y), sprite.draw(0)).unwrap();
            }
        }*/
    }
}
