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
        for (script, p, k) in (&data.0, &data.1, &mut data.2).join() {
            (script.script_fn)(p, k);
        }
    }
}

pub fn test(p: &Position, k: &mut Kinematics) {
    if p.location.x >= 50 {
        k.velocity.x = 0;
        k.velocity.y = 4;
    } else {
        k.velocity.x = 4;
    }
}
