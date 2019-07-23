use amethyst::{
    assets::{Handle, Prefab, PrefabLoader, ProgressCounter, RonFormat},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
};

use super::GameplayState;
use crate::MyPrefabData;

#[derive(Default)]
pub struct LoadingState {
    progress_counter: ProgressCounter,
    scene_handle: Option<Handle<Prefab<MyPrefabData>>>,
    tile_handle: Option<Handle<Prefab<MyPrefabData>>>,
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

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        let progress_counter = &mut self.progress_counter;
        self.scene_handle = Some(load_prefab("prefabs/scene.ron", world, progress_counter));
        self.tile_handle = Some(load_prefab("prefabs/tile.ron", world, progress_counter));
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
                scene_handle: self.scene_handle.clone().unwrap(),
                tile_handle: self.tile_handle.clone().unwrap(),
            }));
        }
        Trans::None
    }
}
