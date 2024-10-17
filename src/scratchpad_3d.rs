use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_3d_scratch(app: &mut App) {
    app.add_systems(Startup, setup_molecule)
        .add_systems(PostStartup, setup_atoms);
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
#[derive(Component)]
struct Shape;
