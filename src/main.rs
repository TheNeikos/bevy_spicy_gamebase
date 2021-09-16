#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_simple_tilemap::plugin::SimpleTileMapPlugin;
use bevy_spicy_aseprite::AsepriteImage;

// Import the world aseprite as used in the world ldtk
bevy_spicy_aseprite::aseprite!(pub world_sprites, "assets/world.aseprite");
// Import the world aseprite as used in the world ldtk
bevy_spicy_aseprite::aseprite!(pub ui_sprites, "assets/ui.aseprite");
// The world data
bevy_spicy_ldtk::ldtk!(pub levels, "assets/world.ldtk");
// Configuration for the game
bevy_spicy_data::data_config!(pub config, "assets/game.config");

mod camera;
mod stages;
mod startup;
mod ui;
mod utils;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SimpleTileMapPlugin)
        .add_plugin(bevy_spicy_data::TomlConfigPlugin::<config::Root>::default())
        .add_plugin(bevy_spicy_aseprite::AsepritePlugin)
        .add_plugin(bevy_spicy_ldtk::LdtkPlugin::<levels::Project>::default())
        .add_plugin(utils::UtilsPlugin::default())
        .add_plugin(ui::UiPlugin::default())
        .add_plugin(camera::CameraPlugin::default())
        .add_plugin(stages::StagesPlugin::default())
        .add_plugin(startup::StartupPlugin::default())
        .add_plugin(world::WorldPlugin::default())
        .run();
}

pub struct GameAssets {
    pub config: Handle<config::Root>,
    pub levels: Handle<levels::Project>,
    pub world_sprites: Handle<AsepriteImage>,
    pub world_tile_atlas: Handle<TextureAtlas>,
    pub ui_sprites: Handle<AsepriteImage>,
    pub main_font: Handle<Font>,
}
impl GameAssets {
    fn add_to_loadtracker(&self, loading: &mut bevy_loading::prelude::AssetsLoading) {
        loading.add(&self.config);
        loading.add(&self.levels);
        loading.add(&self.world_sprites);
        loading.add(&self.world_tile_atlas);
        loading.add(&self.ui_sprites);
        loading.add(&self.main_font);
    }
}
