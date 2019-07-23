use amethyst::{
    assets::{
        Handle, HotReloadStrategy, Prefab, AssetStorage, Loader,
    },
    window::ScreenDimensions,
    core::math::Vector3,
    ecs::Join,
    core::Named,
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, Texture, SpriteSheetFormat, SpriteSheet},
};

use log::info;
use crate::MyPrefabData;
use super::PauseState;

pub struct GameplayState {
    pub scene_handle: Handle<Prefab<MyPrefabData>>,
    pub tile_handle: Handle<Prefab<MyPrefabData>>,
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
            return Trans::None
        }
        let names = data.world.read_storage::<Named>();
        let mut transforms = data.world.write_storage::<Transform>();
        for (name, transform) in (&names, &mut transforms).join() {
            match &name.name[..] {
                "tile_left" => {
                    transform.move_left(1.0);
                },
                "tile_right" => {
                    transform.move_right(1.0);
                },
                _ => {},
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

        if false {
            // Load our sprites and display them
            let sprites = load_sprites(world);
            init_sprites(world, &sprites, &dimensions);
        }

        world
            .create_entity()
            .with(self.scene_handle.clone())
            .build();

        let padding = 64.0; // tiles container padding
        let tile_half_extent = 32.0;
        let margin = 1.0; // margin around tiles
        let size = 15; // width & height

        for i in 0..size {
            for j in 0..size {
                let x =  padding + tile_half_extent + 2.0 * (margin + tile_half_extent) * i as f32;
                let y =  padding + tile_half_extent + 2.0 * (margin + tile_half_extent) * j as f32;
                let mut transform = Transform::default();
                transform.set_translation_xyz(x, y, 0.0);
                transform.set_scale(Vector3::new(tile_half_extent, tile_half_extent, 1.0));

                world
                    .create_entity()
                    .with(self.tile_handle.clone())
                    .with(transform)
                    .named(if i % 2 == 0 {"tile_left"} else { "tile_right"})
                    .build();
            }
        }
    }
}

fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_scale(Vector3::new(1.0, -1.0, 1.0));
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/logo.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/logo.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    (0..3)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

fn init_sprites(world: &mut World, sprites: &[SpriteRender], _dimensions: &ScreenDimensions) {
    for (i, sprite) in sprites.iter().enumerate() {
        // Center our sprites around the center of the window
        // let x = (i as f32 - 1.) * 115. + dimensions.width() * 0.5;
        // let y = (i as f32 - 1.) * 115. + dimensions.height() * 0.5;
        let x = (i as f32 + 1.) * 115.;
        let y = (i as f32 + 1.) * 115.;
        let mut transform = Transform::default();
        // transform.set_translation_xyz(x, y, 0.);
        transform.set_translation_xyz(0., 0., 0.);

        // Create an entity for each sprite and attach the `SpriteRender` as
        // well as the transform. If you want to add behaviour to your sprites,
        // you'll want to add a custom `Component` that will identify them, and a
        // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
        world
            .create_entity()
            .with(sprite.clone())
            .with(transform)
            .build();
    }
}
