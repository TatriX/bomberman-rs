use amethyst::{
    assets::{AssetStorage, Handle, Loader, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use super::GameplayState;
use crate::MyPrefabData;

#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
    scene_handle: Option<Handle<Prefab<MyPrefabData>>>,
    bomberman_sprite: Option<SpriteRender>,
    blocks_sprites: Option<Vec<SpriteRender>>,
}

fn load_prefab(
    path: &str,
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<Prefab<MyPrefabData>> {
    world.exec(|loader: PrefabLoader<'_, MyPrefabData>| {
        loader.load(path, RonFormat, progress_counter)
    })
}

fn load_sprite_sheet(
    path: &str,
    texture_handle: Handle<Texture>,
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<SpriteSheet> {
    let loader = world.read_resource::<Loader>();
    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        path,
        SpriteSheetFormat(texture_handle),
        progress_counter,
        &sheet_storage,
    )
}

fn load_texture(
    path: &str,
    world: &mut World,
    progress_counter: &mut ProgressCounter,
) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(
        path,
        ImageFormat::default(),
        progress_counter,
        &texture_storage,
    )
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        let progress_counter = &mut self.progress_counter;

        self.scene_handle = Some(load_prefab("prefabs/scene.ron", world, progress_counter));

        self.bomberman_sprite = Some(SpriteRender {
            sprite_sheet: load_sprite_sheet(
                "sprites/bomberman.ron",
                load_texture(
                    "sprites/Bomberman/Front/Bman_F_f00.png",
                    world,
                    progress_counter,
                ),
                world,
                progress_counter,
            ),
            sprite_number: 0,
        });

        let blocks_sprite_sheet = load_sprite_sheet(
            "sprites/blocks.ron",
            load_texture("sprites/blocks.png", world, progress_counter),
            world,
            progress_counter,
        );

        self.blocks_sprites = Some(
            (0..4)
                .map(|i| SpriteRender {
                    sprite_sheet: blocks_sprite_sheet.clone(),
                    sprite_number: i,
                })
                .collect(),
        );
    }

    fn handle_event(
        &mut self,
        _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        // Keep going
        Trans::None
    }

    fn update(&mut self, _data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        if self.progress_counter.is_complete() {
            return Trans::Switch(Box::new(GameplayState {
                scene_handle: self.scene_handle.take().unwrap(),
                block_sprites: self.blocks_sprites.take().unwrap(),
                bomberman_sprite: self.bomberman_sprite.take().unwrap(),
            }));
        }
        Trans::None
    }
}
