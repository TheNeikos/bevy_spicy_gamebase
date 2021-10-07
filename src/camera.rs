use std::ops::Range;

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    math::Vec3Swizzles,
    prelude::*,
    render::camera::OrthographicProjection,
    window::WindowResized,
};

#[derive(Debug, Default)]
pub struct CameraPlugin;

#[derive(StageLabel, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct CameraStage;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_after(
            CoreStage::Update,
            CameraStage,
            SystemStage::single_threaded()
                .with_system(clamp_camera)
                .with_system(align_camera),
        );

        app.add_system(update_camera);
    }
}

pub struct Free2DCamera {
    pub current_scale: f32,
    pub scale_levels: Range<u32>,
    pub limit: Option<Rect<f32>>,
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

    let primary_window = if let Some(win) = windows.get_primary() {
        win
    } else {
        error!("No primary window!");
        return;
    };

    for (mut transform, mut ortographic_project, mut free_2d_camera) in camera_query.iter_mut() {
        if zoom_scroll != 0. {
            let screen_size = Vec2::new(primary_window.width(), primary_window.height());

            let cursor_position = primary_window
                .cursor_position()
                .unwrap_or(screen_size / 2.0);

            let new_scale = (free_2d_camera.current_scale + zoom_scroll)
                .min(free_2d_camera.scale_levels.end as f32)
                .max(free_2d_camera.scale_levels.start as f32);

            let pos_change = ((screen_size / free_2d_camera.current_scale)
                - (screen_size / new_scale))
                * ((cursor_position - (screen_size / 2.)) / screen_size);

            transform.scale = Vec3::splat(1. / new_scale);
            transform.translation += pos_change.extend(0.);
            ortographic_project.far = (1000. * new_scale).floor();
            free_2d_camera.current_scale = new_scale;
        }

        if mouse_movement != Vec2::ZERO && mouse_input.pressed(MouseButton::Right) {
            transform.translation +=
                (mouse_movement * Vec2::new(-1., 1.)).extend(0.) / free_2d_camera.current_scale;
        } else if translation != Vec2::ZERO {
            transform.translation += translation.extend(0.);
        }
    }
}

fn clamp_camera(
    windows: Res<Windows>,
    mut window_resize_events: EventReader<WindowResized>,
    mut camera_query: Query<(&mut Transform, &Free2DCamera, ChangeTrackers<Free2DCamera>)>,
) {
    let primary_window = if let Some(win) = windows.get_primary() {
        win
    } else {
        error!("No primary window!");
        return;
    };

    let resize_event = window_resize_events.iter().last().is_some();

    let screen_size = Vec2::new(primary_window.width(), primary_window.height());

    for (mut transform, free_2d_camera, change_free_2d_camera) in camera_query.iter_mut() {
        if !(transform.is_changed() || change_free_2d_camera.is_changed() || resize_event) {
            continue;
        }

        if let Some(limit) = free_2d_camera.limit.as_ref() {
            let half_scaled_screen_size = screen_size / 2. / free_2d_camera.current_scale;

            let left_right_pixels = transform.translation.xx()
                + Vec2::splat(half_scaled_screen_size.x) * Vec2::new(-1., 1.);
            let top_bottom_pixels = transform.translation.yy()
                + Vec2::splat(half_scaled_screen_size.y) * Vec2::new(-1., 1.);

            let left_right_limit = Vec2::new(limit.left, limit.right);
            let top_bottom_limit = Vec2::new(limit.bottom, limit.top);

            let left_right_mask =
                ((left_right_pixels - left_right_limit) * Vec2::new(1., -1.)).cmplt(Vec2::ZERO);
            let top_bottom_mask =
                ((top_bottom_pixels - top_bottom_limit) * Vec2::new(1., -1.)).cmplt(Vec2::ZERO);

            if !left_right_mask.any() && !top_bottom_mask.any() {
                // All checks are in bounds
                continue;
            }

            let mut new_center = transform.translation.xy();

            new_center.x = if left_right_mask.all()
                || (left_right_pixels.y - left_right_pixels.x)
                    >= (left_right_limit.y - left_right_limit.x)
            {
                // If both are over the limit, we set it to the center between the two limits
                (left_right_limit.x + left_right_limit.y) / 2.
            } else if left_right_mask.any() {
                // If one is over the limit, we move it in the relevant direction
                let left_right_diff = left_right_pixels - left_right_limit;
                let left_right_relevant =
                    Vec2::select(left_right_mask, left_right_diff, Vec2::ZERO);

                new_center.x - (left_right_relevant.x + left_right_relevant.y)
            } else {
                new_center.x
            };

            new_center.y = if top_bottom_mask.all()
                || (top_bottom_pixels.y - top_bottom_pixels.x)
                    >= (top_bottom_limit.y - top_bottom_limit.x)
            {
                // If both are over the limit, we set it to the center between the two limits
                (top_bottom_limit.x + top_bottom_limit.y) / 2.
            } else if top_bottom_mask.any() {
                // If one is over the limit, we move it in the relevant direction
                let top_bottom_diff = top_bottom_pixels - top_bottom_limit;
                let top_bottom_relevant =
                    Vec2::select(top_bottom_mask, top_bottom_diff, Vec2::ZERO);

                new_center.y - (top_bottom_relevant.x + top_bottom_relevant.y)
            } else {
                new_center.y
            };

            transform.translation = new_center.extend(transform.translation.z);
        }
    }
}

fn align_camera(mut camera_query: Query<&mut Transform, (Changed<Transform>, With<Free2DCamera>)>) {
    for mut transform in camera_query.iter_mut() {
        if transform.translation != transform.translation.floor() {
            transform.translation = transform.translation.floor();
        }
    }
}
