use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::states::States;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_loading_state(
                LoadingState::new(States::Loading)
                .continue_to_state(States::Menu)
                .with_collection::<LoadedAssets>()
            );
    }
}

#[derive(AssetCollection)]
pub struct LoadedAssets {
    #[asset(path = "MotionPicture_PersonalUseOnly.ttf")]
    pub font: Handle<Font>,

    #[asset(path = "bevy jam song - 0002 - Instrument - Drums1.ogg")]
    pub drums_1: Handle<AudioSource>
}