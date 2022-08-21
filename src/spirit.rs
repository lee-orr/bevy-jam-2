use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance, LdtkEntity};
use bevy_kira_audio::{Audio, AudioSource};

use crate::{
    audio::AudioEmitter, loading_state::LoadedAssets, player::PlayerControl,
    spirit_collection::Collecting, states::States,
};

pub struct SpiritPlugin;

impl Plugin for SpiritPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(States::InGame)
                .with_system(spirit_random_walk)
                .with_system(spirit_surrounder)
                .with_system(spawn_spirit),
        );
    }
}

#[derive(Component)]
pub struct Spirit(Vec3, f32);

#[derive(Component)]
pub struct SpiritRandomWalker;

#[derive(Component)]
pub struct SpiritSurrounder(f32, f32);

fn spawn_spirit(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<LoadedAssets>,
    asset_server: Res<AssetServer>,
    entities: Query<(&EntityInstance, &Transform), Added<EntityInstance>>,
) {
    for (instance, transform) in entities.iter() {
        let spawning = match instance.identifier.as_str() {
            "RandomWalkSpirit" => {
                Some(commands.spawn().insert(SpiritRandomWalker).id())
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
                    transform: transform.with_scale(Vec3::ONE * 3.),
                    ..default()
                })
                .insert(Spirit(Vec3::ZERO, max_speed));

            if let Some((audio, file)) = audio {
                spawning.insert(AudioEmitter(audio, file));
            }
        }
    }
}

fn spirit_random_walk(
    mut spirits: Query<
        (&mut Transform, &mut Spirit),
        (
            With<SpiritRandomWalker>,
            Without<Collecting>,
            Without<PlayerControl>,
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

    for (mut spirit, mut velocity) in spirits.iter_mut() {
        let direction = spirit.translation - player_position;
        let direction = direction.normalize();

        velocity.0 += direction * delta * speed;

        if velocity.0.length() > velocity.1 {
            velocity.0 = velocity.0.normalize() * (velocity.1 - 1.);
        }

        spirit.translation += velocity.0 * delta;

        if spirit.translation.x.abs() > bounds.0 {
            velocity.0.x = -1. * velocity.0.x;
        }
        if spirit.translation.y.abs() > bounds.1 {
            velocity.0.y = -1. * velocity.0.y;
        }
    }
}

fn spirit_surrounder(
    mut spirits: Query<
        (&mut Transform, &mut Spirit, &SpiritSurrounder),
        (Without<Collecting>, Without<PlayerControl>),
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

    for (mut spirit, mut velocity, SpiritSurrounder(angle, target_distance)) in
        spirits.iter_mut()
    {
        let target_position = player_position
            + *target_distance
                * (Quat::from_rotation_z(angle * elapsed).mul_vec3(Vec3::Y));

        let direction = target_position - spirit.translation;
        let direction = direction.normalize();

        velocity.0 += direction * delta * speed;

        if velocity.0.length() > velocity.1 {
            velocity.0 = velocity.0.normalize() * (velocity.1 - 1.);
        }

        spirit.translation += velocity.0 * delta;

        if spirit.translation.x.abs() > bounds.0 {
            velocity.0.x = -1. * velocity.0.x;
        }
        if spirit.translation.y.abs() > bounds.1 {
            velocity.0.y = -1. * velocity.0.y;
        }
    }
}
