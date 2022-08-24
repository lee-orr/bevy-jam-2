use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance, LdtkEntity};
use bevy_kira_audio::{Audio, AudioSource};
use heron::{prelude::*, rapier_plugin::PhysicsWorld};

use crate::{
    audio::AudioEmitter,
    loading_state::LoadedAssets,
    physics::GameCollisionLayers,
    player::PlayerControl,
    spirit_collection::{Collected, Collecting},
    states::States,
};

pub struct SpiritPlugin;

impl Plugin for SpiritPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AwaitingEmitters>()
            .add_system_set(
                SystemSet::on_update(States::InGame)
                    .with_system(spirit_avoid_player)
                    .with_system(spirit_surrounder)
                    .with_system(determine_sightline),
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

    if ready {
        if let Ok(_) = app_state.set(States::InGame) {
            bevy::log::debug!("Starting Level");
            awaiting_emitters.emitters = vec![];
            awaiting_emitters.is_set = false;
            return;
        }
    }

    bevy::log::debug!("Waiting - not ready yet");
}

fn spawn_spirit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut awaiting_emitters: ResMut<AwaitingEmitters>,
    assets: Res<LoadedAssets>,
    asset_server: Res<AssetServer>,
    entities: Query<(&EntityInstance, &Transform), Added<EntityInstance>>,
) {
    let mut emitters: Vec<Handle<AudioSource>> = vec![];
    let mut found_entites = false;
    for (instance, transform) in entities.iter() {
        found_entites = true;
        let spawning = match instance.identifier.as_str() {
            "StationarySpirit" => {Some(commands.spawn().id())},
            "RandomWalkSpirit" => {
                Some(commands.spawn().insert(SpiritAvoidPlayer).id())
            }
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
                        .insert(SpiritSurrounder(angle * PI / 180., distance))
                        .id(),
                )
            }
            _ => None,
        };
        if let Some(entity) = spawning {
            let mut spawning = commands.entity(entity);
            let (max_speed, audio, color) = {
                let mut max_speed = 9.5f32;
                let mut audio = None;
                let mut color = Color::WHITE;

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
                        _ => {}
                    }
                }

                (max_speed, audio, color)
            };

            spawning
                .insert_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle::default()))
                        .into(),
                    material: materials.add(ColorMaterial::from(color)),
                    transform: transform.with_scale(Vec3::ONE * 6.),
                    ..default()
                })
                .insert(Spirit(max_speed))
                .insert(RigidBody::Dynamic)
                .insert(CollisionShape::Sphere { radius: 6. })
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
        }
    }

    if (found_entites) {
        awaiting_emitters.emitters = emitters;
        awaiting_emitters.is_set = true;
    }
}

fn determine_sightline(
    mut commands: Commands,
    spirits: Query<
        (Entity, &Transform),
        (With<Spirit>, Without<Collecting>, Without<Collected>),
    >,
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
            Without<Collecting>,
            Without<PlayerControl>,
            With<CanSeePlayer>,
        ),
    >,
    players: Query<&Transform, With<PlayerControl>>,
    time: Res<Time>,
    window: Res<Windows>,
) {
    let target = players.get_single();
    let speed = 95f32;
    let delta = time.delta().as_secs_f32();
    let bounds = if let Some(window) = window.get_primary() {
        (window.width() / 2., window.height() / 2.)
    } else {
        (500., 500.)
    };

    let player_position = if let Ok(player) = target {
        player.translation
    } else {
        Vec3::ZERO
    };

    for (mut transform, spirit, mut velocity_component) in spirits.iter_mut() {
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
            Without<Collecting>,
            Without<PlayerControl>,
            With<CanSeePlayer>,
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
    let bounds = if let Some(window) = window.get_primary() {
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
