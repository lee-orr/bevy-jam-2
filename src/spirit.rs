use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

pub struct SpiritPlugin;

impl Plugin for SpiritPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_spirit);
    }
}

fn spawn_spirit(mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {
    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
        transform: Transform::default().with_translation(Vec3::new(100.,0.,0.)).with_scale(Vec3::new(30., 30., 30.)),
        material: materials.add(ColorMaterial::from(Color::BLUE)),
        ..default()
    });
}