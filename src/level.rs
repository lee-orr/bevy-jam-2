use crate::loading_state::LoadedAssets;
use crate::physics::GameCollisionLayers;
use crate::states::{GameMode, States};
use bevy::prelude::*;

use bevy_ecs_ldtk::prelude::*;

use heron::prelude::*;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LdtkPlugin)
            .add_event::<SetLevelEvent>()
            .add_event::<ActivationEvent>()
            .insert_resource(LevelSelection::Identifier("Level_0".into()))
            .insert_resource(LdtkSettings {
                level_background: LevelBackground::Nonexistent,
                ..default()
            })
            .add_system(set_level)
            .add_system_set(
                SystemSet::on_update(GameMode::Exploration)
                    .with_system(trigger_portal),
            )
            .add_system_set(
                SystemSet::on_enter(States::LoadingLevel)
                    .with_system(start_level),
            )
            .add_system_set(
                SystemSet::on_update(States::LoadingLevel)
                    .with_system(build_walls)
                    .with_system(build_portals),
            )
            .add_system_set(
                SystemSet::on_exit(States::InGame).with_system(exit_game),
            )
            .add_system(set_activation)
            .add_system_to_stage(CoreStage::Last, deactivate_elements)
            .add_system_to_stage(CoreStage::Last, clear_level_elements);
    }
}

pub struct SetLevelEvent(pub String);

fn set_level(
    mut commands: Commands,
    mut events: EventReader<SetLevelEvent>,
    mut app_state: ResMut<State<States>>,
    mut game_mode: ResMut<State<GameMode>>,
) {
    let event = events.iter().last();

    if let Some(SetLevelEvent(level)) = event {
        commands.insert_resource(LevelSelection::Identifier(level.into()));
        app_state.set(States::LoadingLevel);
        game_mode.set(GameMode::None);
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct LevelElement;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct ClearLevelElement;

#[derive(Component)]
pub struct ActiveElement;

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct DeactivateElement;

pub struct ActivationEvent(pub bool, pub String);

#[derive(Component)]
pub struct NamedElement(pub String);

#[derive(Component)]
pub struct Portal(String);

fn start_level(
    mut commands: Commands,
    assets: Res<LoadedAssets>,
    elements: Query<Entity, With<LevelElement>>,
) {
    for entity in elements.iter() {
        commands.entity(entity).insert(ClearLevelElement);
    }
    bevy::log::info!("Loading level");
    commands
        .spawn_bundle(LdtkWorldBundle {
            ldtk_handle: assets.test_level.clone(),
            ..Default::default()
        })
        .insert(LevelElement);
}

fn exit_game(
    mut commands: Commands,
    elements: Query<Entity, With<LevelElement>>,
) {
    for entity in elements.iter() {
        commands.entity(entity).insert(ClearLevelElement);
    }
}

fn clear_level_elements(
    mut commands: Commands,
    elements: Query<Entity, With<ClearLevelElement>>,
) {
    for entity in elements.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn deactivate_elements(
    mut commands: Commands,
    elements: Query<Entity, With<DeactivateElement>>,
) {
    for entity in elements.iter() {
        commands.entity(entity).remove::<DeactivateElement>();
        commands.entity(entity).remove::<ActiveElement>();
    }
}

fn set_activation(mut commands: Commands, elements: Query<(Entity, &NamedElement)>, mut event_reader: EventReader<ActivationEvent>) {
    for event in event_reader.iter() {
        bevy::log::info!("Activation: {} {}", &event.0, &event.1);
        for (entity, name) in elements.iter() {
            if name.0 == event.1 {
                bevy::log::info!("Found entity");
                if event.0 {
                    commands.entity(entity).insert(ActiveElement);
                } else {
                    commands.entity(entity).insert(DeactivateElement);
                }
                break;
            }
        }
    }
}

fn build_walls(
    mut commands: Commands,
    cells: Query<(Entity, &IntGridCell), Added<IntGridCell>>,
) {
    for (entity, cell) in cells.iter() {
        if cell.value == 2 {
            commands
                .entity(entity)
                .insert(RigidBody::Static)
                .insert(CollisionShape::Cuboid {
                    half_extends: Vec3::new(36., 36., 0.),
                    border_radius: None,
                })
                .insert(
                    CollisionLayers::all_masks::<GameCollisionLayers>()
                        .with_group(GameCollisionLayers::World),
                );
        }
    }
}

fn build_portals(
    mut commands: Commands,
    entities: Query<
        (Entity, &EntityInstance, &Transform),
        Added<EntityInstance>,
    >,
) {
    for (entity, instance, _transform) in entities.iter() {
        if instance.identifier == "Portal" {
            let mut target_level = None;
            let mut active = false;
            let mut id = None;

            for field in instance.field_instances.iter() {
                match field.identifier.as_str() {
                    "TargetLevel" => {
                        if let FieldValue::String(Some(level)) = &field.value {
                            target_level = Some(level.clone());
                        }
                    }
                    "EntityId" => {
                        if let FieldValue::String(Some(level)) = &field.value {
                            id = Some(level.clone());
                        }
                    }
                    "StartEnabled" => {
                        if let FieldValue::Bool(start_enabled) = &field.value {
                            active = *start_enabled;
                        }
                    }
                    _ => {}
                }
            }

            if let Some(level) = target_level {
                let mut entity_commands = commands.entity(entity);

                entity_commands
                    .insert(Portal(level))
                    .insert(LevelElement)
                    .insert(RigidBody::Static)
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::new(32., 32., 0.),
                        border_radius: None,
                    })
                    .insert(
                        CollisionLayers::all_masks::<GameCollisionLayers>()
                            .with_group(GameCollisionLayers::Portal),
                    );

                if active {
                    entity_commands.insert(ActiveElement);
                }
                if let Some(id) = id {
                    entity_commands.insert(NamedElement(id));
                }
            }
        }
    }
}

fn trigger_portal(
    mut collisions: EventReader<CollisionEvent>,
    portals: Query<&Portal, With<ActiveElement>>,
    mut set_level: EventWriter<SetLevelEvent>,
) {
    for event in collisions.iter().filter(|e| e.is_started()) {
        let (entity_1, entity_2) = event.rigid_body_entities();
        let (layers_1, layers_2) = event.collision_layers();
        if layers_1.contains_group(GameCollisionLayers::Player)
            && layers_2.contains_group(GameCollisionLayers::Portal)
        {
            if let Ok(portal) = portals.get(entity_2) {
                set_level.send(SetLevelEvent(portal.0.clone()));
                break;
            }
        }
        if layers_2.contains_group(GameCollisionLayers::Player)
            && layers_1.contains_group(GameCollisionLayers::Portal)
        {
            if let Ok(portal) = portals.get(entity_1) {
                set_level.send(SetLevelEvent(portal.0.clone()));
                break;
            }
        }
    }
}
