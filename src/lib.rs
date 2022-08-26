mod audio;
mod camera;
mod ink;
mod interactive_narrative;
mod level;
mod loading_state;
mod menu;
mod physics;
mod player;
mod spirit;
mod states;
pub mod theme;

use audio::*;
use bevy::{prelude::*, render::texture::ImageSettings};

use camera::*;
use heron::PhysicsPlugin;
use ink::InkPlugin;
use interactive_narrative::*;
use level::*;
use loading_state::*;
use menu::*;
use player::*;
use spirit::*;
use states::{GameMode, States};
use theme::*;

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(ImageSettings::default_nearest())
        .insert_resource(ClearColor(BACKGROUIND_COLOR))
        .insert_resource(WindowDescriptor {
            title: LAUNCHER_TITLE.to_string(),
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        })
        .add_state(States::Loading)
        .add_state(GameMode::None)
        .add_plugins(DefaultPlugins)
        .add_plugin(LevelPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(SpiritPlugin)
        .add_plugin(AudioPlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PhysicsPlugin::default())
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InkPlugin)
        .add_plugin(InteractiveNarrativePlugin);
    app
}
