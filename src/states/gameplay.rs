use amethyst::{
    assets::{Handle, HotReloadStrategy, Prefab},
    core::transform::Transform,
    core::Named,
    ecs::Join,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, SpriteRender},
    window::ScreenDimensions,
};

use super::PauseState;
use crate::components::{Block, Bomberman};
use crate::BombermanPrefabData;
use log::info;

/// Padding between screen edge and the grid
const PADDING: f32 = 64.0;
const BLOCK_SIZE: f32 = 32.0; // block's half-extent

pub struct GameplayState {
    pub block_sprites: Vec<SpriteRender>,
    pub bomberman_handle: Handle<Prefab<BombermanPrefabData>>,
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        self.reload_scene(world);
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            if is_key_down(&event, VirtualKeyCode::R) {
                info!("HotReload triggered");
                data.world.write_resource::<HotReloadStrategy>().trigger();
                self.reload_scene(data.world);
            }

            if is_key_down(&event, VirtualKeyCode::P) {
                return Trans::Push(Box::new(PauseState));
            }
        }
        Trans::None
    }

    fn fixed_update(&mut self, data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if true {
            return Trans::None;
        }
        let names = data.world.read_storage::<Named>();
        let mut transforms = data.world.write_storage::<Transform>();
        for (name, transform) in (&names, &mut transforms).join() {
            match &name.name[..] {
                "tile" => {
                    transform.move_left(1.0);
                }
                _ => {}
            }
        }
        Trans::None
    }
}

impl GameplayState {
    fn reload_scene(&self, world: &mut World) {
        world.delete_all();

        let dimensions = world.read_resource::<ScreenDimensions>().clone();

        self.create_grid(world);
        self.create_bomberman(world);

        init_camera(world, &dimensions);
    }

    fn create_grid(&self, world: &mut World) {
        let margin = 1.0; // margin around tiles
        let width = 18;
        let height = 15;

        for i in 0..width {
            for j in 0..height {
                let x = PADDING + BLOCK_SIZE + 2.0 * (margin + BLOCK_SIZE) * i as f32;
                let y = PADDING + BLOCK_SIZE + 2.0 * (margin + BLOCK_SIZE) * j as f32;
                let mut transform = Transform::default();
                transform.set_translation_xyz(x, y, 0.0);

                let (block, sprite) = if i == 0 || j == 0 || i == width - 1 || j == height - 1 {
                    // edges of the level are always solid
                    (Block::Solid, self.block_sprites[2].clone())
                } else {
                    match rand::random::<u8>() {
                        0...200 => (Block::Background, self.block_sprites[0].clone()),
                        200...220 => (Block::Explodable, self.block_sprites[1].clone()),
                        220...240 => (Block::Solid, self.block_sprites[2].clone()),
                        _ => (Block::Portal, self.block_sprites[3].clone()),
                    }
                };

                world
                    .create_entity()
                    .with(block)
                    .with(sprite)
                    .with(transform)
                    .named("block")
                    .build();
            }
        }
    }

    fn create_bomberman(&self, world: &mut World) {
        let x = PADDING + 2.0 * BLOCK_SIZE + 32.0;
        let y = PADDING + 2.0 * BLOCK_SIZE + 32.0;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.1);

        world
            .create_entity()
            .with(Bomberman::default())
            .with(self.bomberman_handle.clone())
            .with(transform)
            .named("bomberman")
            .build();
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}
