use std::f32::consts::PI;

use crate::{states::States, loading_state::LoadedAssets};
use bevy::{prelude::*, render::render_resource::encase::rts_array::Length};
use bevy_kira_audio::prelude::*;

use super::player::PlayerControl;

pub struct AudioPlayerPlugin;

impl Plugin for AudioPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(AudioPlugin)
            .add_system_set(
                SystemSet::on_enter(States::InGame).with_system(play_loop),
            )
            .add_system_set(
                SystemSet::on_update(States::InGame).with_system(play_loop),
            );
    }
}

struct AudioInstanceHandle(Handle<AudioInstance>);

#[derive(Component)]
pub struct AudioEmitter;

fn play_loop(
    mut commands: Commands,
    audio: Res<Audio>,
     assets: Res<LoadedAssets>
) {
    let handle = audio
        .play(
            assets.drums_1.clone(),
        )
        .looped()
        .handle();
    commands.insert_resource(AudioInstanceHandle(handle));
}

fn adjust_audio_loop_position_and_volume(
    handle: Res<AudioInstanceHandle>,
    mut instances: ResMut<Assets<AudioInstance>>,
    emitter: Query<&Transform, With<AudioEmitter>>,
    target: Query<&Transform, With<PlayerControl>>,
) {
    let emitter = emitter.get_single();
    let target = target.get_single();

    if let (Some(mut instance), Ok(emitter), Ok(target)) =
        (instances.get_mut(&handle.0), emitter, target)
    {
        let diff = emitter.translation - target.translation;
        let volume = (1000. - diff.length()) / 1000.;

        let direction = diff.normalize_or_zero();
        let facing = target.rotation.mul_vec3(Vec3::Y).normalize_or_zero();

        let angle = -1.
            * Quat::from_rotation_arc(facing, direction)
                .to_euler(EulerRot::XYZ)
                .2;

        let pan = angle.sin();
        let volume = volume * 0.8 + volume * 0.2 * (1. - angle.abs() / PI);
        bevy::log::info!("Angle: {} Volume: {}", angle, volume);

        instance.set_volume(volume.into(), AudioTween::default());
        instance.set_panning(pan.into(), AudioTween::default());
    }
}
