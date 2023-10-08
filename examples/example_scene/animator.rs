use specs::prelude::*;

use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Kinematics>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        use self::Direction::*;
        // TODO: parallel join
        for (anim, sprite, pos, k) in (&mut data.0, &mut data.1, &data.2, &data.3).join() {
            let frames = match pos.orientation {
                Left => &anim.left_frames,
                Right => &anim.right_frames,
                Up => &anim.up_frames,
                Down => &anim.down_frames,
            };
            if k.velocity.x == 0 && k.velocity.y == 0 {
                anim.current_frame = anim.neutral_frame;
                anim.frames_since_update = 0;
                *sprite = frames[anim.current_frame].clone();
                continue;
            }
            anim.frames_since_update += 1;
            if anim.frames_since_update >= anim.frame_period {
                anim.current_frame += 1;
                anim.current_frame %= frames.len();
                anim.frames_since_update = 0;
            }
            *sprite = frames[anim.current_frame].clone();
        }
    }
}
