use crate::interactive_narrative::SetCurrentKnotEvent;
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
pub enum Portal {
    Level(String),
    Knot(String)
}

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
        bevy::log::info!("Deactivated entity");
        commands.entity(entity).remove::<DeactivateElement>();
        commands.entity(entity).remove::<ActiveElement>();
    }
}

fn set_activation(mut commands: Commands, elements: Query<(Entity, &NamedElement)>, mut event_reader: EventReader<ActivationEvent>) {
    for event in event_reader.iter() {
        bevy::log::info!("Activation: {} {}", &event.0, &event.1);
        for (entity, name) in elements.iter() {
            bevy::log::info!("Entity name {}", &name.0);
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
            let mut target_knot =  None;
            let mut has_target = false;
            let mut solid = true;

            for field in instance.field_instances.iter() {
                match field.identifier.as_str() {
                    "TargetLevel" => {
                        if let FieldValue::String(Some(level)) = &field.value {
                            has_target = true;
                            target_level = Some(level.clone());
                        }
                    }
                    "TargetKnot" => {
                        if let FieldValue::String(Some(knot)) = &field.value {
                            has_target = true;
                            target_knot = Some(knot.clone());
                        }
                    }
                    "EntityId" => {
                        bevy::log::info!("Checking for portal name {:?}", &field.value);
                        if let FieldValue::String(Some(name)) = &field.value {
                            id = Some(name.clone());
                        }
                    }
                    "StartEnabled" => {
                        if let FieldValue::Bool(start_enabled) = &field.value {
                            active = *start_enabled;
                        }
                    }
                    "Solid" => {
                        if let FieldValue::Bool(is_solid) = &field.value {
                            solid = *is_solid;
                        }
                    }
                    _ => {}
                }
            }

            if has_target || solid {
                let mut entity_commands = commands.entity(entity);

                entity_commands
                    .insert(LevelElement)
                    .insert(if solid { RigidBody::Static } else { RigidBody::Sensor })
                    .insert(CollisionShape::Cuboid {
                        half_extends: Vec3::new((instance.width as f32)/2., (instance.height as f32)/2., 0.),
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
                    bevy::log::info!("Created named portal  {}", &id);
                    entity_commands.insert(NamedElement(id));
                } else {
                    bevy::log::info!("Created un-named portal");
                }

                if let Some(level) = target_level {
                    entity_commands.insert(Portal::Level(level));
                } else if let Some(knot) = target_knot {
                    entity_commands.insert(Portal::Knot(knot));
                }
            }
        }
    }
}

fn trigger_portal(
    mut collisions: EventReader<CollisionEvent>,
    portals: Query<&Portal, With<ActiveElement>>,
    mut set_level: EventWriter<SetLevelEvent>,
    mut set_knot: EventWriter<SetCurrentKnotEvent>,
) {
    for event in collisions.iter().filter(|e| e.is_started()) {
        let (entity_1, entity_2) = event.rigid_body_entities();
        let (layers_1, layers_2) = event.collision_layers();
        if layers_1.contains_group(GameCollisionLayers::Player)
            && layers_2.contains_group(GameCollisionLayers::Portal)
        {
            if let Ok(portal) = portals.get(entity_2) {
                match portal {
                    Portal::Level(level) => set_level.send(SetLevelEvent(level.clone())),
                    Portal::Knot(knot) => set_knot.send(SetCurrentKnotEvent(Some(knot.clone()))),
                }
                break;
            }
        }
        if layers_2.contains_group(GameCollisionLayers::Player)
            && layers_1.contains_group(GameCollisionLayers::Portal)
        {
            if let Ok(portal) = portals.get(entity_1) {
                match portal {
                    Portal::Level(level) => set_level.send(SetLevelEvent(level.clone())),
                    Portal::Knot(knot) => set_knot.send(SetCurrentKnotEvent(Some(knot.clone()))),
                }
                break;
            }
        }
    }
}
