use bevy::prelude::*;
use std::fmt;

use crate::mol::component::MyMolecule;

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

impl fmt::Display for Rotator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "
Rotator Controls:
    {:?} \t- rotate around x
    {:?} \t- rotate around y
    {:?} \t- rotate around z",
            self.key_x, self.key_y, self.key_z,
        )
    }
}

fn run_molecule_rotator(
    key_input: Res<ButtonInput<KeyCode>>,
    mut sphere: Query<&mut Transform, With<MyMolecule>>,
    mut camera: Query<&mut Rotator>,
) {
    if let Ok(mut sphere_transform) = sphere.get_single_mut() {
        let camera = camera.get_single_mut();
        if let Ok(rotator) = camera {
            let mut rotation = 0.03;
            if key_input.pressed(rotator.key_shift_left)
                || key_input.pressed(rotator.key_shift_right)
            {
                rotation = -rotation;
            }

            if key_input.pressed(rotator.key_y) {
                sphere_transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
                );
            }
            if key_input.pressed(rotator.key_z) {
                sphere_transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation),
                );
            }
            if key_input.pressed(rotator.key_x) {
                sphere_transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_euler(EulerRot::XYZ, rotation, 0.0, 0.0),
                );
            }
        }
    };
}

#[allow(clippy::too_many_arguments, unused)]
fn run_camera_rotator(
    key_input: Res<ButtonInput<KeyCode>>,
    mut camera: Query<(&mut Transform, &mut Rotator), With<Camera>>,
) {
    if let Ok((mut transform, mut controller)) = camera.get_single_mut() {
        let mut rotation = 0.03;

        if key_input.pressed(controller.key_shift_left)
            || key_input.pressed(controller.key_shift_right)
        {
            rotation = -rotation;
        }

        if key_input.pressed(controller.key_i) {
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
            );
        }
        if key_input.pressed(controller.key_o) {
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation),
            );
        }
        if key_input.pressed(controller.key_p) {
            transform.rotate_around(
                Vec3::ZERO,
                Quat::from_euler(EulerRot::XYZ, rotation, 0.0, 0.0),
            );
        }
    }
}
