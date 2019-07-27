use amethyst::core::{Float, Transform};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};

use crate::components::Bomberman;

pub struct BombermanSystem;

impl<'s> System<'s> for BombermanSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Bomberman>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
        let speed = 5.0;

        for (_, transform) in (&paddles, &mut transforms).join() {
            let move_x = input.axis_value("move_x");
            if let Some(mv_amount) = move_x {
                let scaled_amount = speed * mv_amount as f32;
                let x = transform.translation().x;
                transform.set_translation_x(x + Float::from(scaled_amount));
            }

            let move_y = input.axis_value("move_y");
            if let Some(mv_amount) = move_y {
                let scaled_amount = speed * mv_amount as f32;
                let y = transform.translation().y;
                transform.set_translation_y(y + Float::from(scaled_amount));
            }
        }
    }
}
