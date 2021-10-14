use std::ops::RangeInclusive;

use bevy::{
    input::mouse::MouseWheel, math::Vec3Swizzles, prelude::*,
    render::camera::OrthographicProjection, window::WindowResized,
};

#[derive(Debug, Default)]
pub struct CameraPlugin;

#[derive(StageLabel, Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct CameraStage;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_stage_before(
            CoreStage::PostUpdate,
            CameraStage,
            SystemStage::single_threaded()
                .with_system(clamp_camera)
                .with_system(align_camera),
        );

        
        app.add_system(update_camera);
    }
}

#[derive(Debug)]
struct DragPosition {
    cursor_position: Vec2,
    camera_position: Vec2,
}

pub struct Free2DCamera {
    pub current_scale: f32,
    pub scale_levels: RangeInclusive<f32>,
    pub limits: Option<Rect<f32>>,
    start_drag: Option<DragPosition>,
}

impl Free2DCamera {
    pub fn new(current_scale: f32) -> Free2DCamera {
        Self {
            current_scale,
            scale_levels: current_scale..=current_scale,
            limits: None,
            start_drag: None,
        }
    }

    pub fn with_scale_range(mut self, scale_levels: RangeInclusive<f32>) -> Self {
        self.scale_levels = scale_levels;
        self
    }

    pub fn with_limits(mut self, limits: Option<Rect<f32>>) -> Self {
        self.limits = limits;
        self
    }
}

fn update_camera(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut cursor_movement_events: EventReader<CursorMoved>,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut camera_query: Query<(
        &mut Transform,
        &mut OrthographicProjection,
        &mut Free2DCamera,
    )>,
    mut last_cursor_position: Local<Vec2>,
) {
    // Without cameras, we don't care
    if camera_query.is_empty() {
        return;
    }

    let zoom_scroll: f32 = mouse_wheel_events.iter().map(|wheel| wheel.y).sum();

    let cursor_position = cursor_movement_events.iter().map(|mov| mov.position).last();

    if let Some(cursor_pos) = cursor_position {
        *last_cursor_position = cursor_pos;
    }

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
                .min(*free_2d_camera.scale_levels.end())
                .max(*free_2d_camera.scale_levels.start());

            let pos_change = ((screen_size / free_2d_camera.current_scale)
                - (screen_size / new_scale))
                * ((cursor_position - (screen_size / 2.)) / screen_size);

            transform.scale = Vec3::splat(1. / new_scale);
            transform.translation += pos_change.extend(0.);
            ortographic_project.far = (1000. * new_scale).floor();
            free_2d_camera.current_scale = new_scale;
        }

        if mouse_input.pressed(MouseButton::Right) {
            if free_2d_camera.start_drag.is_some() {
                let start_drag = free_2d_camera.start_drag.as_ref().unwrap();
                transform.translation = start_drag.camera_position.extend(transform.translation.z)
                    + (start_drag.cursor_position - *last_cursor_position).extend(0.)
                        / free_2d_camera.current_scale;
            } else {
                free_2d_camera.start_drag = Some(DragPosition {
                    camera_position: transform.translation.xy(),
                    cursor_position: *last_cursor_position,
                });
            }
        } else {
            free_2d_camera.start_drag = None;
        }

        if translation != Vec2::ZERO {
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
        // If any of the inputs changed, we re-calculate the
        if !(transform.is_changed() || change_free_2d_camera.is_changed() || resize_event) {
            continue;
        }

        if let Some(limit) = free_2d_camera.limits.as_ref() {
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
