use crate::mol::component::MyMolecule;
use bevy::prelude::*;

pub struct RotatorPlugin;

impl Plugin for RotatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                run_molecule_rotator,
                // run_camera_rotator
            ),
        );
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
