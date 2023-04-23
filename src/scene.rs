use std::collections::VecDeque;
use termion::event::Key;

use crate::character::Character;
use crate::common::*;
use crate::map::Map;
use crate::party::Party;
use crate::sprite::Sprite;

pub mod sceneui;
use sceneui::SceneUI;
/*
struct PlacedSprite<Fr> {
    id: Id,
    sprite: IndexedOrLiteral<Sprite<Fr>>,
    x: Coord,
    y: Coord,
    z: Coord,
    //effects: VisualEffects,
    //script: SpriteScript,
}
*/
//type PlacedSprites = Vec::<PlacedSprite>;
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
    /// focus is the Map Coord that is aligned with the SceneUI.focus.
    /// Typically, it will follow the lead party member.
    pub focus: XY,
    pub map: Map,
    pub actors: Vec<Sprite>,
    text: VecDeque<String>,
    ended: bool,
    pub ticker: u8,
}

impl Scene {
    pub fn new(map: Map, focus: XY) -> Scene {
        let mut text = VecDeque::<String>::new();
        let party_leader = Sprite::new_solid('@');
        let actors = vec![party_leader];
        text.push_back("So, here you are.".to_string());
        Scene {
            focus, // this will be a scene's entrance location
            map,
            actors,
            text,
            ended: false,
            ticker: 0,
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
                //break;
            }
            self.ticker += 1;
        }
    }
    fn handle_input(&mut self, key: Key) -> bool {
        if !self.text.is_empty() {
            self.pop_text();
            return false;
        }
        self.party_command(key);
        self.ended
    }
    fn party_command(&mut self, key: Key) {
        if key == Key::Up {
            self.focus.1 -= 1;
        } else if key == Key::Left {
            self.focus.0 -= 1;
        } else if key == Key::Right {
            self.focus.0 += 1;
        } else if key == Key::Down {
            self.focus.1 += 1;
        }
    }
    pub fn get_text(&self) -> Option<&String> {
        self.text.front()
    }
    fn pop_text(&mut self) -> Option<String> {
        self.text.pop_front()
    }
}
