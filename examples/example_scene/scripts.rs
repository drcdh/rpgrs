use specs::prelude::*;

use crate::components::*;

pub struct MovementScripts;

impl<'a> System<'a> for MovementScripts {
    type SystemData = (
        ReadStorage<'a, Script>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Kinematics>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        for (script, p, k) in (&data.0, &mut data.1, &mut data.2).join() {
            (script.script_fn)(p, k);
        }
    }
}

pub fn test(p: &mut Position, k: &mut Kinematics) {
    use crate::components::Direction::*;
    if p.location.x <= 130 {
        k.velocity.x = 1;
        p.orientation = Right;
    } else if p.location.x >= 180 {
        k.velocity.x = -1;
        p.orientation = Left;
    }
}
