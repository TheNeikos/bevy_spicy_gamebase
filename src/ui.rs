use aseprite_reader::NineSlice;
use bevy::{
    ecs::{entity, system::EntityCommands},
    prelude::*,
};
use bevy_spicy_aseprite::AsepriteImage;

use crate::utils::GetSubHandle;

#[derive(Debug, Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn create_nine_patch(
    mut entity_builder: &mut EntityCommands,
    slice: &aseprite_reader::AsepriteSlice,
    aseprite_handle: Handle<AsepriteImage>,
    materials: &mut Assets<ColorMaterial>,
    texture_assets: &mut Assets<Texture>,
) -> Entity {
    let mut ret = None;

    let nine_patch_info = if let Some(nine_patch_info) = slice.nine_patch_info.as_ref() {
        nine_patch_info
    } else {
        error!("No ninepatch in slice given: {}", slice.name);
        panic!();
    };

    let tl_x = nine_patch_info.x_center;
    let tl_y = nine_patch_info.y_center;

    entity_builder.with_children(|parent| {
        ret = Some(
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: Rect {
                            left: Val::Px(tl_x as f32),
                            top: Val::Px(tl_y as f32),
                            bottom: Val::Px(tl_y as f32),
                            right: Val::Px(tl_x as f32),
                        },
                        ..Default::default()
                    },
                    material: materials.add(ColorMaterial::texture(
                        aseprite_handle.get_sub_handle(
                            &format!("Slice/{}/Center", slice.name),
                            texture_assets,
                        ),
                    )),
                    ..Default::default()
                })
                .id(),
        );
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(tl_x as f32),
                    height: Val::Px(tl_y as f32),
                },
                ..Default::default()
            },
            material: materials.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &format!("Slice/{}/{:?}", slice.name, NineSlice::TopLeft),
                texture_assets,
            ))),
            ..Default::default()
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.),
                    right: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(tl_x as f32),
                    height: Val::Px(tl_y as f32),
                },
                ..Default::default()
            },
            material: materials.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &format!("Slice/{}/{:?}", slice.name, NineSlice::TopRight),
                texture_assets,
            ))),
            ..Default::default()
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(tl_x as f32),
                    top: Val::Px(0.),
                    right: Val::Px(tl_x as f32),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Px(tl_y as f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &format!("Slice/{}/{:?}", slice.name, NineSlice::TopCenter),
                texture_assets,
            ))),
            ..Default::default()
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(tl_x as f32),
                    height: Val::Px(tl_y as f32),
                },
                ..Default::default()
            },
            material: materials.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &format!("Slice/{}/{:?}", slice.name, NineSlice::BottomLeft),
                texture_assets,
            ))),
            ..Default::default()
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(0.),
                    right: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(tl_x as f32),
                    height: Val::Px(tl_y as f32),
                },
                ..Default::default()
            },
            material: materials.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &format!("Slice/{}/{:?}", slice.name, NineSlice::BottomRight),
                texture_assets,
            ))),
            ..Default::default()
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(tl_x as f32),
                    bottom: Val::Px(0.),
                    right: Val::Px(tl_x as f32),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Px(tl_y as f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &format!("Slice/{}/{:?}", slice.name, NineSlice::BottomCenter),
                texture_assets,
            ))),
            ..Default::default()
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.),
                    bottom: Val::Px(tl_y as f32),
                    top: Val::Px(tl_y as f32),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(tl_y as f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &format!("Slice/{}/{:?}", slice.name, NineSlice::LeftCenter),
                texture_assets,
            ))),
            ..Default::default()
        });
        parent.spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(0.),
                    bottom: Val::Px(tl_y as f32),
                    top: Val::Px(tl_y as f32),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(tl_y as f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &format!("Slice/{}/{:?}", slice.name, NineSlice::RightCenter),
                texture_assets,
            ))),
            ..Default::default()
        });
    });

    ret.unwrap()
}
