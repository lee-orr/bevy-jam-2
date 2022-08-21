use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ecs_ldtk::EntityInstance;
use leafwing_input_manager::prelude::*;

use crate::states::States;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .init_resource::<PlayerControlSettings>()
            .add_system_set(
                SystemSet::on_update(States::InGame)
                    .with_system(setup_player_control)
                    .with_system(move_player)
                    .with_system(spawn_player),
            );
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    MoveUp,
    MoveDown,
    RotateLeft,
    RotateRight,
    Collect,
}

#[derive(Component)]
pub struct PlayerControl;

pub struct PlayerControlSettings {
    move_speed: f32,
    rotate_speed: f32,
}

impl Default for PlayerControlSettings {
    fn default() -> Self {
        Self {
            move_speed: 100.,
            rotate_speed: 45. * PI / 180.,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    entities: Query<(&EntityInstance, &Transform), Added<EntityInstance>>,
) {
    for (instance, transform) in entities.iter() {
        if instance.identifier == "Player" {
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle::default()))
                        .into(),
                    transform: transform.with_scale(Vec3::new(50., 50., 50.)),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    ..default()
                })
                .insert(PlayerControl)
                .with_children(|parent| {
                    parent.spawn_bundle(MaterialMesh2dBundle {
                        mesh: meshes
                            .add(Mesh::from(shape::Circle::default()))
                            .into(),
                        transform: Transform::default()
                            .with_translation(Vec3::new(0., 0.5, 0.))
                            .with_scale(Vec3::new(0.2, 0.2, 0.2)),
                        material: materials
                            .add(ColorMaterial::from(Color::GREEN)),
                        ..default()
                    });
                });
        }
    }
}

fn setup_player_control(
    mut commands: Commands,
    query: Query<Entity, (With<PlayerControl>, Without<ActionState<Action>>)>,
) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert_bundle(InputManagerBundle::<Action> {
                // Stores "which actions are currently pressed"
                action_state: ActionState::default(),
                // Describes how to convert from player inputs into those
                // actions
                input_map: InputMap::new([
                    (KeyCode::Up, Action::MoveUp),
                    (KeyCode::Down, Action::MoveDown),
                    (KeyCode::Left, Action::RotateLeft),
                    (KeyCode::Right, Action::RotateRight),
                    (KeyCode::W, Action::MoveUp),
                    (KeyCode::S, Action::MoveDown),
                    (KeyCode::A, Action::RotateLeft),
                    (KeyCode::D, Action::RotateRight),
                    (KeyCode::Return, Action::Collect),
                    (KeyCode::Space, Action::Collect),
                ]),
            });
    }
}

fn move_player(
    mut query: Query<
        (&ActionState<Action>, &mut Transform),
        With<PlayerControl>,
    >,
    player_settings: Res<PlayerControlSettings>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    let speed = player_settings.move_speed;
    let rotation_speed = player_settings.rotate_speed;
    for (action, mut transform) in query.iter_mut() {
        let z_rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
        if action.pressed(Action::RotateRight) {
            let z_rotation = z_rotation - rotation_speed * delta;
            transform.rotation = Quat::from_rotation_z(z_rotation);
        } else if action.pressed(Action::RotateLeft) {
            let z_rotation = z_rotation + rotation_speed * delta;
            transform.rotation = Quat::from_rotation_z(z_rotation);
        }

        let direction_vector =
            transform
                .rotation
                .mul_vec3(if action.pressed(Action::MoveUp) {
                    Vec3::Y
                } else if action.pressed(Action::MoveDown) {
                    Vec3::NEG_Y
                } else {
                    Vec3::ZERO
                });

        transform.translation +=
            direction_vector.normalize_or_zero() * delta * speed;
    }
}
