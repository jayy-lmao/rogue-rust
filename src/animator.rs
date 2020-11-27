use specs::prelude::*;

use crate::components::*;

pub struct Animator;

impl<'a> System<'a> for Animator {
    type SystemData = (
        WriteStorage<'a, MovementAnimation>,
        WriteStorage<'a, Sprite>,
        ReadStorage<'a, Velocity>,
    );

    fn run(&mut self, mut data: Self::SystemData) {
        // You can see the data is what we grab from system data
        for (anim, sprite, vel) in (&mut data.0, &mut data.1, &data.2).join() {
            if vel.speed == 0 {
                continue;
            }
            let frames = match vel.direction {
                Direction::Left => &anim.left_frames,
                Direction::Right => &anim.right_frames,
                Direction::Up => &anim.up_frames,
                Direction::Down => &anim.down_frames,
            };

            anim.current_frame = (anim.current_frame + 1) % frames.len();

            *sprite = frames[anim.current_frame].clone();
        }
    }
}
