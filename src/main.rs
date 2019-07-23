use amethyst::{
    assets::{HotReloadBundle, HotReloadStrategy, PrefabLoaderSystem, Processor},
    core::transform::TransformBundle,
    prelude::*,
    renderer::{
        rendy::mesh::{Normal, Position, TexCoord},
        sprite_visibility::SpriteVisibilitySortingSystem,
        types::DefaultBackend,
        RenderingSystem, SpriteSheet,
    },
    utils::{application_root_dir, scene::BasicScenePrefab},
    window::WindowBundle,
};

mod render;
mod states;

use states::LoadingState;

pub type MyPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let assets = app_root.join("assets");
    let display_config = assets.join("display_config.ron");

    let render_graph = render::RenderGraph::default();
    let render_system = RenderingSystem::<DefaultBackend, _>::new(render_graph);

    let game_data = GameDataBuilder::default()
        .with_bundle(WindowBundle::from_config_path(display_config))?
        .with_bundle(HotReloadBundle::new(HotReloadStrategy::when_triggered()))?
        .with(PrefabLoaderSystem::<MyPrefabData>::default(), "", &[])
        .with_bundle(TransformBundle::new())?
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