use bevy::{prelude::*, render::texture::ImageSettings};
use bevy_ecs_ldtk::prelude::*;
use crate::states::States;
use crate::loading_state::LoadedAssets;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(ImageSettings::default_nearest())
            .add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Identifier("Level_0".into()))
            .add_system_set(
                SystemSet::on_enter(States::InGame).with_system(start_level),
            );
    }
}

fn start_level(mut commands: Commands, assets: Res<LoadedAssets>) {
    bevy::log::info!("Loading level");
    commands.spawn_bundle(LdtkWorldBundle{
        ldtk_handle: assets.test_level.clone(),
        ..Default::default()
    });
    commands
    .insert_resource(LevelSelection::Identifier("Level_0".into()));
}

// fn current_tiles(tiles: Query<&TilePos>) {
//     bevy::log::info!("We have {} tiles!", tiles.iter().len());
// }