use aseprite_reader::NineSlice;
use bevy::{
    ecs::{component::Component, system::EntityCommands},
    prelude::*,
    ui::FocusPolicy,
};
use bevy_spicy_aseprite::{AsepriteImage, AsepriteSlice, AsepriteSliceName};

use crate::utils::GetSubHandle;

#[derive(Debug, Default)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_nine_patch_info)
            .add_system(update_nine_patch_image)
            .add_system(update_nine_patch_button);
    }
}

fn update_nine_patch_image(
    aseprite_assets: Res<Assets<AsepriteImage>>,
    mut texture_assets: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut nine_patch_query: Query<
        (
            &Handle<AsepriteImage>,
            &NinePatch,
            &mut Handle<ColorMaterial>,
        ),
        Or<(Changed<NinePatch>, Changed<Handle<AsepriteImage>>)>,
    >,
) {
    for (aseprite_image_handle, nine_patch, mut color_material) in nine_patch_query.iter_mut() {
        let ui_aseprite = if let Some(ui_aseprite) = aseprite_assets.get(aseprite_image_handle) {
            ui_aseprite
        } else {
            return;
        };

        let slices = ui_aseprite.aseprite().slices();

        let slice = if let Some(slice) = slices.get_by_name(&nine_patch.slice_name) {
            slice
        } else {
            error!("Could not find slice: {}", nine_patch.slice_name);
            return;
        };

        *color_material = materials.add(ColorMaterial::texture(
            aseprite_image_handle.get_sub_handle(
                &slice.label_with_nine_slice(nine_patch.nine_patch),
                &mut texture_assets,
            ),
        ));
    }
}

struct NinePatch {
    nine_patch: NineSlice,
    pub slice_name: String,
}

fn update_nine_patch_info(
    mut aseprite_asset_events: EventReader<AssetEvent<AsepriteImage>>,
    aseprite_assets: Res<Assets<AsepriteImage>>,
    mut nine_patch_query: Query<(
        &Handle<AsepriteImage>,
        &NinePatch,
        &mut Style,
        ChangeTrackers<NinePatch>,
    )>,
) {
    let last_event = if let Some(event) = aseprite_asset_events.iter().last() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => Some(handle),
            AssetEvent::Removed { .. } => return,
        }
    } else {
        None
    };

    for (aseprite_image_handle, nine_patch, mut style, change_tracker) in
        nine_patch_query.iter_mut()
    {
        if Some(aseprite_image_handle) != last_event && !change_tracker.is_changed() {
            continue;
        }

        let ui_aseprite = if let Some(ui_aseprite) = aseprite_assets.get(aseprite_image_handle) {
            ui_aseprite
        } else {
            return;
        };

        let slices = ui_aseprite.aseprite().slices();

        let slice = if let Some(slice) = slices.get_by_name(&nine_patch.slice_name) {
            slice
        } else {
            error!("Could not find slice: {}", nine_patch.slice_name);
            return;
        };

        let nine_patch_info = if let Some(nine_patch_info) = slice.nine_patch_info.as_ref() {
            nine_patch_info
        } else {
            error!("No ninepatch in slice given: {}", slice.name);
            continue;
        };

        let from_top_left = Vec2::new(
            nine_patch_info.x_center as f32,
            nine_patch_info.y_center as f32,
        );
        let from_bottom_right = Vec2::new(slice.width as f32, slice.height as f32)
            - (from_top_left
                + Vec2::new(nine_patch_info.width as f32, nine_patch_info.height as f32));

        match nine_patch.nine_patch {
            NineSlice::TopLeft => {
                style.size = Size {
                    width: Val::Px(from_top_left.x),
                    height: Val::Px(from_top_left.y),
                };
            }
            NineSlice::TopCenter => {
                style.position = Rect {
                    left: Val::Px(from_top_left.x),
                    top: Val::Px(0.),
                    right: Val::Px(from_bottom_right.x),
                    ..Default::default()
                };
                style.size = Size {
                    height: Val::Px(from_top_left.y),
                    ..Default::default()
                };
            }
            NineSlice::TopRight => {
                style.position = Rect {
                    top: Val::Px(0.),
                    right: Val::Px(0.),
                    ..Default::default()
                };
                style.size = Size {
                    width: Val::Px(from_bottom_right.x),
                    height: Val::Px(from_top_left.y as f32),
                };
            }
            NineSlice::RightCenter => {
                style.position = Rect {
                    right: Val::Px(0.),
                    bottom: Val::Px(from_bottom_right.y),
                    top: Val::Px(from_top_left.y),
                    ..Default::default()
                };
                style.size = Size {
                    width: Val::Px(from_bottom_right.x),
                    ..Default::default()
                };
            }
            NineSlice::BottomRight => {
                style.position = Rect {
                    bottom: Val::Px(0.),
                    right: Val::Px(0.),
                    ..Default::default()
                };
                style.size = Size {
                    width: Val::Px(from_bottom_right.x),
                    height: Val::Px(from_bottom_right.y),
                };
            }
            NineSlice::BottomCenter => {
                style.position = Rect {
                    left: Val::Px(from_top_left.x),
                    bottom: Val::Px(0.),
                    right: Val::Px(from_bottom_right.x),
                    ..Default::default()
                };
                style.size = Size {
                    height: Val::Px(from_bottom_right.y),
                    ..Default::default()
                };
            }
            NineSlice::BottomLeft => {
                style.position = Rect {
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    ..Default::default()
                };
                style.size = Size {
                    width: Val::Px(from_top_left.x),
                    height: Val::Px(from_bottom_right.y),
                };
            }
            NineSlice::LeftCenter => {
                style.position = Rect {
                    left: Val::Px(0.),
                    bottom: Val::Px(from_bottom_right.y),
                    top: Val::Px(from_top_left.y),
                    ..Default::default()
                };
                style.size = Size {
                    width: Val::Px(from_top_left.x),
                    ..Default::default()
                };
            }
            NineSlice::Center => {
                style.margin = Rect {
                    left: Val::Px(from_top_left.x),
                    top: Val::Px(from_top_left.y),
                    right: Val::Px(from_bottom_right.x),
                    bottom: Val::Px(from_bottom_right.y),
                    ..Default::default()
                };
            }
        }
    }
}

pub fn create_nine_patch<'w, 's, 'a, 'f>(
    commands: &'f mut ChildBuilder<'w, 's, 'a>,
    slice: &aseprite_reader::AsepriteSlice,
    aseprite_handle: Handle<AsepriteImage>,
    material_assets: &mut Assets<ColorMaterial>,
    texture_assets: &mut Assets<Texture>,
    style: Option<Style>,
) -> EntityCommands<'w, 's, 'f> {
    let nine_patch_info = if let Some(nine_patch_info) = slice.nine_patch_info.as_ref() {
        nine_patch_info
    } else {
        error!("No ninepatch in slice given: {}", slice.name);
        return commands.spawn_bundle(NodeBundle::default());
    };

    let from_top_left = Vec2::new(
        nine_patch_info.x_center as f32,
        nine_patch_info.y_center as f32,
    );
    let from_bottom_right = Vec2::new(slice.width as f32, slice.height as f32)
        - (from_top_left + Vec2::new(nine_patch_info.width as f32, nine_patch_info.height as f32));

    let visible = Visible {
        is_visible: true,
        is_transparent: true,
    };

    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.),
                    top: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(from_top_left.x),
                    height: Val::Px(from_top_left.y),
                },
                ..Default::default()
            },
            material: material_assets.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &slice.label_with_nine_slice(NineSlice::TopLeft),
                texture_assets,
            ))),
            // visible: visible.clone(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .insert(aseprite_handle.clone())
        .insert(NinePatch {
            nine_patch: NineSlice::TopLeft,
            slice_name: slice.name.clone(),
        });
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.),
                    right: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(from_bottom_right.x),
                    height: Val::Px(from_top_left.y as f32),
                },
                ..Default::default()
            },
            material: material_assets.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &slice.label_with_nine_slice(NineSlice::TopRight),
                texture_assets,
            ))),
            visible: visible.clone(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .insert(aseprite_handle.clone())
        .insert(NinePatch {
            nine_patch: NineSlice::TopRight,
            slice_name: slice.name.clone(),
        });
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(from_top_left.x),
                    top: Val::Px(0.),
                    right: Val::Px(from_bottom_right.x),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Px(from_top_left.y),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material_assets.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &slice.label_with_nine_slice(NineSlice::TopCenter),
                texture_assets,
            ))),
            visible: visible.clone(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .insert(aseprite_handle.clone())
        .insert(NinePatch {
            nine_patch: NineSlice::TopCenter,
            slice_name: slice.name.clone(),
        });
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(from_top_left.x),
                    height: Val::Px(from_bottom_right.y),
                },
                ..Default::default()
            },
            material: material_assets.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &slice.label_with_nine_slice(NineSlice::BottomLeft),
                texture_assets,
            ))),
            visible: visible.clone(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .insert(aseprite_handle.clone())
        .insert(NinePatch {
            nine_patch: NineSlice::BottomLeft,
            slice_name: slice.name.clone(),
        });
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(0.),
                    right: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(from_bottom_right.x),
                    height: Val::Px(from_bottom_right.y),
                },
                ..Default::default()
            },
            material: material_assets.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &slice.label_with_nine_slice(NineSlice::BottomRight),
                texture_assets,
            ))),
            visible: visible.clone(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .insert(aseprite_handle.clone())
        .insert(NinePatch {
            nine_patch: NineSlice::BottomRight,
            slice_name: slice.name.clone(),
        });
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(from_top_left.x),
                    bottom: Val::Px(0.),
                    right: Val::Px(from_bottom_right.x),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Px(from_bottom_right.y),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material_assets.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &slice.label_with_nine_slice(NineSlice::BottomCenter),
                texture_assets,
            ))),
            visible: visible.clone(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .insert(aseprite_handle.clone())
        .insert(NinePatch {
            nine_patch: NineSlice::BottomCenter,
            slice_name: slice.name.clone(),
        });
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Px(0.),
                    bottom: Val::Px(from_bottom_right.y),
                    top: Val::Px(from_top_left.y),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(from_top_left.x),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material_assets.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &slice.label_with_nine_slice(NineSlice::LeftCenter),
                texture_assets,
            ))),
            visible: visible.clone(),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .insert(aseprite_handle.clone())
        .insert(NinePatch {
            nine_patch: NineSlice::LeftCenter,
            slice_name: slice.name.clone(),
        });
    commands
        .spawn_bundle(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    right: Val::Px(0.),
                    bottom: Val::Px(from_bottom_right.y),
                    top: Val::Px(from_top_left.y),
                    ..Default::default()
                },
                size: Size {
                    width: Val::Px(from_bottom_right.x),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: material_assets.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
                &slice.label_with_nine_slice(NineSlice::RightCenter),
                texture_assets,
            ))),
            ..Default::default()
        })
        .insert(FocusPolicy::Pass)
        .insert(aseprite_handle.clone())
        .insert(NinePatch {
            nine_patch: NineSlice::RightCenter,
            slice_name: slice.name.clone(),
        });

    let mut center = commands.spawn_bundle(ImageBundle {
        style: Style {
            // size: Size::new(Val::Auto, Val::Auto),
            margin: Rect {
                left: Val::Px(from_top_left.x),
                top: Val::Px(from_top_left.y),
                right: Val::Px(from_bottom_right.x),
                bottom: Val::Px(from_bottom_right.y),
                ..Default::default()
            },
            ..style.unwrap_or_default()
        },
        material: material_assets.add(ColorMaterial::texture(aseprite_handle.get_sub_handle(
            &slice.label_with_nine_slice(NineSlice::Center),
            texture_assets,
        ))),
        visible,
        ..Default::default()
    });

    center
        .insert(FocusPolicy::Pass)
        .insert(aseprite_handle.clone())
        .insert(NinePatch {
            nine_patch: NineSlice::Center,
            slice_name: slice.name.clone(),
        });

    center
}

pub struct NinePatchButton {
    pub normal: AsepriteSlice,
    pub hover: Option<AsepriteSlice>,
    pub pressed: Option<AsepriteSlice>,
}

fn update_nine_patch_button(
    button_query: Query<
        (&Interaction, &NinePatchButton, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut nine_patch_query: Query<&mut NinePatch>,
) {
    for (interaction, nine_patch_button, children) in button_query.iter() {
        match *interaction {
            Interaction::Clicked => children.iter().for_each(|entity| {
                if let Some(pressed) = nine_patch_button.pressed.as_ref() {
                    if let Ok(mut nine_patch) = nine_patch_query.get_mut(*entity) {
                        nine_patch.slice_name = (**pressed).to_owned();
                    }
                }
            }),
            Interaction::Hovered => children.iter().for_each(|entity| {
                if let Some(hover) = nine_patch_button.hover.as_ref() {
                    if let Ok(mut nine_patch) = nine_patch_query.get_mut(*entity) {
                        nine_patch.slice_name = (**hover).to_owned();
                    }
                }
            }),
            Interaction::None => children.iter().for_each(|entity| {
                if let Ok(mut nine_patch) = nine_patch_query.get_mut(*entity) {
                    nine_patch.slice_name = (*nine_patch_button.normal).to_owned();
                }
            }),
        }
    }
}

pub struct ButtonPressCommand<T: Component + Clone> {
    pub event: T,
}

impl<T: Component + Clone> ButtonPressCommand<T> {
    pub fn send_button_press(
        mut events: EventWriter<T>,
        button_query: Query<
            (&Interaction, &ButtonPressCommand<T>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, button_press_command) in button_query.iter() {
            match *interaction {
                Interaction::Clicked => {
                    events.send(button_press_command.event.clone());
                }
                _ => {}
            }
        }
    }
}
