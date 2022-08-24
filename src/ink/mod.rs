use bevy::prelude::*;

use self::{ink_asset::*, ink_story::StoryEvent};

pub mod ink_asset;
pub mod ink_story;

pub struct InkPlugin;

impl Plugin for InkPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<StoryEvent>()
            .add_asset::<InkAsset>()
            .init_asset_loader::<InkAssetLoader>();
    }
}