mod audio;
mod camera;
mod level;
mod loading_state;
mod menu;
mod player;
mod spirit;
mod spirit_collection;
mod states;
mod physics;
mod ink;
mod interactive_narrative;
pub mod theme;

use audio::*;
use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_inspector_egui::{WorldInspectorPlugin, plugin::InspectorWindows};
use camera::*;
use heron::PhysicsPlugin;
use ink::InkPlugin;
use level::*;
use loading_state::*;
use menu::*;
use player::*;
use spirit::*;
use spirit_collection::*;
use interactive_narrative::*;
use states::States;
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
        .add_plugins(DefaultPlugins)
        .add_plugin(LevelPlugin)
        .add_plugin(LoadingPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(SpiritPlugin)
        .add_plugin(AudioPlayerPlugin)
        .add_plugin(SpiritCollection)
        .add_plugin(CameraPlugin)
        .add_plugin(PhysicsPlugin::default())
        // .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(InkPlugin)
        .add_plugin(InteractiveNarrativePlugin);
    app
}