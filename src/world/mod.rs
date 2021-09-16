mod startup;

use crate::{stages::GameState, GameAssets};
use bevy::{prelude::*, utils::HashMap};
use bevy_simple_tilemap::{prelude::TileMapBundle, Tile, TileFlags, TileMap};

use self::startup::WorldLevels;

#[derive(Debug, SystemLabel, Clone, Copy, Hash, PartialEq, Eq)]
enum WorldSystems {
    WorldSetup,
    InitialLoad,
}

#[derive(Debug, Default)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Running)
                .with_system(startup::setup_levels.label(WorldSystems::WorldSetup))
                .with_system(
                    insert_configured_levels
                        .label(WorldSystems::InitialLoad)
                        .after(WorldSystems::WorldSetup),
                ),
        );
        app.add_system_set(SystemSet::on_update(GameState::Running).with_system(load_new_levels));
        app.add_system_set(
            SystemSet::on_exit(GameState::Running).with_system(startup::remove_level),
        );
    }
}

#[derive(Debug, Default)]
pub struct LevelMap {
    levels: HashMap<String, Entity>,
}

#[derive(Debug, Default)]
pub struct LevelName(pub String);

#[derive(Debug, Default)]
pub struct Level;

#[derive(Debug, Default, Bundle)]
pub struct LevelBundle {
    pub level: Level,
    pub level_name: LevelName,
}

pub struct DefaultLevels(pub Vec<String>);

fn insert_configured_levels(mut commands: Commands, default_levels: Option<Res<DefaultLevels>>) {
    if let Some(default_levels) = default_levels {
        for level in &default_levels.0 {
            commands.spawn_bundle(LevelBundle {
                level_name: LevelName(level.clone()),
                ..Default::default()
            });
        }
    } else {
        info!("No default levels configured");
    }
}

fn load_new_levels(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    ldtk_assets: Res<Assets<crate::levels::Project>>,
    mut world_level: ResMut<WorldLevels>,
    level_query: Query<(Entity, &LevelName), Added<Level>>,
) {
    for (entity, level_name) in level_query.iter() {
        world_level
            .level_map
            .levels
            .insert(level_name.0.clone(), entity);

        let ldtk_project = if let Some(ldtk) = ldtk_assets.get(&world_level.level_handle) {
            ldtk
        } else {
            error!("Could not get ldtk level for newly spawned level");
            continue;
        };

        let ldtk_level = if let Some(level) = ldtk_project
            .levels
            .iter()
            .find(|level| level.identifier == level_name.0)
        {
            level
        } else {
            error!("Could not find level with name {} in project", level_name.0);
            continue;
        };

        info!("Spawned the level: {}", level_name.0);
        info!("Tile atlas: {:?}", game_assets.world_tile_atlas);

        commands.entity(entity).with_children(|parent| {
            let mut tilemap = TileMap::default();
            add_layer(&ldtk_level.layers.front, 2, &mut tilemap);

            parent.spawn_bundle(TileMapBundle {
                tilemap,
                texture_atlas: game_assets.world_tile_atlas.clone(),
                ..Default::default()
            });
        });
    }
}

fn add_layer(
    layer: &bevy_spicy_ldtk::Layer<crate::levels::ProjectEntities>,
    height: i32,
    map: &mut TileMap,
) {
    match &layer.special {
        bevy_spicy_ldtk::SpecialValues::IntGrid {
            auto_layer: tiles, ..
        }
        | bevy_spicy_ldtk::SpecialValues::Tiles { tiles, .. }
        | bevy_spicy_ldtk::SpecialValues::AutoLayer { auto_layer: tiles } => {
            for tile in tiles {
                let pos = tile.position_px / layer.grid_size as i32;
                // info!("Spawning at {}", pos);
                let sprite_index = tile.id as _;
                let mut flags = TileFlags::empty();

                if tile.flip_x {
                    flags |= TileFlags::FLIP_X;
                }

                if tile.flip_y {
                    flags |= TileFlags::FLIP_Y;
                }

                map.set_tile(
                    pos.extend(height),
                    Some(Tile {
                        sprite_index,
                        flags,
                        ..Default::default()
                    }),
                );
            }
        }
        bevy_spicy_ldtk::SpecialValues::Entities(_) => (),
    }
}
