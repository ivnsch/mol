use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_3d_scratch(app: &mut App) {
    app.add_systems(Startup, (setup_sphere));
}

#[derive(Component, Default)]
pub struct MySphere;

fn setup_sphere(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let debug_material = materials.add(StandardMaterial { ..default() });

    let sphere = meshes.add(Sphere::default().mesh().uv(32, 18));
    let cuboid = meshes.add(Cuboid::default());

    let line_scale = 2.0;
    let line_thickness = 0.01;

    commands
        .spawn((Name::new("group"), MySphere, SpatialBundle { ..default() }))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: cuboid.clone(),
                    material: debug_material.clone(),
                    transform: Transform {
                        translation: Vec3::ZERO,
                        rotation: Quat::IDENTITY,
                        scale: Vec3 {
                            x: line_scale,
                            y: line_thickness,
                            z: line_thickness,
                        },
                    },
                    ..default()
                },
                Shape,
            ));
            parent.spawn((
                PbrBundle {
                    mesh: cuboid.clone(),
                    material: debug_material.clone(),
                    transform: Transform {
                        translation: Vec3::ZERO,
                        rotation: Quat::IDENTITY,
                        scale: Vec3 {
                            x: line_thickness,
                            y: line_scale,
                            z: line_thickness,
                        },
                    },
                    ..default()
                },
                Shape,
            ));
            parent.spawn((
                PbrBundle {
                    mesh: cuboid,
                    material: debug_material.clone(),
                    transform: Transform {
                        translation: Vec3::ZERO,
                        rotation: Quat::IDENTITY,
                        scale: Vec3 {
                            x: line_thickness,
                            y: line_thickness,
                            z: line_scale,
                        },
                    },
                    ..default()
                },
                Shape,
            ));
            parent.spawn((
                PbrBundle {
                    mesh: sphere,
                    material: debug_material.clone(),
                    transform: Transform::from_xyz(0.0, 0.0, 0.0),
                    ..default()
                },
                Shape,
            ));
        });
}

/// A marker component for our shapes so we can query them separately from other things
#[derive(Component)]
struct Shape;
