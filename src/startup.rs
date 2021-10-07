use bevy::prelude::*;
use bevy_loading::prelude::AssetsLoading;

use crate::{
    camera::Free2DCamera,
    utils::{AsepriteTextureAtlasConfiguration, AsepriteTileAtlasBundle},
    GameAssets,
};

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
    texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
    mut loading: ResMut<AssetsLoading>,
) {
    asset_server.watch_for_changes().unwrap();

    let world_sprites = asset_server.load("world.aseprite");
    let world_tile_atlas = texture_atlas_assets.get_handle(Handle::<TextureAtlas>::default());

    commands.spawn_bundle(AsepriteTileAtlasBundle::new(
        world_sprites.clone(),
        world_tile_atlas.clone(),
        AsepriteTextureAtlasConfiguration {
            tile_size: Vec2::new(16., 16.),
            columns: 8,
            rows: 8,
            padding: Vec2::ZERO,
        },
    ));

    let game_assets = GameAssets {
        config: asset_server.load("game.config"),
        levels: asset_server.load("world.ldtk"),
        world_sprites,
        world_tile_atlas,
        ui_sprites: asset_server.load("ui.aseprite"),
        main_font: asset_server.load("PressStart2P-Regular.ttf"),
    };

    game_assets.add_to_loadtracker(&mut loading);

    commands.insert_resource(game_assets);
}

fn spawn_cameras(mut commands: Commands) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(Free2DCamera::new(1.0).with_scale_range((1.)..=4.));

    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_clear_color(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::rgb_u8(163, 169, 194)));
}
