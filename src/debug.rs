use bevy::{
    color::palettes::css::{BLUE, GREEN, RED},
    prelude::*,
};

use crate::mol::MyMolecule;

const AXIS_LEN: f32 = 3.0;

#[allow(dead_code)]
pub fn add_debug(app: &mut App) {
    app.add_systems(Startup, setup_cube)
        .add_systems(Update, setup_global_axes);
}

fn setup_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: GREEN.into(),
        ..default()
    });

    let mesh_handle = meshes.add(Cuboid { ..default() }.mesh());


    let scale = 1.0;
    let cube = (
        PbrBundle {
            mesh: mesh_handle,
            material: debug_material.clone(),
            transform: Transform::from_translation(Vec3::ZERO)
                .with_scale(Vec3::new(scale, scale, scale)),
            ..default()
        },
        MyMolecule,
    );

    commands.spawn(cube);
}

#[allow(dead_code)]
fn setup_global_axes(mut gizmos: Gizmos) {
    let zero = 0.0;
    // x
    gizmos.line(
        Vec3 {
            x: -AXIS_LEN,
            y: zero,
            z: zero,
        },
        Vec3 {
            x: AXIS_LEN,
            y: zero,
            z: zero,
        },
        GREEN,
    );
    // y
    gizmos.line(
        Vec3 {
            x: zero,
            y: -AXIS_LEN,
            z: zero,
        },
        Vec3 {
            x: zero,
            y: AXIS_LEN,
            z: zero,
        },
        RED,
    );
    // z
    gizmos.line(
        Vec3 {
            x: zero,
            y: zero,
            z: -AXIS_LEN,
        },
        Vec3 {
            x: zero,
            y: zero,
            z: AXIS_LEN,
        },
        BLUE,
    );
}
