use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_3d_scratch(app: &mut App) {
    app.add_systems(Startup, setup_molecule)
        .add_systems(PostStartup, (setup_atoms, setup_bonds));
}

#[derive(Component, Default)]
pub struct MySphere;

#[derive(Component, Default)]
pub struct MyMolecule;

fn add_bond(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    molecule: &mut Query<Entity, With<MyMolecule>>,
    atom1_loc: Vec3,
    atom2_loc: Vec3,
) {
    if let Ok(molecule) = molecule.get_single_mut() {
        let material: Handle<StandardMaterial> = materials.add(StandardMaterial { ..default() });

        let bond = create_bond(meshes, &material, atom1_loc, atom2_loc);

        let entity = commands.spawn(bond).id();
        commands.entity(molecule).push_children(&[entity]);
    } else {
        println!("couldn't get molecule")
    }
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
    molecule: &mut Query<Entity, With<MyMolecule>>,
    position: Vec3,
) {
    if let Ok(molecule) = molecule.get_single_mut() {
        let debug_material: Handle<StandardMaterial> =
            materials.add(StandardMaterial { ..default() });

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
    } else {
        println!("couldn't get molecule entity");
    }
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
    add_atom(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut molecule,
        Vec3::ZERO,
    );
    add_atom(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut molecule,
        Vec3::new(1.0, -1.0, 1.0),
    );
}

/// A marker component for our shapes so we can query them separately from other things
fn setup_bonds(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut molecule: Query<Entity, With<MyMolecule>>,
) {
    add_bond(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut molecule,
        Vec3::ZERO,
        Vec3::new(1.0, -1.0, 1.0),
    );
}

#[derive(Component)]
struct Shape;
