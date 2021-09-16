use bevy::prelude::*;

use crate::GameAssets;

use super::LevelMap;

/// The world management entity
#[derive(Debug)]
pub struct WorldLevels {
    pub level_handle: Handle<crate::levels::Project>,
    pub level_map: LevelMap,
    pub main_entity: Entity,
}

pub fn setup_levels(mut commands: Commands, game_assets: Res<GameAssets>) {
    let main_entity = commands.spawn().id();

    commands.insert_resource(WorldLevels {
        level_handle: game_assets.levels.clone(),
        level_map: LevelMap::default(),
        main_entity,
    });
}

pub fn remove_level(mut commands: Commands, world: Res<WorldLevels>) {
    commands.entity(world.main_entity).despawn_recursive();
    commands.remove_resource::<WorldLevels>();
}
