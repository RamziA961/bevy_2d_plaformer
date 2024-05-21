use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub mod hud;
pub mod menu;

#[derive(AssetCollection, Resource)]
pub struct FontAssetCollection {
    #[asset(path = "fonts/pixeloid-regular.ttf")]
    pub regular: Handle<Font>,

    #[asset(path = "fonts/pixeloid-bold.ttf")]
    pub bold: Handle<Font>,

    #[asset(path = "fonts/pixeloid-mono.ttf")]
    pub mono: Handle<Font>,
}

#[derive(Component)]
struct RefreshableUIElement;

pub fn load_assets(app: &mut App) {
    info!("Loading UI assets");
    app.init_collection::<FontAssetCollection>();
    info!("Loading UI assets successful");
}

pub fn initialize_systems(app: &mut App) {
    hud::intialize_hud_systems(app);
}
