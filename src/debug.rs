use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, YELLOW},
    prelude::*,
};

use crate::mol::MyMolecule;

const AXIS_LEN: f32 = 3.0;

#[allow(dead_code)]
pub fn add_debug(app: &mut App) {
    app.add_systems(Startup, setup_cube)
        .add_systems(Update, setup_global_axes);
}

#[allow(dead_code)]
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

    // // print vertices
    // if let Some(mesh) = meshes.get(mesh_handle.id()) {
    //     println!("mesh: {:?}", mesh);
    // }

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

    add_dot(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-0.5, -0.5, -0.5),
    );

    add_dot(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(-0.5, -0.5, 0.5),
    );
#[allow(dead_code)]
fn add_dot(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    pos: Vec3,
) {
    let debug_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: YELLOW.into(),
        ..default()
    });

    let mesh_handle = meshes.add(Sphere { ..default() }.mesh());

    // // print vertices
    // if let Some(mesh) = meshes.get(mesh_handle.id()) {
    //     println!("dot mesh: {:?}", mesh);
    // }

    let scale = 0.1;
    let cube = (PbrBundle {
        mesh: mesh_handle,
        material: debug_material.clone(),
        transform: Transform::from_translation(pos).with_scale(Vec3::new(scale, scale, scale)),
        ..default()
    },);

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
