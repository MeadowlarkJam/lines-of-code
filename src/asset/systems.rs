use bevy::prelude::*;

pub fn load_assets_system(mut commands: Commands, mut asset_server: Res<AssetServer>) {
    let _ = asset_server.load::<Image, &str>("zapper.png");
    let _ = asset_server.load::<Image, &str>("debris.png");
    let _ = asset_server.load::<Image, &str>("player.png");
}
