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
            .insert_resource(LevelSelection::Identifier("Level_1".into()))
            .insert_resource(LdtkSettings { level_background: LevelBackground::Nonexistent, ..default()})
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
    commands.insert_resource(LevelSelection::Identifier("Level_1".into()));
}

fn build_walls(mut commands: Commands, cells: Query<(Entity, &IntGridCell), Added<IntGridCell>>) {
    for (entity, cell) in cells.iter() {
        if cell.value == 2 {
            commands.entity(entity)
                .insert(RigidBody::Static)
                .insert(CollisionShape::Cuboid { half_extends: Vec3::new(36., 36., 0.), border_radius: None })
                .insert(CollisionLayers::all_masks::<GameCollisionLayers>().with_group(GameCollisionLayers::World));
        }
    }
}