use amethyst::{
    assets::{Handle, HotReloadStrategy, Prefab},
    core::math::Vector3,
    core::transform::Transform,
    core::Named,
    ecs::Join,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, SpriteRender},
    window::ScreenDimensions,
};

use super::PauseState;
use crate::components::Bomberman;
use crate::MyPrefabData;
use log::info;

/// Padding between screen edge and the grid
const PADDING: f32 = 64.0;

pub struct GameplayState {
    pub scene_handle: Handle<Prefab<MyPrefabData>>,
    pub tile_handle: Handle<Prefab<MyPrefabData>>,
    pub bomberman_sprite: SpriteRender,
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
        init_camera(world, &dimensions);

        world
            .create_entity()
            .with(self.scene_handle.clone())
            .build();

        self.create_grid(world);
        self.create_bomberman(world);
    }

    fn create_grid(&self, world: &mut World) {
        let tile_half_extent = 32.0;
        let margin = 1.0; // margin around tiles
        let width = 18;
        let height = 15;

        for i in 0..width {
            for j in 0..height {
                let x = PADDING + tile_half_extent + 2.0 * (margin + tile_half_extent) * i as f32;
                let y = PADDING + tile_half_extent + 2.0 * (margin + tile_half_extent) * j as f32;
                let mut transform = Transform::default();
                transform.set_translation_xyz(x, y, 0.0);
                transform.set_scale(Vector3::new(tile_half_extent, tile_half_extent, 1.0));

                world
                    .create_entity()
                    .with(self.tile_handle.clone())
                    .with(transform)
                    .named("tile")
                    .build();
            }
        }
    }

    fn create_bomberman(&self, world: &mut World) {
        let x = PADDING + 32.0;
        let y = PADDING + 64.0;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.1);

        world
            .create_entity()
            .with(Bomberman {})
            .with(self.bomberman_sprite.clone())
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
