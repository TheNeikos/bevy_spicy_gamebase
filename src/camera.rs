use std::ops::Range;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::OrthographicProjection,
};

#[derive(Debug, Default)]
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_camera);
    }
}

pub struct Free2DCamera {
    pub current_zoom: f32,
    pub zoom_levels: Range<u32>,
}

fn update_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_movement_events: EventReader<MouseMotion>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut camera_query: Query<(
        &mut Transform,
        &mut OrthographicProjection,
        &mut Free2DCamera,
    )>,
) {
    let zoom_scroll: f32 = mouse_wheel_events.iter().map(|wheel| wheel.y).sum();

    let mouse_movement: Vec2 = mouse_movement_events
        .iter()
        .map(|mov| mov.delta)
        .fold(Vec2::ZERO, |acc, elem| acc + elem);

    let translation = {
        let mut dir = Vec2::ZERO;

        if keyboard_input.pressed(KeyCode::Left) {
            dir += -Vec2::X;
        }
        if keyboard_input.pressed(KeyCode::Right) {
            dir += Vec2::X;
        }
        if keyboard_input.pressed(KeyCode::Up) {
            dir += Vec2::Y;
        }
        if keyboard_input.pressed(KeyCode::Down) {
            dir += -Vec2::Y;
        }

        dir.normalize_or_zero() * 10.
    };

    for (mut transform, mut ortographic_project, mut free_2d_camera) in camera_query.iter_mut() {
        if zoom_scroll != 0. {
            let primary_window = if let Some(win) = windows.get_primary() {
                win
            } else {
                error!("No primary window!");
                return;
            };

            let screen_size = Vec2::new(primary_window.width(), primary_window.height());

            let cursor_position = primary_window
                .cursor_position()
                .unwrap_or(screen_size / 2.0);

            let new_zoom = (free_2d_camera.current_zoom + zoom_scroll)
                .min(free_2d_camera.zoom_levels.end as f32)
                .max(free_2d_camera.zoom_levels.start as f32);

            let pos_change = ((screen_size / free_2d_camera.current_zoom)
                - (screen_size / new_zoom))
                * ((cursor_position - (screen_size / 2.)) / screen_size);

            transform.scale = Vec3::splat(1. / new_zoom);
            transform.translation += pos_change.extend(0.);
            ortographic_project.far = 1000. * new_zoom;
            free_2d_camera.current_zoom = new_zoom;
        }

        if mouse_movement != Vec2::ZERO && mouse_input.pressed(MouseButton::Right) {
            transform.translation +=
                (mouse_movement * Vec2::new(-1., 1.)).extend(0.) / free_2d_camera.current_zoom;
        } else if translation != Vec2::ZERO {
            transform.translation += translation.extend(0.);
        }
    }
}
