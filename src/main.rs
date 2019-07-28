use amethyst::{
    animation::{AnimationSetPrefab, AnimationBundle},
    assets::{
        HotReloadBundle, HotReloadStrategy, PrefabData, PrefabLoaderSystem, Processor,
        ProgressCounter,
    },
    core::transform::TransformBundle,
    derive::PrefabData,
    ecs::Entity,
    error::Error,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        sprite::{prefab::SpriteScenePrefab, SpriteRender},
        sprite_visibility::SpriteVisibilitySortingSystem,
        types::DefaultBackend,
        RenderingSystem, SpriteSheet,
    },
    utils::application_root_dir,
    window::WindowBundle,
};

use serde::{Deserialize, Serialize};

mod components;
mod render;
mod states;
mod systems;

use states::LoadingState;

/// Animation ids used in a AnimationSet
#[derive(Eq, PartialOrd, PartialEq, Hash, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum AnimationId {
    Walk,
}

#[derive(Debug, Clone, Deserialize, PrefabData)]
pub struct BombermanPrefabData {
    pub sprite_scene: SpriteScenePrefab,
    pub animation_set: AnimationSetPrefab<AnimationId, SpriteRender>,
}

fn main() -> amethyst::Result<()> {
    amethyst::Logger::from_config(Default::default())
        .level_for("gfx_backend_vulkan", log::LevelFilter::Error)
        .start();

    let app_root = application_root_dir()?;

    let assets = app_root.join("assets");
    let display_config = assets.join("display_config.ron");

    let render_graph = render::RenderGraph::default();
    let render_system = RenderingSystem::<DefaultBackend, _>::new(render_graph);

    let input_bundle =
        InputBundle::<StringBindings>::new().with_bindings_from_file(assets.join("input.ron"))?;

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config))?
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::when_triggered()))?
        .with(
            PrefabLoaderSystem::<BombermanPrefabData>::default(),
            "",
            &[],
        )
        .with_bundle(TransformBundle::new())?
    .with_bundle(AnimationBundle::<AnimationId, SpriteRender>::new("sprite_animation_control", "sprite_animation_sampling"))?
        .with_bundle(input_bundle)?
        .with(
            systems::BombermanSystem,
            "bomberman_system",
            &["input_system"],
        )
        .with(
            SpriteVisibilitySortingSystem::new(),
            "sprite_visibility_system",
            &["transform_system"],
        )
        .with(
            Processor::<SpriteSheet>::new(),
            "sprite_sheet_processor",
            &[],
        )
        .with_thread_local(render_system);

    let mut game = Application::new(assets, LoadingState::default(), game_data)?;
    game.run();

    Ok(())
}
