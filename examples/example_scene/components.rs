use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::{Point, Rect};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct KeyboardControlled;

/// The current position of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub location: Point,
    pub z: i32,
    pub orientation: Direction,
}

/// The current motion of a given entity
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Kinematics {
    pub velocity: Point,
    pub max_speed: i32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Sprite {
    // TODO: replace this with an ID or unique string or whatever
    /// The index of the spritesheet to render from
    pub spritesheet: usize,
    /// The current region of the spritesheet to be rendered
    pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct MovementAnimation {
    /// Number of game frames per animation frame
    pub frame_period: usize,
    pub frames_since_update: usize,
    pub neutral_frame: usize,
    // The current frame in the animation of the direction this entity is moving in
    pub current_frame: usize,
    pub up_frames: Vec<Sprite>,
    pub down_frames: Vec<Sprite>,
    pub left_frames: Vec<Sprite>,
    pub right_frames: Vec<Sprite>,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct TileAnimation {
    pub frame_period: usize,
    pub frames_since_update: usize,
    pub current_frame: usize,
    pub frames: Vec<Sprite>,
}
