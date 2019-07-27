use amethyst::core::{Float, Time, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::{Block, Bomberman};

pub struct BombermanSystem;

impl<'s> System<'s> for BombermanSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Bomberman>,
        ReadStorage<'s, Block>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut bomberman, blocks, input, time): Self::SystemData) {
        let speed = 300.0 * time.delta_seconds();

        // first calculate velocity and check for collisions
        for (bomberman, transform) in (&mut bomberman, &transforms).join() {
            let x = transform.translation().x.as_f32();
            let y = transform.translation().y.as_f32();

            let vx = input.axis_value("move_x").unwrap_or(0.0) as f32 * speed;
            let vy = input.axis_value("move_y").unwrap_or(0.0) as f32 * speed;

            let new_x = x + vx;
            let new_y = y + vy;

            // calculate collision box as (bomberman.width/2 + block.width/2)
            let half_width = (32 + 32) as f32;
            let half_height = (32 + 32) as f32;

            for (block, transform) in (&blocks, &transforms).join() {
                match block {
                    Block::Background => continue,
                    _ => {},
                }
                let block_x = transform.translation().x.as_f32();
                let block_y = transform.translation().y.as_f32();

                if point_in_rect(
                    new_x,
                    new_y,
                    block_x - half_width,
                    block_y - half_height,
                    block_x + half_width,
                    block_y + half_height,
                ) {
                    return;
                }
            }

            bomberman.velocity = [vx as f32, vy as f32];
        }

        // if no collision was detected adjust position according to velocity
        for (bomberman, transform) in (&bomberman, &mut transforms).join() {
            let x = transform.translation().x + Float::from(bomberman.velocity[0]);
            let y = transform.translation().y + Float::from(bomberman.velocity[1]);

            transform.set_translation_xyz(x, y, 0.0);
        }
    }
}

fn point_in_rect(x: f32, y: f32, left: f32, bottom: f32, right: f32, top: f32) -> bool {
    x > left && x < right && y > bottom && y < top
}
