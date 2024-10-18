use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{BLACK, DARK_GRAY, WHITE},
    prelude::*,
};

#[allow(dead_code)]
pub fn add_3d_scratch(app: &mut App) {
    app.add_systems(Startup, setup_molecule)
        .add_systems(PostStartup, setup_atoms);
}

#[derive(Component, Default)]
pub struct MySphere;

#[derive(Component, Default)]
pub struct MyMolecule;

fn add_bond(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    molecule: Entity,
    atom1_loc: Vec3,
    atom2_loc: Vec3,
) {
    let material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: DARK_GRAY.into(),
        ..default()
    });

    let bond = create_bond(meshes, &material, atom1_loc, atom2_loc);

    let entity = commands.spawn(bond).id();
    commands.entity(molecule).push_children(&[entity]);
}

fn create_bond(
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    p1: Vec3,
    p2: Vec3,
) -> PbrBundle {
    let cuboid: Handle<Mesh> = meshes.add(Cuboid::default());

    let line_thickness = 0.01;

    let midpoint = (p1 + p2) / 2.0;

    let distance = p1.distance(p2);
    let direction = (p2 - p1).normalize();
    let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

    PbrBundle {
        mesh: cuboid.clone(),
        material: material.clone(),
        transform: Transform {
            translation: midpoint,
            rotation,
            scale: Vec3::new(line_thickness, distance, line_thickness),
        },
        ..default()
    }
}

fn add_atom(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    molecule: Entity,
    position: Vec3,
    color: Color,
) {
    let debug_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: color,
        ..default()
    });

    let mesh = meshes.add(Sphere { ..default() }.mesh().uv(32, 18));

    let scale = 0.4;

    let sphere = (
        PbrBundle {
            mesh,
            material: debug_material.clone(),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::new(scale, scale, scale)),
            ..default()
        },
        Shape,
    );

    let entity = commands.spawn(sphere).id();
    commands.entity(molecule).push_children(&[entity]);
    println!("pushed the shere to molecule..");
}

fn setup_molecule(mut commands: Commands) {
    commands.spawn((
        Name::new("group"),
        MyMolecule,
        SpatialBundle { ..default() },
    ));
}

fn setup_atoms(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut molecule: Query<Entity, With<MyMolecule>>,
) {
    // add_carbon(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     &mut molecule,
    //     Vec3::ZERO,
    // );

    // add_propane(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     &mut molecule,
    //     Vec3::ZERO,
    // );

    add_butane(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut molecule,
        Vec3::ZERO,
    );
}

fn add_carbon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    molecule: &mut Query<Entity, With<MyMolecule>>,
    center: Vec3,
) {
    if let Ok(molecule) = molecule.get_single_mut() {
        // center carbon
        add_atom(commands, meshes, materials, molecule, center, BLACK.into());

        let length = 1.0;

        // tetrahedral angle
        // note that this is used for the angles with the center of the molecule as vertex,
        // the angle between the molecules forming a circle has to be 120° (360° / 3 molecules)
        let bond_angle = 109.5_f32.to_radians();

        // first h up on y axis
        let mut p1 = Vec3::new(0.0, length, 0.0);

        let rot_x = Quat::from_rotation_x(bond_angle);
        let rot_y_angle = 120.0_f32.to_radians();
        let rot_y = Quat::from_rotation_y(rot_y_angle);

        // second h "back-right"
        let mut p2 = (rot_y * rot_x * Vec3::Y) * length;

        // third h "back-left"
        let rot_y_neg = Quat::from_rotation_y(-rot_y_angle);
        let mut p3 = (rot_y_neg * rot_x * Vec3::Y) * length;

        // fourth h "front"
        let mut p4 = rot_x * Vec3::Y * length;

        p1 = p1 + center;
        p2 = p2 + center;
        p3 = p3 + center;
        p4 = p4 + center;

        add_atom(commands, meshes, materials, molecule, p1, WHITE.into());

        add_atom(commands, meshes, materials, molecule, p2, WHITE.into());

        add_atom(commands, meshes, materials, molecule, p3, WHITE.into());

        add_atom(commands, meshes, materials, molecule, p4, WHITE.into());

        // add bonds connecting atoms

        add_bond(commands, meshes, materials, molecule, center, p1);
        add_bond(commands, meshes, materials, molecule, center, p2);
        add_bond(commands, meshes, materials, molecule, center, p3);
        add_bond(commands, meshes, materials, molecule, center, p4);
    } else {
        println!("couldn't get molecule entity");
    }
}

fn add_ethane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    molecule: &mut Query<Entity, With<MyMolecule>>,
    center: Vec3,
) {
    // add wrapper entities to transform as a group
    if let Ok(molecule) = molecule.get_single_mut() {
        let parent1 = commands
            .spawn((
                Name::new("parent1"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_rotation_z(-90.0_f32.to_radians()),
                        translation: Vec3::new(0.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..default()
                },
            ))
            .id();

        let parent2 = commands
            .spawn((
                Name::new("parent2"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_rotation_z(90.0_f32.to_radians()),
                        translation: Vec3::new(1.0, 0.0, 0.0),
                        ..Default::default()
                    },
                    ..default()
                },
            ))
            .id();

        commands.entity(molecule).push_children(&[parent1, parent2]);

        add_outer_carbon(commands, meshes, materials, parent1, center);
        add_outer_carbon(commands, meshes, materials, parent2, center);

        // parent
    } else {
        println!("couldn't get molecule entity");
    }
}

fn add_propane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    molecule: &mut Query<Entity, With<MyMolecule>>,
    center: Vec3,
) {
    // add wrapper entities to transform as a group
    let parent1_trans = Vec3::new(0.0, 0.0, 0.0);
    if let Ok(molecule) = molecule.get_single_mut() {
        let parent1 = commands
            .spawn((
                Name::new("parent1"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_rotation_z(-45.0_f32.to_radians()),
                        translation: parent1_trans,
                        ..Default::default()
                    },
                    ..default()
                },
            ))
            .id();

        let parent2_trans = Vec3::new(1.0, 1.0, 0.0);
        let parent2 = commands
            .spawn((
                Name::new("parent2"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_euler(EulerRot::XYZ, PI, -PI / 4.0, 0.0),
                        translation: parent2_trans,
                        ..Default::default()
                    },
                    ..default()
                },
            ))
            .id();

        let parent3_trans = Vec3::new(2.0, 0.0, 0.0);
        let parent3 = commands
            .spawn((
                Name::new("parent3"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_rotation_z(45.0_f32.to_radians()),
                        translation: parent3_trans,
                        ..Default::default()
                    },
                    ..default()
                },
            ))
            .id();

        commands.entity(molecule).push_children(&[parent1, parent2]);

        add_outer_carbon(commands, meshes, materials, parent1, center);
        // inter part bonds here since we don't know the distance to the next part inside of a part
        // part being an outer carbon or inner carbon (with respective hydrogens)
        add_bond(
            commands,
            meshes,
            materials,
            molecule,
            parent1_trans,
            parent2_trans,
        );
        add_inner_carbon(commands, meshes, materials, parent2, center);
        add_bond(
            commands,
            meshes,
            materials,
            molecule,
            parent3_trans,
            parent2_trans,
        );
        add_outer_carbon(commands, meshes, materials, parent3, center);

        // parent
    } else {
        println!("couldn't get molecule entity");
    }
}

fn add_butane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    molecule: &mut Query<Entity, With<MyMolecule>>,
    center: Vec3,
) {
    // add wrapper entities to transform as a group
    let parent1_trans = Vec3::new(0.0, 0.0, 0.0);
    if let Ok(molecule) = molecule.get_single_mut() {
        let parent1 = commands
            .spawn((
                Name::new("parent1"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_rotation_z(-45.0_f32.to_radians()),
                        translation: parent1_trans,
                        ..Default::default()
                    },
                    ..default()
                },
            ))
            .id();

        let parent2_trans = Vec3::new(1.0, 1.0, 0.0);
        let parent2 = commands
            .spawn((
                Name::new("parent2"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_euler(EulerRot::XYZ, PI, -PI / 4.0, 0.0),
                        translation: parent2_trans,
                        ..Default::default()
                    },
                    ..default()
                },
            ))
            .id();

        let parent3_trans = Vec3::new(2.0, 0.0, 0.0);
        let parent3 = commands
            .spawn((
                Name::new("parent2"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_euler(EulerRot::XYZ, 0.0, 135.0_f32.to_radians(), 0.0),
                        translation: parent3_trans,
                        ..Default::default()
                    },
                    ..default()
                },
            ))
            .id();

        let parent4_trans = Vec3::new(3.0, 1.0, 0.0);
        let parent4 = commands
            .spawn((
                Name::new("parent3"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_rotation_z(135.0_f32.to_radians()),
                        translation: parent4_trans,
                        ..Default::default()
                    },
                    ..default()
                },
            ))
            .id();

        commands.entity(molecule).push_children(&[parent1, parent2]);

        add_outer_carbon(commands, meshes, materials, parent1, center);
        // inter part bonds here since we don't know the distance to the next part inside of a part
        // part being an outer carbon or inner carbon (with respective hydrogens)
        add_bond(
            commands,
            meshes,
            materials,
            molecule,
            parent1_trans,
            parent2_trans,
        );
        add_inner_carbon(commands, meshes, materials, parent2, center);
        add_bond(
            commands,
            meshes,
            materials,
            molecule,
            parent3_trans,
            parent2_trans,
        );
        add_inner_carbon(commands, meshes, materials, parent3, center);
        add_bond(
            commands,
            meshes,
            materials,
            molecule,
            parent4_trans,
            parent3_trans,
        );
        add_outer_carbon(commands, meshes, materials, parent4, center);

        // parent
    } else {
        println!("couldn't get molecule entity");
    }
}

// the same as add_carbon, without one of the H
// TODO refactor
fn add_outer_carbon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // molecule: &mut Query<Entity, With<MyMolecule>>,
    parent: Entity,
    center: Vec3,
) {
    // center carbon
    add_atom(commands, meshes, materials, parent, center, BLACK.into());

    let length = 1.0;

    // tetrahedral angle
    // note that this is used for the angles with the center of the molecule as vertex,
    // the angle between the molecules forming a circle has to be 120° (360° / 3 molecules)
    let bond_angle = 109.5_f32.to_radians();

    let rot_x = Quat::from_rotation_x(bond_angle);
    let rot_y_angle = 120.0_f32.to_radians();
    let rot_y = Quat::from_rotation_y(rot_y_angle);

    // first h up on y axis
    let mut p1 = Vec3::new(0.0, length, 0.0);

    // second h "back-right"
    let mut p2 = (rot_y * rot_x * Vec3::Y) * length;

    // third h "back-left"
    let rot_y_neg = Quat::from_rotation_y(-rot_y_angle);
    let mut p3 = (rot_y_neg * rot_x * Vec3::Y) * length;

    // fourth h "front"
    let mut p4 = rot_x * Vec3::Y * length;

    p1 = p1 + center;
    p2 = p2 + center;
    p3 = p3 + center;
    p4 = p4 + center;

    add_atom(commands, meshes, materials, parent, p2, WHITE.into());

    add_atom(commands, meshes, materials, parent, p3, WHITE.into());

    add_atom(commands, meshes, materials, parent, p4, WHITE.into());

    // add bonds connecting atoms

    add_bond(commands, meshes, materials, parent, center, p2);
    add_bond(commands, meshes, materials, parent, center, p3);
    add_bond(commands, meshes, materials, parent, center, p4);
}

// the same as add_carbon, without two of the H
// TODO refactor
fn add_inner_carbon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    // molecule: &mut Query<Entity, With<MyMolecule>>,
    parent: Entity,
    center: Vec3,
) {
    // center carbon
    add_atom(commands, meshes, materials, parent, center, BLACK.into());

    let length = 1.0;

    // tetrahedral angle
    // note that this is used for the angles with the center of the molecule as vertex,
    // the angle between the molecules forming a circle has to be 120° (360° / 3 molecules)
    let bond_angle = 109.5_f32.to_radians();

    let rot_x = Quat::from_rotation_x(bond_angle);
    let rot_y_angle = 120.0_f32.to_radians();
    let rot_y = Quat::from_rotation_y(rot_y_angle);

    // first h up on y axis
    let mut p1 = Vec3::new(0.0, length, 0.0);

    // second h "back-right"
    let mut p2 = (rot_y * rot_x * Vec3::Y) * length;

    // third h "back-left"
    let rot_y_neg = Quat::from_rotation_y(-rot_y_angle);
    let mut p3 = (rot_y_neg * rot_x * Vec3::Y) * length;

    // fourth h "front"
    let mut p4 = rot_x * Vec3::Y * length;

    p1 = p1 + center;
    p2 = p2 + center;
    p3 = p3 + center;
    p4 = p4 + center;

    add_atom(commands, meshes, materials, parent, p2, WHITE.into());
    add_atom(commands, meshes, materials, parent, p3, WHITE.into());
    add_bond(commands, meshes, materials, parent, center, p2);
    add_bond(commands, meshes, materials, parent, center, p3);
}
#[derive(Component)]
struct Shape;
