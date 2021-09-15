use bevy::{
    asset::{Asset, AssetPathId, HandleId, LabelId},
    prelude::*,
};

pub trait GetSubHandle {
    fn get_sub_handle<T: Asset>(&self, label: &str, assets: &mut Assets<T>) -> Handle<T>;
}

impl<A: Asset> GetSubHandle for Handle<A> {
    fn get_sub_handle<T: Asset>(&self, label: &str, assets: &mut Assets<T>) -> Handle<T> {
        let id = self.id;
        match id {
            HandleId::Id(_, _) => Handle::default(),
            HandleId::AssetPathId(asset_path_id) => {
                let new_asset_path_id: AssetPathId = asset_path_id;

                let source_path = new_asset_path_id.source_path_id();
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
