use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Kinematics>
    );

    fn run(&mut self, mut data: Self::SystemData) {
        // TODO: parallel join
        for (pos, k) in (&mut data.0, &mut data.1).join() {
            // TODO: collision detection
            pos.location.x += k.velocity.x;
            pos.location.y += k.velocity.y;
        }
    }
}
