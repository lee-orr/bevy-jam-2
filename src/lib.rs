mod audio;
mod loading_state;
mod menu;
mod player;
mod spirit;
mod spirit_collection;
mod states;

use audio::*;
use bevy::prelude::*;
use loading_state::*;
use menu::*;
use player::*;
use spirit::*;
use spirit_collection::*;
use states::States;

pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

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
    .add_plugin(LoadingPlugin)
    .add_plugin(MenuPlugin)
    .add_plugin(PlayerPlugin)
    .add_plugin(SpiritPlugin)
    .add_plugin(AudioPlayerPlugin)
    .add_plugin(SpiritCollection)
    .add_startup_system(load_camera);
    app
}

fn load_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
