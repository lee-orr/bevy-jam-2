use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_ecs_ldtk::{EntityInstance, prelude::FieldValue};
use bevy_inspector_egui::{Inspectable, RegisterInspectable};
use leafwing_input_manager::prelude::*;
use heron::prelude::*;

use crate::states::States;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_system_set(
                SystemSet::on_update(States::InGame)
                    .with_system(setup_player_control)
                    .with_system(move_player)
                    .with_system(spawn_player),
            )
            .register_type::<PlayerControl>();
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

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerControl {
    move_speed: f32,
    rotate_speed: f32,
}

impl Default for PlayerControl {
    fn default() -> Self {
        Self {
            move_speed: 10.,
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
            let (move_speed, rotate_speed) = {
                let mut move_speed = 10f32;
                let mut rotate_speed = 10f32;
    
                for field in instance.field_instances.iter() {
                    bevy::log::info!("Field ID: {}, Value: {:?}", &field.identifier, &field.value);
                    match field.identifier.as_str() {
                        "MoveSpeed" => {
                            if let FieldValue::Float(Some(speed)) = field.value
                            {
                                move_speed = speed;
                            }
                        }
                        "RotateSpeed" => {
                            if let FieldValue::Float(Some(speed)) = field.value
                            {
                                rotate_speed = speed;
                            }
                        }
                        _ => {}
                    }
                }
    
                (move_speed, rotate_speed * PI / 180.)
            };
    
            commands
                .spawn_bundle(MaterialMesh2dBundle {
                    mesh: meshes
                        .add(Mesh::from(shape::Circle::default()))
                        .into(),
                    transform: transform.with_scale(Vec3::new(5., 5., 5.)),
                    material: materials.add(ColorMaterial::from(Color::RED)),
                    ..default()
                })
                .insert(PlayerControl {move_speed, rotate_speed})
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
                })
                .insert(RigidBody::Dynamic)
                .insert(CollisionShape::Sphere { radius: 3.})
                .insert(PhysicMaterial { restitution: 0.9, friction: 0.1, density: 10.0, ..Default::default() })
                .insert(Velocity::from_linear(Vec3::ZERO));
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
        (&ActionState<Action>, &Transform, &PlayerControl, &mut Velocity),
        With<PlayerControl>,
    >,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();

    for (action, transform, PlayerControl{move_speed, rotate_speed}, mut velocity) in query.iter_mut() {
        let mut z_rotation = 0.;
        if action.pressed(Action::RotateRight) {
            z_rotation = z_rotation - *rotate_speed;
        } else if action.pressed(Action::RotateLeft) {
            z_rotation = z_rotation + *rotate_speed;
        }
        velocity.angular = AxisAngle::new(Vec3::Z, z_rotation);

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

        velocity.linear =
            direction_vector.normalize_or_zero() * *move_speed;
    }
}
