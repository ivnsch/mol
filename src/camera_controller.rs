//! A freecam-style camera controller plugin.
//! To use in your own application:
//! - Copy the code for the [`CameraControllerPlugin`] and add the plugin to your App.
//! - Attach the [`CameraController`] component to an entity with a [`Camera3dBundle`].

use crate::rotator::{
    cursor_grab_update, update_cursor_and_window_for_grab_input, CursorGrabInput, CursorGrabStatus,
};
use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                run_camera_controller,
                // if testing this, disable the mouse_handler mouse_handler as both use the same mouse event
                // mouse_handler
            ),
        );
    }
}

#[derive(Component)]
pub struct CameraController {
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub velocity: Vec3,
    mouse_key_cursor_zoom: MouseButton,
    zoom_sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            key_forward: KeyCode::KeyW,
            key_back: KeyCode::KeyS,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyE,
            key_down: KeyCode::KeyQ,
            key_run: KeyCode::ShiftLeft,
            walk_speed: 10.0,
            run_speed: 15.0,
            friction: 0.5,
            velocity: Vec3::ZERO,
            mouse_key_cursor_zoom: MouseButton::Left,
            zoom_sensitivity: 0.3,
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn run_camera_controller(
    time: Res<Time>,
    key_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    if let Ok((mut transform, mut controller)) = query.get_single_mut() {
        handle_keyboard(time, &key_input, &mut transform, &mut controller);
    }
}

fn handle_keyboard(
    time: Res<Time>,
    key_input: &Res<ButtonInput<KeyCode>>,
    transform: &mut Transform,
    controller: &mut CameraController,
) {
    let axes_input = input_as_axes(controller, key_input);
    update_controller_velocity(axes_input, controller, key_input);

    update_translation_with_velocity(time, transform, controller.velocity);
}

/// maps the entered keys to values on x/y/z
/// currently this value is always 1 so this is equivalent to setting a flag ("axis is active")
fn input_as_axes(controller: &mut CameraController, key_input: &Res<ButtonInput<KeyCode>>) -> Vec3 {
    let mut axis_input = Vec3::ZERO;
    if key_input.pressed(controller.key_forward) {
        axis_input.z += 1.0;
    }
    if key_input.pressed(controller.key_back) {
        axis_input.z -= 1.0;
    }
    if key_input.pressed(controller.key_right) {
        axis_input.x += 1.0;
    }
    if key_input.pressed(controller.key_left) {
        axis_input.x -= 1.0;
    }
    if key_input.pressed(controller.key_up) {
        axis_input.y += 1.0;
    }
    if key_input.pressed(controller.key_down) {
        axis_input.y -= 1.0;
    }
    axis_input
}

/// updates velocity, based on pressed keys
fn update_controller_velocity(
    axis_input: Vec3,
    controller: &mut CameraController,
    key_input: &Res<ButtonInput<KeyCode>>,
) {
    if axis_input != Vec3::ZERO {
        controller.velocity = to_velocity(&controller, key_input, axis_input);
    } else {
        // nothing pressed: start slow down
        let friction = controller.friction.clamp(0.0, 1.0);
        controller.velocity *= 1.0 - friction;
        // set back to 0 when close enough
        if controller.velocity.length_squared() < 1e-6 {
            controller.velocity = Vec3::ZERO;
        }
    }
}

/// uses velocity to update the transform's translation
fn update_translation_with_velocity(time: Res<Time>, transform: &mut Transform, velocity: Vec3) {
    let dt = time.delta_seconds();

    let forward = *transform.forward();
    let right = *transform.right();
    transform.translation +=
        velocity.x * dt * right + velocity.y * dt * Vec3::Y + velocity.z * dt * forward;
}

/// maps the keys pressed to a velocity vector
fn to_velocity(
    controller: &CameraController,
    key_input: &Res<ButtonInput<KeyCode>>,
    axis_input: Vec3,
) -> Vec3 {
    let max_speed = if key_input.pressed(controller.key_run) {
        controller.run_speed
    } else {
        controller.walk_speed
    };
    axis_input.normalize() * max_speed
}

/// TODO re-implement using MouseWheel
#[allow(dead_code)]
fn mouse_handler(
    mut windows: Query<&mut Window>,
    mut mouse_events: EventReader<MouseMotion>,
    mut mouse_cursor_grab: Local<bool>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut camera: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    if let Ok((mut transform, controller)) = camera.get_single_mut() {
        // get current gesture (just started pressing or released)
        let grab = cursor_grab_update(mouse_button_input, controller.mouse_key_cursor_zoom);

        // determine whether currently there's an active grab
        let update_status = match &grab {
            // just did something: map to status and save
            Some(grab) => {
                let status = match grab {
                    CursorGrabInput::JustPressed => CursorGrabStatus::Active,
                    CursorGrabInput::JustReleased => CursorGrabStatus::Inactive,
                };
                // save current state (no-op if user just didn't do anything)
                *mouse_cursor_grab = match &status {
                    CursorGrabStatus::Active => true,
                    CursorGrabStatus::Inactive => false,
                };
                status
            }
            // didn't do anything: use current state
            None => match *mouse_cursor_grab {
                true => CursorGrabStatus::Active,
                false => CursorGrabStatus::Inactive,
            },
        };

        // if there was a gesture, do cursor and window updates
        if let Some(input) = &grab {
            update_cursor_and_window_for_grab_input(&mut windows, &mut mouse_events, input);
        };

        // zoom during active grab
        match &update_status {
            CursorGrabStatus::Active => update_zoom_with_mouse(
                &mut mouse_events,
                &mut transform,
                controller.zoom_sensitivity,
            ),
            CursorGrabStatus::Inactive => {}
        };
    }
}

fn update_zoom_with_mouse(
    mouse_events: &mut EventReader<MouseMotion>,
    transform: &mut Transform,
    sensitivity: f32,
) {
    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.read() {
        mouse_delta += mouse_event.delta;
        if mouse_delta != Vec2::ZERO {
            transform.translation.z -= mouse_event.delta.x + mouse_event.delta.y * sensitivity;
        }
    }
}
