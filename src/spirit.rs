use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{
    audio::AudioEmitter, loading_state::LoadedAssets, player::PlayerControl,
    spirit_collection::Collecting, states::States,
};

pub struct SpiritPlugin;

impl Plugin for SpiritPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(States::InGame).with_system(spawn_spirit),
        )
        .add_system_set(
            SystemSet::on_update(States::InGame)
                .with_system(spirit_random_walk)
                .with_system(spirit_surrounder),
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
) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(100., 0., 0.))
                .with_scale(Vec3::new(30., 30., 30.)),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..default()
        })
        .insert(Spirit(Vec3::ZERO, 120.))
        .insert(SpiritRandomWalker)
        .insert(AudioEmitter(assets.bass_1.clone(), "Bass".to_owned()));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(100., 50., 0.))
                .with_scale(Vec3::new(30., 30., 30.)),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..default()
        })
        .insert(SpiritRandomWalker)
        .insert(Spirit(Vec3::ZERO, 100.));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(100., -50., 0.))
                .with_scale(Vec3::new(30., 30., 30.)),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..default()
        })
        .insert(Spirit(Vec3::ZERO, 140.));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(70., 30., 0.))
                .with_scale(Vec3::new(30., 30., 30.)),
            material: materials.add(ColorMaterial::from(Color::BLUE)),
            ..default()
        })
        .insert(Spirit(Vec3::ZERO, 95.))
        .insert(SpiritSurrounder(10. * PI / 180., 250.));
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
