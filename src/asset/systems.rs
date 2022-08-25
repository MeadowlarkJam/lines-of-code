use crate::consts::{
    ASSET_FONTS_DEFAULT, ASSET_SPRITES_DEBRIS, ASSET_SPRITES_PLAYER, ASSET_SPRITES_ZAPPER, ASSET_SPRITES_SHIELD, ASSET_SPRITES_FORCEFIELD,
};
use bevy::prelude::*;

pub fn load_ingame_assets_system(asset_server: Res<AssetServer>) {
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_DEBRIS);
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_ZAPPER);
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_PLAYER);
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_SHIELD);
    let _ = asset_server.load::<Image, &str>(ASSET_SPRITES_FORCEFIELD);
    println!("Loaded ingame assets");
}

pub fn load_ui_assets_system(asset_server: Res<AssetServer>) {
    let _ = asset_server.load::<Font, &str>(ASSET_FONTS_DEFAULT);
}
