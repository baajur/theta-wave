#![cfg_attr(feature = "fail-on-warnings", deny(warnings))]

extern crate amethyst;
extern crate serde;

use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
    gltf::GltfSceneLoaderSystemDesc,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderDebugLines, RenderFlat2D, RenderFlat3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

pub mod audio;
pub mod components;
pub mod constants;
pub mod entities;
pub mod events;
pub mod resources;
pub mod states;
pub mod systems;

use resources::{
    ConsumablesResource, DebugLinesConfig, EnemiesResource, ItemsResource,
    JuggernautPlayerResource, SoundsConfig, SpriteSheetsConfig, ThrustersResource,
};
use states::MainGameState;

use amethyst::config::Config;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;
    let config_path = app_root.join("config");
    let data_path = app_root.join("assets").join("data");
    let players_path = data_path.join("players");

    let display_config_path = config_path.join("display_config_960.ron");
    let bindings_path = config_path.join("bindings_config.ron");

    let debug_lines =
        <DebugLinesConfig as Config>::load(config_path.join("debug_lines_config.ron"))
            .expect("failed to load configuration file: debug_lines_config.ron");
    let spritesheets =
        <SpriteSheetsConfig as Config>::load(config_path.join("spritesheets_config.ron"))
            .expect("failed to load configuration file: spritesheets_config.ron");
    let sounds = <SoundsConfig as Config>::load(config_path.join("sounds_config.ron"))
        .expect("failed to load configuration file: sounds_config.ron");
    let items = <ItemsResource as Config>::load(data_path.join("items.ron"))
        .expect("failed to load data file: items.ron");
    let enemies = <EnemiesResource as Config>::load(data_path.join("enemies.ron"))
        .expect("failed to load data file: enemies.ron");
    let thrusters = <ThrustersResource as Config>::load(data_path.join("thrusters.ron"))
        .expect("failed to load data file: thrusters.ron");
    let consumables = <ConsumablesResource as Config>::load(data_path.join("consumables.ron"))
        .expect("failed to load data file: consumables.ron");
    let players = <JuggernautPlayerResource as Config>::load(players_path.join("juggernaut.ron"))
        .expect("failed to load data file: players.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(GltfSceneLoaderSystemDesc::default(), "gltf_system", &[])
        .with_bundle(TransformBundle::new())?
        .with_bundle(InputBundle::<StringBindings>::new().with_bindings_from_file(bindings_path)?)?
        .with_bundle(AudioBundle::default())?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config_path)?
                        .with_clear([0.0, 0.0, 0.0, 1.0]),
                )
                .with_plugin(RenderFlat3D::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderUi::default())
                .with_plugin(RenderDebugLines::default()),
        )?;

    let mut game = Application::build(app_root.join("assets"), MainGameState::default())?
        .with_resource(items)
        .with_resource(enemies)
        .with_resource(thrusters)
        .with_resource(consumables)
        .with_resource(players)
        .with_resource(spritesheets)
        .with_resource(sounds)
        .with_resource(debug_lines)
        .build(game_data)?;

    game.run();

    Ok(())
}
