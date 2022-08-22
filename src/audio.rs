use std::f32::consts::PI;

use crate::{spirit_collection::Collected, states::States};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use super::player::PlayerControl;

pub struct AudioPlayerPlugin;

impl Plugin for AudioPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(
                SystemSet::on_update(States::InGame).with_system(play_loop),
            )
            .add_system_set(
                SystemSet::on_update(States::InGame)
                    .with_system(adjust_audio_loop_position_and_volume)
                    .with_system(cleanup_collected_audio),
            );
    }
}

#[derive(Component)]
struct AudioInstanceHandle(Handle<AudioInstance>);

#[derive(Component)]
pub struct AudioEmitter(pub Handle<AudioSource>, pub String);

const AUDIO_RANGE: f32 = 50.;

fn play_loop(
    mut commands: Commands,
    audio: Res<Audio>,
    emitters: Query<(Entity, &AudioEmitter), Without<AudioInstanceHandle>>,
) {
    for (entity, emitter) in emitters.iter() {
        let handle = audio
            .play(emitter.0.clone())
            .looped()
            .with_volume(0.)
            .handle();
        commands.entity(entity).insert(AudioInstanceHandle(handle));
    }
}

fn adjust_audio_loop_position_and_volume(
    mut instances: ResMut<Assets<AudioInstance>>,
    emitters: Query<(&Transform, &AudioInstanceHandle, &AudioEmitter)>,
    target: Query<&Transform, With<PlayerControl>>,
) {
    let target = target.get_single();

    if let Ok(target) = target {
        for (emitter, handle, emitter_info) in emitters.iter() {
            if let Some(instance) = instances.get_mut(&handle.0) {
                let diff = emitter.translation - target.translation;
                let volume = (AUDIO_RANGE - diff.length()) / AUDIO_RANGE;

                let direction = diff.normalize_or_zero();
                let facing =
                    target.rotation.mul_vec3(Vec3::Y).normalize_or_zero();

                let angle = -1.
                    * Quat::from_rotation_arc(facing, direction)
                        .to_euler(EulerRot::XYZ)
                        .2;

                let pan = (angle.sin() + 1.) / 2.;
                let volume =
                    volume * 0.9 + volume * 0.1 * (1. - angle.abs() / PI);
                let volume = volume.clamp(0., 1.);
                let pan = pan.clamp(0., 1.);
                bevy::log::debug!(
                    "{} - Angle: {} Volume: {}, Pan: {}",
                    emitter_info.1,
                    angle,
                    volume,
                    pan
                );

                instance.set_volume(volume.into(), AudioTween::default());
                instance.set_panning(pan.into(), AudioTween::default());
            }
        }
    }
}

fn cleanup_collected_audio(
    mut instances: ResMut<Assets<AudioInstance>>,
    emitters: Query<&AudioInstanceHandle, With<Collected>>,
) {
    for handle in emitters.iter() {
        if let Some(instance) = instances.get_mut(&handle.0) {
            instance.stop(AudioTween::default());
        }
        instances.remove(&handle.0);
    }
}
