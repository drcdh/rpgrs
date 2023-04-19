use std::collections::VecDeque;
use termion::event::Key;

use crate::character::Character;
use crate::common::*;
use crate::map::Map;
use crate::party::Party;

pub mod sceneui;
use sceneui::SceneUI;

struct PlacedSprite {
    id: Id,
    sprite: IndexedOrLiteral<Sprite>,
    x: Coord,
    y: Coord,
    z: Coord,
    //effects: VisualEffects,
    //script: SpriteScript,
}

type PlacedSprites = Vec::<PlacedSprite>;
/*
struct State {
    sprites: PlacedSprites,
}

impl State {
    pub fn add_sprite(&self, sprite: &PlacedSprite) -> Id {
        self.sprites.push(sprite);
        (self.sprites.len()-1).try_into().unwrap()
        //sprite.id
    }
}
*/
pub struct Scene {
    pub center: XY,
    pub map: Map,
    text: VecDeque::<String>,
    ended: bool,
}

impl Scene {
    pub fn new(map: Map) -> Scene {
        let mut text = VecDeque::<String>::new();
        text.push_back("So, here you are.".to_string());
        Scene {
            center: (0i16, 0i16),
            map,
            text,
            ended: false,
        }
    }
    pub fn run(&mut self, ui: &mut dyn SceneUI) {
        loop {
            ui.refresh(self);
            let key = ui.get_key();
            if key == Key::Char('q') {
                break;
            }
            if let Key::Char(_c) = key {
                // Collect it as entropy
//                self.rand.write_u8(c as u8);
            }
            if self.handle_input(key) {
                break;
            }
        }
    }
    fn handle_input(&mut self, key: Key) -> bool {
        if !self.text.is_empty() {
            self.pop_text();
            return false;
        }
//        self.party_command(key);
        self.ended
    }
    pub fn get_text(&self) -> Option<&String> {
        self.text.front()
    }
    fn pop_text(&mut self) -> Option<String> {
        self.text.pop_front()
    }
}
