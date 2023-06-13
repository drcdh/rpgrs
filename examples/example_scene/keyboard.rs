use specs::prelude::*;

use crate::components::*;

use super::MovementCommand;


pub struct Keyboard;

impl<'a> System<'a> for Keyboard {
    type SystemData = (
        ReadExpect<'a, Option<MovementCommand>>,
        ReadStorage<'a, KeyboardControlled>,
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
        for (_, k) in (&data.1, &mut data.2).join() {
            match movement_command {
                &MovementCommand::Move(Left) => {
					if k.velocity.0.x == 0 {
						k.velocity.0.x = -k.max_speed;
						k.orientation = Left;
					}
                },
                &MovementCommand::Move(Right) => {
					if k.velocity.0.x == 0 {
						k.velocity.0.x = k.max_speed;
						k.orientation = Right;
					}
                },
                &MovementCommand::Move(Up) => {
					if k.velocity.0.y == 0 {
						k.velocity.0.y = -k.max_speed;
						k.orientation = Up;
					}
                },
                &MovementCommand::Move(Down) => {
					if k.velocity.0.y == 0 {
						k.velocity.0.y = k.max_speed;
						k.orientation = Down;
					}
                },
                &MovementCommand::Stop(Up) |
                &MovementCommand::Stop(Down) => {
					k.velocity.0.y = 0;
				},
                &MovementCommand::Stop(Left) |
                &MovementCommand::Stop(Right) => {
					k.velocity.0.x = 0;
				},
            }
        }
    }
}
