use std::f32::consts::PI;

use bevy::{prelude::*};
use bevy_ecs_ldtk::{prelude::FieldValue, EntityInstance};

use heron::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{
    interactive_narrative::SetCurrentKnotEvent,
    level::LevelElement,
    loading_state::LoadedAssets,
    physics::GameCollisionLayers,
    spirit::CharacterAtlas,
    states::{GameMode, States},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .add_system_set(
                SystemSet::on_update(States::InGame)
                    .with_system(spawn_player)
                    .with_system(animate_player)
                    .with_system(animate_companion),
            )
            .add_system_set(
                SystemSet::on_update(GameMode::Exploration)
                    .with_system(setup_player_control)
                    .with_system(move_player),
            )
            .add_system_set(
                SystemSet::on_enter(GameMode::Conversation)
                    .with_system(stop_player),
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
    Interact,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerControl {
    move_speed: f32,
    rotate_speed: f32,
}

#[derive(Component)]
pub struct Companion;

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
    _meshes: ResMut<Assets<Mesh>>,
    _materials: ResMut<Assets<ColorMaterial>>,
    entities: Query<(&EntityInstance, &Transform), Added<EntityInstance>>,
    mut event_writer: EventWriter<SetCurrentKnotEvent>,
    assets: Res<LoadedAssets>,
    _asset_server: Res<AssetServer>,
    texture_atlas: Option<Res<CharacterAtlas>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
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

    for (instance, transform) in entities.iter() {
        if instance.identifier == "Player" {
            let (move_speed, rotate_speed) = {
                let mut move_speed = 10f32;
                let mut rotate_speed = 10f32;
                let mut set_knot = false;

                for field in instance.field_instances.iter() {
                    bevy::log::info!(
                        "Field ID: {}, Value: {:?}",
                        &field.identifier,
                        &field.value
                    );
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
                        "LevelStartKnot" => {
                            if let FieldValue::String(Some(knot)) = &field.value
                            {
                                event_writer.send(SetCurrentKnotEvent(Some(
                                    knot.clone(),
                                )));
                                set_knot = true;
                            }
                        }
                        _ => {}
                    }
                }

                if !set_knot {
                    event_writer.send(SetCurrentKnotEvent(None));
                }

                (move_speed, rotate_speed * PI / 180.)
            };

            commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: PLAYER_IDLE_START,
                        ..default()
                    },
                    texture_atlas: atlas_handle.clone(),
                    transform: transform.with_scale(Vec3::ONE * 0.5),
                    ..default()
                })
                .insert(LevelElement)
                .insert(PlayerControl {
                    move_speed,
                    rotate_speed,
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(SpriteSheetBundle {
                            sprite: TextureAtlasSprite {
                                index: CASS_START,
                                ..default()
                            },
                            texture_atlas: atlas_handle.clone(),
                            transform: Transform::default()
                                .with_translation(Vec3::new(-3., -11.5, 1.))
                                .with_scale(Vec3::new(0.8, 0.8, 0.8)),
                            ..default()
                        })
                        .insert(Companion);
                })
                .insert(RigidBody::Dynamic)
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
                        .with_group(GameCollisionLayers::Player),
                );
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
                    (KeyCode::Return, Action::Interact),
                    (KeyCode::Space, Action::Interact),
                ]),
            });
    }
}

fn move_player(
    mut query: Query<
        (
            &ActionState<Action>,
            &Transform,
            &PlayerControl,
            &mut Velocity,
        ),
        With<PlayerControl>,
    >,
    time: Res<Time>,
) {
    let _delta = time.delta().as_secs_f32();

    for (
        action,
        transform,
        PlayerControl {
            move_speed,
            rotate_speed,
        },
        mut velocity,
    ) in query.iter_mut()
    {
        let mut z_rotation = 0.;
        if action.pressed(Action::RotateRight) {
            z_rotation -= *rotate_speed;
        } else if action.pressed(Action::RotateLeft) {
            z_rotation += *rotate_speed;
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

        velocity.linear = direction_vector.normalize_or_zero() * *move_speed;
    }
}

fn stop_player(mut query: Query<&mut Velocity, With<PlayerControl>>) {
    for mut velocity in query.iter_mut() {
        velocity.linear = Vec3::ZERO;
        velocity.angular = AxisAngle::new(Vec3::Z, 0.);
    }
}

const PLAYER_IDLE_START: usize = 3;
const PLAYER_IDLE_LEN: usize = 5;

const PLAYER_WALK_START: usize = 19;
const PLAYER_WALK_LEN: usize = 4;

const CASS_START: usize = 0;
const CASS_LEN: usize = 3;

fn animate_player(
    mut players: Query<
        (&mut TextureAtlasSprite, &Velocity),
        With<PlayerControl>,
    >,
    time: Res<Time>,
) {
    let time = (time.seconds_since_startup() * 5.) as usize;
    for (mut sprite, velocity) in players.iter_mut() {
        if velocity.linear.length() < 0.1 {
            let current_index =
                PLAYER_IDLE_START + ((time / 10) % PLAYER_IDLE_LEN);
            sprite.index = current_index;
        } else {
            let current_index = PLAYER_WALK_START + (time % PLAYER_WALK_LEN);
            sprite.index = current_index;
        }
    }
}

fn animate_companion(
    mut companions: Query<
        (&mut TextureAtlasSprite, &mut Transform),
        With<Companion>,
    >,
    time: Res<Time>,
) {
    let time = (time.seconds_since_startup() * 5.) as usize;
    for (mut sprite, mut transform) in companions.iter_mut() {
        let current_index = CASS_START + (time % CASS_LEN);
        sprite.index = current_index;
        let position_horizontal = ((time as f32) / 3.).sin() * 6.;
        let position_vertical = ((time as f32) / 4.).cos() * 3.;

        transform.translation = Vec3::new(-3., -11.5, 1.)
            + Vec3::new(position_horizontal, position_vertical, 0.);
    }
}
