use bevy::prelude::*;
use bevy_loading::{track, Progress, ProgressCounter};

use super::GameState;

pub struct LoadingStagePlugin;

impl Plugin for LoadingStagePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(create_loading_progress),
        );
        app.add_system_set(
            SystemSet::on_update(GameState::Loading)
                .with_system(track(add_delay.system()))
                .with_system(update_load_progress),
        );
        app.add_system_set(
            SystemSet::on_exit(GameState::Loading).with_system(remove_load_progress),
        );
    }
}

struct LoadingScreenEntity(Entity);

struct LoadingBar;

fn create_loading_progress(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let loading_screen = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    left: Val::Percent((100. - 65.) / 2.0),
                    right: Val::Undefined,
                    top: Val::Percent(50. - 10. / 2.),
                    bottom: Val::Undefined,
                },
                size: Size {
                    width: Val::Percent(65.),
                    height: Val::Percent(10.),
                },
                padding: Rect::all(Val::Px(5.)),
                ..Default::default()
            },
            material: materials.add(Color::rgb_u8(72, 29, 76).into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Relative,
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(100.),
                        },
                        ..Default::default()
                    },
                    material: materials.add(Color::rgba(1., 1., 1., 0.05).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                position_type: PositionType::Relative,
                                size: Size {
                                    width: Val::Percent(23.),
                                    height: Val::Percent(100.),
                                },
                                ..Default::default()
                            },
                            material: materials.add(Color::rgb_u8(156, 42, 112).into()),
                            ..Default::default()
                        })
                        .insert(LoadingBar);
                });
        })
        .id();
    commands.insert_resource(LoadingScreenEntity(loading_screen));
    debug!("Created loading progress");
}

fn update_load_progress(
    counter: Res<ProgressCounter>,
    mut loading_bar_query: Query<&mut Style, With<LoadingBar>>,
) {
    for mut loading_bar_style in loading_bar_query.iter_mut() {
        let progress = counter.progress();
        loading_bar_style.size.width =
            Val::Percent(100. * progress.done as f32 / progress.total as f32);
    }
}

fn remove_load_progress(mut commands: Commands, ls_entity: Res<LoadingScreenEntity>) {
    commands.entity(ls_entity.0).despawn_recursive();

    commands.remove_resource::<LoadingScreenEntity>();
    debug!("Done loading!")
}

fn add_delay(time: Res<Time>) -> Progress {
    Progress {
        done: (time.seconds_since_startup() * 40.) as u32,
        total: 40,
    }
}
