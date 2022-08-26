

use bevy::{
    asset::{AssetLoader, LoadedAsset},
    reflect::TypeUuid,
};


#[derive(Debug, TypeUuid)]
#[uuid = "71befef9-babf-4927-b360-1844b7e7fc97"]
pub struct InkAsset {
    pub story: String,
}

#[derive(Default)]
pub struct InkAssetLoader;

impl AssetLoader for InkAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut bevy::asset::LoadContext,
    ) -> bevy::utils::BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let string = String::from_utf8(bytes.into());
            match string {
                Ok(string) => {
                    load_context.set_default_asset(LoadedAsset::new(
                        InkAsset { story: string },
                    ));
                    Ok(())
                }
                Err(_err) => Err(bevy::asset::Error::msg(
                    "Failed to read text from file",
                )),
            }
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ink"]
    }
}
