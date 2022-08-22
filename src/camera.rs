use bevy::prelude::*;

use crate::{player::PlayerControl, states::States};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_camera).add_system_set(
            SystemSet::on_update(States::InGame)
                .with_system(follow_user)
                .with_system(set_user_position),
        );
    }
}

#[derive(Component)]
struct FollowCam(Vec3, f32);

fn load_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.2,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FollowCam(Vec3::ZERO, 50.));
}

fn set_user_position(
    mut camera: Query<
        (&mut Transform, &mut FollowCam),
        (With<Camera>, Without<PlayerControl>),
    >,
    player: Query<&GlobalTransform, Added<PlayerControl>>,
) {
    let target = player.get_single();

    if let Ok(target) = target {
        let target = target.translation();

        for (mut transform, mut follow) in camera.iter_mut() {
            transform.translation += target;
            follow.0 = Vec3::ZERO;
        }
    }
}

fn follow_user(
    mut camera: Query<
        (&mut Transform, &mut FollowCam),
        (With<Camera>, Without<PlayerControl>),
    >,
    player: Query<&GlobalTransform, With<PlayerControl>>,
    time: Res<Time>,
) {
    let delta = time.delta().as_secs_f32();
    let target = player.get_single();

    if let Ok(target) = target {
        let target = target.translation();

        for (mut transform, mut follow) in camera.iter_mut() {
            let direction = target - transform.translation;
            let direction = direction.normalize();
            let distance = follow.1.clone();

            follow.0 += direction * delta * distance;

            if follow.0.length() > distance {
                follow.0 = follow.0.normalize() * (distance - 1.);
            }

            transform.translation += follow.0 * delta;
            transform.translation.z = 99.;
        }
    }
}
