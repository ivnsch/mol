//! A freecam-style camera controller plugin.
//! To use in your own application:
//! - Copy the code for the [`CameraControllerPlugin`] and add the plugin to your App.
//! - Attach the [`CameraController`] component to an entity with a [`Camera3dBundle`].

use bevy::prelude::*;

pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, run_camera_controller);
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
}

pub struct RotPars {
    pub initialized: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub sensitivity: f32,
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
