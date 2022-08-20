use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<Action>::default())
            .init_resource::<PlayerControlSettings>()
            .add_system(setup_player_control)
            .add_system(move_player);
    }
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
}

#[derive(Component)]
pub struct PlayerControl;

pub struct PlayerControlSettings {
    move_speed: f32
}

impl Default for PlayerControlSettings {
    fn default() -> Self {
        Self {
            move_speed: 100.
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
                // Describes how to convert from player inputs into those actions
                input_map: InputMap::new([
                    (KeyCode::Up, Action::MoveUp),
                    (KeyCode::Down, Action::MoveDown),
                    (KeyCode::Left, Action::MoveLeft),
                    (KeyCode::Right, Action::MoveRight),
                ]),
            });
    }
}

fn move_player(mut query: Query<(&ActionState<Action>, &mut Transform), With<PlayerControl>>, player_settings: Res<PlayerControlSettings>, time: Res<Time>) {
    let delta = time.delta().as_secs_f32();
    let speed = player_settings.move_speed;
    for (action, mut transform) in query.iter_mut() {
        let movement = Vec3::new(if action.pressed(Action::MoveLeft) {
            -1.
        } else if action.pressed(Action::MoveRight) {
            1.
        } else {
            0.
        },
        if action.pressed(Action::MoveUp) {
            1.
        } else if action.pressed(Action::MoveDown) {
            -1.
        } else {
            0.
        },
    0.);
        transform.translation += movement.normalize_or_zero() * delta * speed;
    }
}