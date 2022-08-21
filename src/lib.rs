mod audio;
mod level;
mod loading_state;
mod menu;
mod player;
mod spirit;
mod spirit_collection;
mod states;

use audio::*;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use level::*;
use loading_state::*;
use menu::*;
use player::*;
use spirit::*;
use spirit_collection::*;
use states::States;

pub const LAUNCHER_TITLE: &str = "Memory Mixer";

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .add_state(States::Loading)
    .add_plugins(DefaultPlugins)
    .add_plugin(LevelPlugin)
    .add_plugin(LoadingPlugin)
    .add_plugin(MenuPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(SpiritPlugin)
    .add_plugin(AudioPlayerPlugin)
    .add_plugin(SpiritCollection)
    .add_plugin(WorldInspectorPlugin::new())
    .add_startup_system(load_camera);
    app
}

fn load_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
