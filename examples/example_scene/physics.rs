use specs::prelude::*;

use crate::components::*;

pub struct Physics;

impl<'a> System<'a> for Physics {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Kinematics>);

    fn run(&mut self, mut data: Self::SystemData) {
		// TODO: parallel join
		for (pos, k) in (&mut data.0, &mut data.1).join() {
			let diffx = k.mark.0.x - pos.0.x;
			let diffy = k.mark.0.y - pos.0.y;
			let vx = k.max_speed * diffx.signum();
			let vy = k.max_speed * diffy.signum();
			if vx.abs() >= diffx.abs() {
				if k.velocity.0.x != 0 {
					pos.0.x += vx;
					k.mark.0.x += 16*k.velocity.0.x.signum();
				} else {
					pos.0.x = k.mark.0.x;
				}
			} else {
				pos.0.x += vx;
			}
			if vy.abs() >= diffy.abs() {
				if k.velocity.0.y != 0 {
					pos.0.y += vy;
					k.mark.0.y += 16*k.velocity.0.y.signum();
				} else {
					pos.0.y = k.mark.0.y;
				}
			} else {
				pos.0.y += vy;
			}
		}
	}
}
