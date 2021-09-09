use bevy::prelude::*;
use bevy_loading::prelude::AssetsLoading;

use crate::GameAssets;

#[derive(Debug, Default)]
pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_assets);
        app.add_startup_system(spawn_cameras);
        app.add_startup_system(setup_clear_color);
    }
}

fn load_assets(
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    asset_server.watch_for_changes().unwrap();

    let game_assets = GameAssets {
        config: asset_server.load("game.config"),
        levels: asset_server.load("world.ldtk"),
        world_sprites: asset_server.load("world.aseprite"),
        ui_sprites: asset_server.load("ui.aseprite"),
    };

    game_assets.add_to_loadtracker(&mut loading);

    commands.insert_resource(game_assets);
}

fn spawn_cameras(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_clear_color(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::rgb_u8(163, 169, 194)));
}
