use crate::scene::component::MyMoleculeWrapper;
use bevy::{input::mouse::MouseMotion, prelude::*};
use sim_controls::rotator::{handle_mouse, rotate, MouseController, Rotator};

/// Based on Valorant's default sensitivity, not entirely sure why it is exactly 1.0 / 180.0,
/// but I'm guessing it is a misunderstanding between degrees/radians and then sticking with
/// it because it felt nice.
#[allow(unused)]
pub const RADIANS_PER_DOT: f32 = 1.0 / 180.0;

pub struct RotatorPlugin;

impl Plugin for RotatorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (run_molecule_rotator, mouse_handler));
    }
}

fn run_molecule_rotator(
    key_input: Res<ButtonInput<KeyCode>>,
    mut sphere: Query<&mut Transform, With<MyMoleculeWrapper>>,
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

#[allow(unused)]
#[allow(clippy::too_many_arguments)]
fn mouse_handler(
    mut windows: Query<&mut Window>,
    mouse_events: EventReader<MouseMotion>,
    mouse_cursor_grab: Local<bool>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut mol: Query<(&mut Transform, &mut MouseController), With<MyMoleculeWrapper>>,
) {
    if let Ok((mut transform, mut controller)) = mol.get_single_mut() {
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
