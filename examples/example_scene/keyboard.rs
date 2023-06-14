use specs::prelude::*;

use crate::components::*;

use super::MovementCommand;


pub struct Keyboard;

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

        // Loop over keyboard-controlled objects
        for (_, p, k) in (&data.1, &mut data.2, &mut data.3).join() {
            match movement_command {
                &MovementCommand::Move(Left) => {
                    k.velocity.x += -k.max_speed;
                    p.orientation = Left;
                },
                &MovementCommand::Move(Right) => {
                    k.velocity.x += k.max_speed;
                    p.orientation = Right;
                },
                &MovementCommand::Move(Up) => {
                    k.velocity.y += -k.max_speed;
                    p.orientation = Up;
                },
                &MovementCommand::Move(Down) => {
                    k.velocity.y += k.max_speed;
                    p.orientation = Down;
                },
                &MovementCommand::Stop(Left) => {
                    k.velocity.x -= -k.max_speed;
                },
                &MovementCommand::Stop(Right) => {
                    k.velocity.x -= k.max_speed;
                },
                &MovementCommand::Stop(Up) => {
                    k.velocity.y -= -k.max_speed;
                },
                &MovementCommand::Stop(Down) => {
                    k.velocity.y -= k.max_speed;
                },
            }
        }
    }
}
