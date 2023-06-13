use specs::prelude::*;

use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Kinematics>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;
		// TODO: parallel join
        for (anim, sprite, k) in (&mut data.0, &mut data.1, &data.2).join() {
			// TODO: skip if not moving
            let frames = match k.orientation {
                Left => &anim.left_frames,
                Right => &anim.right_frames,
                Up => &anim.up_frames,
                Down => &anim.down_frames,
            };

            anim.current_frame = (anim.current_frame + 1) % frames.len();
            *sprite = frames[anim.current_frame].clone();
        }
    }
}
