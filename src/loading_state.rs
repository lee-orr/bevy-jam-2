use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_ecs_ldtk::prelude::*;


use crate::{ink::ink_asset::InkAsset, states::States};

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

    #[asset(path = "test.ink")]
    pub test_ink: Handle<InkAsset>,

    #[asset(path = "characters.png")]
    pub character_atlas: Handle<Image>,
}
