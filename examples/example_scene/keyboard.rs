use specs::prelude::*;

use crate::components::*;

use super::MovementCommand;


pub struct Keyboard {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}
impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Kinematics>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        use crate::components::Direction::*;
        //TODO: This code can be made nicer and more idiomatic using more pattern matching.
        // Look up "rust irrefutable patterns" and use them here.
        let movement_command = match &*data.0 {
            Some(movement_command) => movement_command,
            None => return, // no change
        };

        match movement_command {
            &MovementCommand::Move(Left) => {
                self.left = true;
            },
            &MovementCommand::Move(Right) => {
                self.right = true;
            },
            &MovementCommand::Move(Up) => {
                self.up = true;
            },
            &MovementCommand::Move(Down) => {
                self.down = true;
            },
            &MovementCommand::Stop(Left) => {
                self.left = false;
            },
            &MovementCommand::Stop(Right) => {
                self.right = false;
            },
            &MovementCommand::Stop(Up) => {
                self.up = false;
            },
            &MovementCommand::Stop(Down) => {
                self.down = false;
            },
        }
        // Loop over keyboard-controlled objects
        for (_, p, k) in (&data.1, &mut data.2, &mut data.3).join() {
            k.velocity.x = k.max_speed * (self.right as i32 - self.left as i32);
            k.velocity.y = k.max_speed * (self.down as i32 - self.up as i32);
            if k.velocity.y > 0 {
                p.orientation = Down;
            } else if k.velocity.y < 0 {
                p.orientation = Up;
            }
            if k.velocity.x > 0 {
                p.orientation = Right;
            } else if k.velocity.x < 0 {
                p.orientation = Left;
            }
        }
    }
}
