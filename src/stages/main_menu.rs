use bevy::{app::AppExit, prelude::*};
use bevy_spicy_aseprite::{AsepriteImage, AsepriteSliceName};

use crate::{
    ui::{create_nine_patch, ButtonPressCommand, NinePatchButton},
    utils::GetSubHandle,
    world::DefaultLevels,
    GameAssets,
};

use super::GameState;

pub struct MainMenuStagePlugin;

impl Plugin for MainMenuStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MainMenuEvents>();
        app.add_system_set(SystemSet::on_enter(GameState::MainMenu).with_system(create_main_menu));
        app.add_system_set(
            SystemSet::on_update(GameState::MainMenu)
                .with_system(ButtonPressCommand::<MainMenuEvents>::send_button_press)
                .with_system(listen_for_menu_events),
        );
        app.add_system_set(SystemSet::on_exit(GameState::MainMenu).with_system(remove_main_menu));
    }
}

#[derive(Debug, Clone, Copy)]
enum MainMenuEvents {
    StartGame,
    Exit,
}

fn listen_for_menu_events(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    mut main_menu_events: EventReader<MainMenuEvents>,
    mut exit_events: EventWriter<AppExit>,
) {
    let last_event = main_menu_events.iter().last();

    match last_event {
        Some(&MainMenuEvents::Exit) => {
            exit_events.send(AppExit);
        }
        Some(&MainMenuEvents::StartGame) => {
            commands.insert_resource(DefaultLevels(vec![String::from("Level_0")]));
            state.set(GameState::Running).unwrap();
        }
        None => {}
    }
}

struct MainMenuScreenEntity(Entity);

fn create_main_menu(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_assets: ResMut<Assets<Texture>>,
    asprite_assets: Res<Assets<AsepriteImage>>,
) {
    let ui_aseprite = if let Some(ui_aseprite) = asprite_assets.get(&game_assets.ui_sprites) {
        ui_aseprite
    } else {
        return;
    };

    let slices = ui_aseprite.aseprite().slices();

    let menu_nine_slice =
        if let Some(menu_nine_slice) = slices.get_by_name(&crate::ui_sprites::slices::Menu) {
            menu_nine_slice
        } else {
            error!("Could not find menu slice");
            return;
        };

    let normal_button_nine_slice = if let Some(button_nine_slice) =
        slices.get_by_name(&crate::ui_sprites::slices::ButtonNormal)
    {
        button_nine_slice
    } else {
        error!("Could not find menu slice");
        return;
    };

    let title_text_style = TextStyle {
        font: game_assets.main_font.clone(),
        font_size: 8. * 5.,
        color: Color::BLACK,
    };

    let menu_text_style = TextStyle {
        font: game_assets.main_font.clone(),
        font_size: 8. * 3.,
        color: Color::BLACK,
    };

    let transparent_material = materials.add(Color::NONE.into());

    let loading_screen = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                flex_direction: FlexDirection::ColumnReverse,
                align_items: AlignItems::Center,
                align_content: AlignContent::FlexStart,
                ..Default::default()
            },
            material: transparent_material.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: Rect {
                            top: Val::Px(32.),
                            bottom: Val::Auto,
                            ..Default::default()
                        },
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: transparent_material.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Px(64.), Val::Px(64.)),
                            margin: Rect {
                                right: Val::Px(10.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        material: materials.add(ColorMaterial::texture(
                            game_assets.ui_sprites.get_sub_handle(
                                &crate::ui_sprites::slices::SpicyIcon.label(),
                                &mut texture_assets,
                            ),
                        )),
                        ..Default::default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section(
                            "Bevy Spicy Gamebase",
                            title_text_style,
                            TextAlignment {
                                vertical: VerticalAlign::Center,
                                horizontal: HorizontalAlign::Center,
                            },
                        ),
                        style: Style {
                            margin: Rect::all(Val::Px(5.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        margin: Rect {
                            bottom: Val::Px(32.),
                            ..Default::default()
                        },
                        align_content: AlignContent::Stretch,
                        align_items: AlignItems::Stretch,
                        justify_content: JustifyContent::Center,
                        ..Default::default()
                    },
                    material: transparent_material.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    create_nine_patch(
                        parent,
                        menu_nine_slice,
                        game_assets.ui_sprites.clone(),
                        &mut materials,
                        &mut texture_assets,
                        Some(Style {
                            // size: Size::new(Val::Percent(100.), Val::Auto),
                            flex_grow: 1.,
                            align_content: AlignContent::FlexStart,
                            align_items: AlignItems::Center,
                            flex_direction: FlexDirection::ColumnReverse,
                            ..Default::default()
                        }),
                    )
                    .with_children(|parent| {
                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    margin: Rect::all(Val::Px(25.)),
                                    ..Default::default()
                                },
                                material: transparent_material.clone(),
                                ..Default::default()
                            })
                            .insert(ButtonPressCommand {
                                event: MainMenuEvents::StartGame,
                            })
                            .insert(NinePatchButton {
                                normal: crate::ui_sprites::slices::ButtonNormal,
                                hover: Some(crate::ui_sprites::slices::ButtonHover),
                                pressed: Some(crate::ui_sprites::slices::ButtonPressed),
                            })
                            .with_children(|parent| {
                                create_nine_patch(
                                    parent,
                                    normal_button_nine_slice,
                                    game_assets.ui_sprites.clone(),
                                    &mut materials,
                                    &mut texture_assets,
                                    None,
                                )
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "New Game",
                                            menu_text_style.clone(),
                                            TextAlignment {
                                                vertical: VerticalAlign::Center,
                                                horizontal: HorizontalAlign::Center,
                                            },
                                        ),
                                        style: Style {
                                            margin: Rect::all(Val::Px(5.)),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });
                            });

                        parent
                            .spawn_bundle(ButtonBundle {
                                style: Style {
                                    margin: Rect {
                                        left: Val::Px(25.),
                                        right: Val::Px(25.),
                                        bottom: Val::Px(25.),
                                        top: Val::Auto,
                                    },
                                    ..Default::default()
                                },
                                material: transparent_material.clone(),
                                ..Default::default()
                            })
                            .insert(ButtonPressCommand {
                                event: MainMenuEvents::Exit,
                            })
                            .insert(NinePatchButton {
                                normal: crate::ui_sprites::slices::ButtonNormal,
                                hover: Some(crate::ui_sprites::slices::ButtonHover),
                                pressed: Some(crate::ui_sprites::slices::ButtonPressed),
                            })
                            .with_children(|parent| {
                                create_nine_patch(
                                    parent,
                                    normal_button_nine_slice,
                                    game_assets.ui_sprites.clone(),
                                    &mut materials,
                                    &mut texture_assets,
                                    None,
                                )
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Exit",
                                            menu_text_style,
                                            TextAlignment {
                                                vertical: VerticalAlign::Center,
                                                horizontal: HorizontalAlign::Center,
                                            },
                                        ),
                                        style: Style {
                                            margin: Rect::all(Val::Px(5.)),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    });
                                });
                            });
                    });
                });
        })
        .id();

    commands.insert_resource(MainMenuScreenEntity(loading_screen));
    debug!("Created main menu");
}

fn remove_main_menu(mut commands: Commands, ls_entity: Res<MainMenuScreenEntity>) {
    commands.entity(ls_entity.0).despawn_recursive();

    commands.remove_resource::<MainMenuScreenEntity>();
    debug!("Done with main menu!")
}
