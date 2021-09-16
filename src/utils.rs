use bevy::{
    asset::{Asset, AssetPathId, HandleId, LabelId},
    prelude::*,
};
use bevy_spicy_aseprite::AsepriteImage;

#[derive(Debug, Default)]
pub struct UtilsPlugin;

impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(AsepriteTileAtlasBundle::keep_in_sync);
    }
}

pub trait GetSubHandle {
    fn get_sub_handle<T: Asset>(&self, label: &str, assets: &mut Assets<T>) -> Handle<T>;
}

impl<A: Asset> GetSubHandle for Handle<A> {
    fn get_sub_handle<T: Asset>(&self, label: &str, assets: &mut Assets<T>) -> Handle<T> {
        let id = self.id;
        match id {
            HandleId::Id(_, _) => Handle::default(),
            HandleId::AssetPathId(asset_path_id) => {
                let source_path = asset_path_id.source_path_id();
                let new_label: LabelId = Some(label).into();

                let serialize = ron::ser::to_string(&(source_path, new_label)).unwrap();

                let new_asset_path_id: AssetPathId = ron::de::from_str(&serialize).unwrap();

                let mut handle = Handle::weak(new_asset_path_id.into());
                handle.make_strong(assets);

                handle
            }
        }
    }
}

#[derive(Debug)]
pub struct AsepriteTextureAtlasConfiguration {
    pub tile_size: Vec2,
    pub columns: usize,
    pub rows: usize,
    pub padding: Vec2,
}

#[derive(Debug, Bundle)]
pub struct AsepriteTileAtlasBundle {
    pub aseprite_handle: Handle<AsepriteImage>,
    pub texture_atlas_handle: Handle<TextureAtlas>,
    pub texture_atlas_config: AsepriteTextureAtlasConfiguration,
}

impl AsepriteTileAtlasBundle {
    pub fn new(
        aseprite_handle: Handle<AsepriteImage>,
        texture_atlas_handle: Handle<TextureAtlas>,
        texture_atlas_config: AsepriteTextureAtlasConfiguration,
    ) -> Self {
        AsepriteTileAtlasBundle {
            aseprite_handle,
            texture_atlas_handle,
            texture_atlas_config,
        }
    }

    fn keep_in_sync(
        mut aseprite_events: EventReader<AssetEvent<AsepriteImage>>,
        mut texture_atlas_assets: ResMut<Assets<TextureAtlas>>,
        mut texture_assets: ResMut<Assets<Texture>>,
        mut ase_query: Query<(
            &Handle<AsepriteImage>,
            &mut Handle<TextureAtlas>,
            &AsepriteTextureAtlasConfiguration,
        )>,
    ) {
        let images = aseprite_events.iter().flat_map(|event| match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => Some(handle),
            AssetEvent::Removed { .. } => None,
        });

        for changed_aseprite_handle in images {
            for (aseprite_handle, mut texture_atlas_handle, aseprite_atlas_configuration) in
                ase_query.iter_mut()
            {
                if changed_aseprite_handle != aseprite_handle {
                    continue;
                }

                let texture_atlas = TextureAtlas::from_grid_with_padding(
                    aseprite_handle.get_sub_handle("Frame0", &mut texture_assets),
                    aseprite_atlas_configuration.tile_size,
                    aseprite_atlas_configuration.columns,
                    aseprite_atlas_configuration.rows,
                    aseprite_atlas_configuration.padding,
                );

                *texture_atlas_handle =
                    texture_atlas_assets.set(&*texture_atlas_handle, texture_atlas);

                info!(
                    "Updated texture atlas: {:?} from {:?}",
                    texture_atlas_handle, aseprite_handle
                );
            }
        }
    }
}
