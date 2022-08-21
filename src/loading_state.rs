use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_ecs_ldtk::prelude::*;

use crate::states::States;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(States::Loading)
                .continue_to_state(States::Menu)
                .with_collection::<LoadedAssets>(),
        );
    }
}

#[derive(AssetCollection)]
pub struct LoadedAssets {
    #[asset(path = "MotionPicture_PersonalUseOnly.ttf")]
    pub font: Handle<Font>,
    
    #[asset(path = "level-test.ldtk")]
    pub test_level: Handle<LdtkAsset>,

    // #[asset(path = "bevy jam song - 0002 - Instrument - Drums1.ogg")]
    // pub drums_1: Handle<AudioSource>,
    // #[asset(path = "bevy jam song - 0006 - Instrument - Drums 2.ogg")]
    // pub drums_2: Handle<AudioSource>,
    // #[asset(path = "bevy jam song - 0011 - Instrument - Drums 3.ogg")]
    // pub drums_3: Handle<AudioSource>,

    // #[asset(path = "bevy jam song - 0003 - Instrument - Bass 1.ogg")]
    // pub bass_1: Handle<AudioSource>,
    // #[asset(path = "bevy jam song - 0010 - Instrument - Bass 2.ogg")]
    // pub bass_2: Handle<AudioSource>,

    // #[asset(path = "bevy jam song - 0004 - Instrument - Piano 1.ogg")]
    // pub piano: Handle<AudioSource>,
    // #[asset(path = "bevy jam song - 0005 - Instrument - Sax.ogg")]
    // pub sax: Handle<AudioSource>,
    // #[asset(path = "bevy jam song - 0007 - Instrument - Strings.ogg")]
    // pub strings: Handle<AudioSource>,
    // #[asset(path = "bevy jam song - 0008 - Instrument - Clarinet.ogg")]
    // pub clarinet: Handle<AudioSource>,
    // #[asset(path = "bevy jam song - 0009 - Instrument - Keys.ogg")]
    // pub keys: Handle<AudioSource>,
    // #[asset(path = "bevy jam song - 0012 - Instrument - Guitar.ogg")]
    // pub guitar: Handle<AudioSource>,
    // #[asset(path = "bevy jam song - 0013 - Instrument - Guitar Lead.ogg")]
    // pub lead_guitar: Handle<AudioSource>,
}
