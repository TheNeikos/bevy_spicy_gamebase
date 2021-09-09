use bevy::{prelude::*, utils::HashMap};
use bevy_spicy_aseprite::AsepriteImage;

#[derive(Debug, Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Debug, Bundle)]
pub struct WorldBundle {
    aseprite_handle: Handle<AsepriteImage>,
    levels: HashMap<String, Entity>,
}
