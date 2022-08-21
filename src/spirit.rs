use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::{audio::AudioEmitter, loading_state::LoadedAssets, states::States};

pub struct SpiritPlugin;

impl Plugin for SpiritPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(States::InGame).with_system(spawn_spirit),
        );
    }
}

#[derive(Component)]
pub struct Spirit;

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
        .insert(Spirit)
        .insert(AudioEmitter(assets.bass_1.clone(), "Bass".to_owned()));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(-100., 0., 0.))
                .with_scale(Vec3::new(30., 30., 30.)),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            ..default()
        })
        .insert(Spirit)
        .insert(AudioEmitter(assets.piano.clone(), "Piano".to_owned()));
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: Transform::default()
                .with_translation(Vec3::new(0., 100., 0.))
                .with_scale(Vec3::new(30., 30., 30.)),
            material: materials.add(ColorMaterial::from(Color::ORANGE)),
            ..default()
        })
        .insert(Spirit)
        .insert(AudioEmitter(assets.drums_2.clone(), "Drums".to_owned()));
}
