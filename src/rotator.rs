use crate::{
    camera_controller::{
        cursor_grab_update, update_cursor_and_window_for_grab_input, update_rotation_with_mouse,
        CursorGrabInput, CursorGrabStatus, RotPars,
    },
    mol::component::MyMolecule,
};
use bevy::{input::mouse::MouseMotion, prelude::*};

pub struct RotatorPlugin;

impl Plugin for RotatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                run_molecule_rotator,
                mouse_handler,
            ),
        );
    }
}

#[derive(Component)]
pub struct MolController {
    pub mouse_key_cursor_grab: MouseButton,
    pub rot_pars: RotPars,
}

impl Default for MolController {
    fn default() -> Self {
        Self {
            mouse_key_cursor_grab: MouseButton::Left,
            rot_pars: RotPars {
                initialized: false,
                pitch: 0.0,
                yaw: 0.0,
                sensitivity: 1.0,
            },
        }
    }
}

#[allow(unused)]
#[derive(Component, Debug)]
pub struct Rotator {
    pub key_y: KeyCode,
    pub key_z: KeyCode,
    pub key_x: KeyCode,
    pub key_i: KeyCode,
    pub key_o: KeyCode,
    pub key_p: KeyCode,
    pub key_shift_left: KeyCode,
    pub key_shift_right: KeyCode,
}

impl Default for Rotator {
    fn default() -> Self {
        Self {
            key_y: KeyCode::KeyY,
            key_z: KeyCode::KeyZ,
            key_x: KeyCode::KeyX,
            key_i: KeyCode::KeyI,
            key_o: KeyCode::KeyO,
            key_p: KeyCode::KeyP,
            key_shift_left: KeyCode::ShiftLeft,
            key_shift_right: KeyCode::ShiftRight,
        }
    }
}

fn run_molecule_rotator(
    key_input: Res<ButtonInput<KeyCode>>,
    mut sphere: Query<&mut Transform, With<MyMolecule>>,
    mut camera: Query<&mut Rotator>,
) {
    if let Ok(mut transform) = sphere.get_single_mut() {
        if let Ok(rotator) = camera.get_single_mut() {
            rotate(
                key_input,
                &rotator,
                &mut transform,
                rotator.key_x,
                rotator.key_y,
                rotator.key_z,
            );
        }
    };
}

#[allow(clippy::too_many_arguments, unused)]
fn run_camera_rotator(
    key_input: Res<ButtonInput<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut Rotator), With<Camera>>,
) {
    if let Ok((mut transform, mut rotator)) = camera.get_single_mut() {
        rotate(
            key_input,
            &rotator,
            &mut transform,
            rotator.key_x,
            rotator.key_y,
            rotator.key_z,
        );
    }
}

fn rotate(
    key_input: Res<ButtonInput<KeyCode>>,
    rotator: &Rotator,
    transform: &mut Transform,
    x_key: KeyCode,
    y_key: KeyCode,
    z_key: KeyCode,
) {
    let mut rotation = 0.03;
    if key_input.pressed(rotator.key_shift_left) || key_input.pressed(rotator.key_shift_right) {
        rotation = -rotation;
    }

    if key_input.pressed(y_key) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
        );
    }
    if key_input.pressed(z_key) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation),
        );
    }
    if key_input.pressed(x_key) {
        transform.rotate_around(
            Vec3::ZERO,
            Quat::from_euler(EulerRot::XYZ, rotation, 0.0, 0.0),
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn mouse_handler(
    mut windows: Query<&mut Window>,
    mouse_events: EventReader<MouseMotion>,
    mouse_cursor_grab: Local<bool>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mol: Query<(&mut Transform, &mut MolController), With<MyMolecule>>,
) {
    if let Ok((mut transform, mut controller)) = mol.get_single_mut() {
        if !controller.rot_pars.initialized {
            let (yaw, pitch, _roll) = transform.rotation.to_euler(EulerRot::YXZ);
            controller.rot_pars.yaw = yaw;
            controller.rot_pars.pitch = pitch;
            controller.rot_pars.initialized = true;
        }

        handle_mouse(
            &mut windows,
            mouse_events,
            mouse_button_input,
            mouse_cursor_grab,
            &mut transform,
            &mut controller,
        );
    }
}

fn handle_mouse(
    windows: &mut Query<&mut Window>,
    mut mouse_events: EventReader<MouseMotion>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mouse_cursor_grab: Local<bool>, // whether there's an active grab (user pressing mouse/trackpad)
    transform: &mut Transform,
    controller: &mut MolController,
) {
    // get current gesture (just started pressing or released)
    let grab = cursor_grab_update(mouse_button_input, controller.mouse_key_cursor_grab);

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
        update_cursor_and_window_for_grab_input(windows, &mut mouse_events, input);
    };

    // rotate mouse during active grab
    match &update_status {
        CursorGrabStatus::Active => {
            update_rotation_with_mouse(&mut mouse_events, transform, &mut controller.rot_pars)
        }
        CursorGrabStatus::Inactive => {}
    };
}
