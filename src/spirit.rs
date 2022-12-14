use std::f32::consts::PI;

use bevy::{
    prelude::*,
};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};
use bevy_kira_audio::{AudioSource};
use heron::{prelude::*, rapier_plugin::PhysicsWorld};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    audio::AudioEmitter,
    interactive_narrative::SetCurrentKnotEvent,
    level::{ActiveElement, DeactivateElement, LevelElement, NamedElement},
    loading_state::LoadedAssets,
    physics::GameCollisionLayers,
    player::{Action, PlayerControl},
    states::{GameMode, States},
};

pub struct SpiritPlugin;

impl Plugin for SpiritPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AwaitingEmitters>()
            .add_system_set(
                SystemSet::on_update(States::InGame)
                    .with_system(spirit_avoid_player)
                    .with_system(spirit_surrounder)
                    .with_system(determine_sightline)
                    .with_system(animate_spirits),
            )
            .add_system_to_stage(CoreStage::PostUpdate, deactivate_elements)
            .add_system_set(
                SystemSet::on_update(GameMode::Exploration)
                    .with_system(trigger_knot),
            )
            .add_system_set(
                SystemSet::on_update(States::LoadingLevel)
                    .with_system(spawn_spirit)
                    .with_system(spirits_ready),
            );
    }
}

#[derive(Component)]
pub struct Spirit(f32);

#[derive(Component)]
pub struct SpiritAvoidPlayer;

#[derive(Component)]
pub struct SpiritSurrounder(f32, f32);

#[derive(Component)]
pub struct TargetKnot(String);

#[derive(Component)]
#[component(storage = "SparseSet")]
struct CanSeePlayer;

pub struct AwaitingEmitters {
    pub emitters: Vec<Handle<AudioSource>>,
    pub is_set: bool,
}

impl Default for AwaitingEmitters {
    fn default() -> Self {
        Self {
            emitters: vec![],
            is_set: false,
        }
    }
}

pub struct CharacterAtlas {
    pub atlas: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct SpiritAnimationIndices {
    pub start: usize,
    pub len: usize,
}

fn spirits_ready(
    mut awaiting_emitters: ResMut<AwaitingEmitters>,
    mut app_state: ResMut<State<States>>,
    asset_server: Res<AssetServer>,
) {
    if !awaiting_emitters.is_set {
        bevy::log::debug!("Waiting - not set yet");
        return;
    }
    let mut ready = true;
    for emitter in awaiting_emitters.emitters.iter() {
        let state = asset_server.get_load_state(emitter.clone());
        match state {
            bevy::asset::LoadState::Loading => {
                ready = false;
                break;
            }
            _ => {}
        }
    }

    if ready && app_state.set(States::InGame).is_ok() {
        bevy::log::debug!("Starting Level");
        awaiting_emitters.emitters = vec![];
        awaiting_emitters.is_set = false;
        return;
    }

    bevy::log::debug!("Waiting - not ready yet");
}

fn spawn_spirit(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    mut awaiting_emitters: ResMut<AwaitingEmitters>,
    assets: Res<LoadedAssets>,
    asset_server: Res<AssetServer>,
    texture_atlas: Option<Res<CharacterAtlas>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    entities: Query<(&EntityInstance, &Transform), Added<EntityInstance>>,
) {
    let atlas_handle = match texture_atlas {
        Some(atlas) => atlas.atlas.clone(),
        None => {
            let atlas = TextureAtlas::from_grid(
                assets.character_atlas.clone(),
                Vec2::ONE * 64.,
                16,
                16,
            );
            let handle = texture_atlases.add(atlas);
            commands.insert_resource(CharacterAtlas {
                atlas: handle.clone(),
            });
            handle
        }
    };

    let mut emitters: Vec<Handle<AudioSource>> = vec![];
    let mut found_entites = false;
    for (instance, transform) in entities.iter() {
        found_entites = true;
        let spawning = match instance.identifier.as_str() {
            "StationarySpirit" => {
                Some(commands.spawn().insert(RigidBody::Sensor).id())
            }
            "RandomWalkSpirit" => Some(
                commands
                    .spawn()
                    .insert(RigidBody::Dynamic)
                    .insert(SpiritAvoidPlayer)
                    .id(),
            ),
            "CirclingSpirit" => {
                let mut angle = 10f32;
                let mut distance = 120f32;

                for field in instance.field_instances.iter() {
                    match field.identifier.as_str() {
                        "AngularSpeed" => {
                            if let FieldValue::Float(Some(speed)) = field.value
                            {
                                angle = speed;
                            }
                        }
                        "TargetDistance" => {
                            if let FieldValue::Float(Some(d)) = field.value {
                                distance = d;
                            }
                        }
                        _ => {}
                    }
                }

                Some(
                    commands
                        .spawn()
                        .insert(RigidBody::Dynamic)
                        .insert(SpiritSurrounder(angle * PI / 180., distance))
                        .id(),
                )
            }
            _ => None,
        };
        if let Some(entity) = spawning {
            let mut spawning = commands.entity(entity);
            let (
                max_speed,
                audio,
                _color,
                knot,
                animation_start,
                animation_end,
                active,
                id,
            ) = {
                let mut max_speed = 9.5f32;
                let mut audio = None;
                let mut color = Color::WHITE;
                let mut knot = None;
                let mut animation_start = 0usize;
                let mut animation_end = 0usize;
                let mut active = false;
                let mut id = None;

                for field in instance.field_instances.iter() {
                    match field.identifier.as_str() {
                        "MaxSpeed" => {
                            if let FieldValue::Float(Some(speed)) = field.value
                            {
                                max_speed = speed;
                            }
                        }
                        "Audio" => {
                            if let FieldValue::String(Some(audio_file)) =
                                &field.value
                            {
                                let handle: Handle<AudioSource> =
                                    asset_server.load(&audio_file.clone());

                                emitters.push(handle.clone());

                                audio = Some((handle, audio_file.clone()));
                            }
                        }
                        "Color" => {
                            if let FieldValue::Color(c) = field.value {
                                color = c;
                            }
                        }
                        "TriggerKnot" => {
                            if let FieldValue::String(Some(knot_name)) =
                                &field.value
                            {
                                knot = Some(knot_name.clone());
                            }
                        }
                        "AnimationStart" => {
                            if let FieldValue::Int(Some(frame)) = field.value {
                                animation_start = frame as usize;
                            }
                        }
                        "AnimationEnd" => {
                            if let FieldValue::Int(Some(frame)) = field.value {
                                animation_end = frame as usize;
                            }
                        }
                        "EntityId" => {
                            if let FieldValue::String(Some(level)) =
                                &field.value
                            {
                                id = Some(level.clone());
                            }
                        }
                        "StartEnabled" => {
                            if let FieldValue::Bool(start_enabled) =
                                &field.value
                            {
                                active = *start_enabled;
                            }
                        }
                        _ => {}
                    }
                }

                (
                    max_speed,
                    audio,
                    color,
                    knot,
                    animation_start,
                    animation_end,
                    active,
                    id,
                )
            };

            spawning
                .insert_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: animation_start,
                        ..default()
                    },
                    texture_atlas: atlas_handle.clone(),
                    transform: transform.with_scale(Vec3::ONE * 0.5),
                    visibility: Visibility { is_visible: false },
                    ..default()
                })
                .insert(LevelElement)
                .insert(SpiritAnimationIndices {
                    len: animation_end
                        .checked_sub(animation_start)
                        .unwrap_or(1)
                        + 1,
                    start: animation_start,
                })
                .insert(Spirit(max_speed))
                .insert(CollisionShape::Sphere { radius: 16. })
                .insert(PhysicMaterial {
                    restitution: 0.9,
                    friction: 0.1,
                    density: 10.0,
                    ..Default::default()
                })
                .insert(Velocity::from_linear(Vec3::ZERO))
                .insert(
                    CollisionLayers::all_masks::<GameCollisionLayers>()
                        .with_group(GameCollisionLayers::Spirit),
                );

            if let Some((audio, file)) = audio {
                spawning.insert(AudioEmitter(audio, file));
            }
            if let Some(knot) = knot {
                spawning.insert(TargetKnot(knot.clone()));
            }

            if active {
                spawning.insert(ActiveElement);
            }
            if let Some(id) = id {
                spawning.insert(NamedElement(id));
            }
        }
    }

    if found_entites {
        awaiting_emitters.emitters = emitters;
        awaiting_emitters.is_set = true;
    }
}

fn determine_sightline(
    mut commands: Commands,
    spirits: Query<(Entity, &Transform), (With<Spirit>, With<ActiveElement>)>,
    players: Query<(Entity, &Transform), With<PlayerControl>>,
    physics_world: PhysicsWorld,
) {
    bevy::log::debug!("Looking for player");
    let target = players.get_single();
    if let Ok((player, player_transform)) = target {
        let target = player_transform.translation;
        bevy::log::debug!("Got player position {:?}", &target);

        for (entity, transform) in spirits.iter() {
            bevy::log::debug!("Checking from {:?}", &transform.translation);
            let result = physics_world.ray_cast_with_filter(
                transform.translation,
                target - transform.translation,
                true,
                CollisionLayers::all_groups::<GameCollisionLayers>()
                    .with_masks([
                        GameCollisionLayers::Player,
                        GameCollisionLayers::World,
                    ]),
                |_entity| true,
            );
            if let Some(collision_info) = result {
                if collision_info.entity == player {
                    bevy::log::debug!("Player found");
                    commands.entity(entity).insert(CanSeePlayer);
                } else {
                    bevy::log::debug!("Player not found");
                    commands.entity(entity).remove::<CanSeePlayer>();
                }
            } else {
                bevy::log::debug!("No collision");
            }
        }
    }
}

fn spirit_avoid_player(
    mut spirits: Query<
        (&Transform, &Spirit, &mut Velocity),
        (
            With<SpiritAvoidPlayer>,
            Without<PlayerControl>,
            With<CanSeePlayer>,
            With<ActiveElement>,
        ),
    >,
    players: Query<&Transform, With<PlayerControl>>,
    time: Res<Time>,
    window: Res<Windows>,
) {
    let target = players.get_single();
    let speed = 95f32;
    let delta = time.delta().as_secs_f32();
    let _bounds = if let Some(window) = window.get_primary() {
        (window.width() / 2., window.height() / 2.)
    } else {
        (500., 500.)
    };

    let player_position = if let Ok(player) = target {
        player.translation
    } else {
        Vec3::ZERO
    };

    for (transform, spirit, mut velocity_component) in spirits.iter_mut() {
        let direction = transform.translation - player_position;
        if direction.length() > 300. {
            return;
        }
        let direction = direction.normalize();

        let mut velocity =
            velocity_component.linear + direction * delta * speed;

        if velocity.length() > spirit.0 {
            velocity = velocity.normalize() * (spirit.0 - 1.);
        }

        velocity_component.linear = velocity;
    }
}

fn spirit_surrounder(
    mut spirits: Query<
        (&Transform, &Spirit, &SpiritSurrounder, &mut Velocity),
        (
            Without<PlayerControl>,
            With<CanSeePlayer>,
            With<ActiveElement>,
        ),
    >,
    players: Query<&Transform, With<PlayerControl>>,
    time: Res<Time>,
    window: Res<Windows>,
) {
    let target = players.get_single();
    let speed = 95f32;
    let delta = time.delta().as_secs_f32();
    let elapsed = time.time_since_startup().as_secs_f32();
    let _bounds = if let Some(window) = window.get_primary() {
        (window.width() / 2., window.height() / 2.)
    } else {
        (500., 500.)
    };

    let player_position = if let Ok(player) = target {
        player.translation
    } else {
        Vec3::ZERO
    };

    for (
        transform,
        spirit,
        SpiritSurrounder(angle, target_distance),
        mut velocity_component,
    ) in spirits.iter_mut()
    {
        let target_position = player_position
            + *target_distance
                * (Quat::from_rotation_z(angle * elapsed).mul_vec3(Vec3::Y));

        let direction = target_position - transform.translation;
        let direction = direction.normalize();

        let mut velocity =
            velocity_component.linear + direction * delta * speed;

        if velocity.length() > spirit.0 {
            velocity = velocity.normalize() * (spirit.0 - 1.);
        }

        velocity_component.linear = velocity;
    }
}

fn trigger_knot(
    mut spirits: Query<
        (&Transform, &TargetKnot, &mut Visibility),
        (
            With<Spirit>,
            Without<PlayerControl>,
            With<CanSeePlayer>,
            With<ActiveElement>,
        ),
    >,
    players: Query<(&Transform, &ActionState<Action>), With<PlayerControl>>,
    mut event_writer: EventWriter<SetCurrentKnotEvent>,
) {
    let mut target_knot = None;
    for (player, action) in players.iter() {
        if action.pressed(Action::Interact) {
            for (spirit, knot, mut visibility) in spirits.iter_mut() {
                if (player.translation - spirit.translation).length() < 100. {
                    target_knot = Some(knot.0.clone());
                    visibility.is_visible = true;
                }
            }
        }
    }

    if let Some(target_knot) = target_knot {
        event_writer.send(SetCurrentKnotEvent(Some(target_knot)));
    }
}

fn animate_spirits(
    mut spirits: Query<
        (&mut TextureAtlasSprite, &SpiritAnimationIndices),
        (With<Spirit>, With<ActiveElement>),
    >,
    time: Res<Time>,
) {
    let time = (time.seconds_since_startup() * 5.) as usize;
    for (mut sprite, animation) in spirits.iter_mut() {
        let current_index = animation.start + (time % animation.len);
        sprite.index = current_index;
    }
}

fn deactivate_elements(
    mut spirits: Query<
        &mut Visibility,
        (With<Spirit>, With<DeactivateElement>),
    >,
) {
    for mut visibility in spirits.iter_mut() {
        visibility.is_visible = false;
    }
}
