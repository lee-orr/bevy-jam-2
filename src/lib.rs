mod player;
mod spirit;
mod audio;

use bevy::prelude::*;
use player::*;
use spirit::*;
use audio::*;

pub const LAUNCHER_TITLE: &str = "Bevy Shell - Template";

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: LAUNCHER_TITLE.to_string(),
        canvas: Some("#bevy".to_string()),
        fit_canvas_to_parent: true,
        ..Default::default()
    })
    .add_plugins(DefaultPlugins)
    .add_plugin(PlayerPlugin)
    .add_plugin(SpiritPlugin)
    .add_plugin(AudioPlayerPlugin)
    .add_startup_system(load_camera);
    app
}

fn load_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
