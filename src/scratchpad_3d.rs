use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_3d_scratch(app: &mut App) {
    app.add_systems(Startup, setup_molecule)
        .add_systems(PostStartup, (setup_atoms, setup_local_axes));
}

#[derive(Component, Default)]
pub struct MySphere;

#[derive(Component, Default)]
pub struct MyMolecule;

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

        let sphere = (
            PbrBundle {
                mesh,
                material: debug_material.clone(),
                transform: Transform::from_translation(position),
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

fn create_local_axis(
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    scale: Vec3,
) -> PbrBundle {
    let cuboid: Handle<Mesh> = meshes.add(Cuboid::default());

    PbrBundle {
        mesh: cuboid.clone(),
        material: material.clone(),
        transform: Transform {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale,
        },
        ..default()
    }
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

fn setup_local_axes(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut molecule: Query<Entity, With<MyMolecule>>,
) {
    let debug_material: Handle<StandardMaterial> = materials.add(StandardMaterial { ..default() });
    let line_scale = 2.0;
    let line_thickness = 0.01;

    if let Ok(molecule) = molecule.get_single_mut() {
        let entity_x = commands
            .spawn((
                create_local_axis(
                    &mut meshes,
                    &debug_material,
                    Vec3 {
                        x: line_scale,
                        y: line_thickness,
                        z: line_thickness,
                    },
                ),
                Shape,
            ))
            .id();
        let entity_y = commands
            .spawn((
                create_local_axis(
                    &mut meshes,
                    &debug_material,
                    Vec3 {
                        x: line_thickness,
                        y: line_scale,
                        z: line_thickness,
                    },
                ),
                Shape,
            ))
            .id();
        let entity_z = commands
            .spawn((
                create_local_axis(
                    &mut meshes,
                    &debug_material,
                    Vec3 {
                        x: line_thickness,
                        y: line_scale,
                        z: line_thickness,
                    },
                ),
                Shape,
            ))
            .id();
        commands
            .entity(molecule)
            .push_children(&[entity_x, entity_y, entity_z]);
    }
}

/// A marker component for our shapes so we can query them separately from other things
#[derive(Component)]
struct Shape;
