use crate::loading_state::LoadedAssets;
use crate::physics::GameCollisionLayers;
use crate::states::States;
use bevy::prelude::*;
use bevy_ecs_ldtk::ldtk::TileInstance;
use bevy_ecs_ldtk::prelude::*;

use heron::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .insert_resource(LevelSelection::Identifier("Level_0".into()))
            .add_system_set(
                SystemSet::on_enter(States::LoadingLevel)
                    .with_system(start_level),
            )
            .add_system_set(
                SystemSet::on_update(States::LoadingLevel)
                    .with_system(build_walls),
            );
    }
}

fn start_level(mut commands: Commands, assets: Res<LoadedAssets>) {
    bevy::log::info!("Loading level");
    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: assets.test_level.clone(),
        ..Default::default()
    });
    commands.insert_resource(LevelSelection::Identifier("Level_0".into()));
}

fn build_walls(mut commands: Commands, cells: Query<(Entity, &IntGridCell), Added<IntGridCell>>) {
    bevy::log::info!("Building walls");
    for (entity, cell) in cells.iter() {
        bevy::log::info!("Checking cell");
        if cell.value == 2 {
            bevy::log::info!("Build wall!");
            commands.entity(entity)
                .insert(RigidBody::Static)
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(32., 32., 0.), border_radius: None })
                .insert(CollisionLayers::all_masks::<GameCollisionLayers>().with_group(GameCollisionLayers::World));
        }
    }
}