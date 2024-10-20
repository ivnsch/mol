use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{BLACK, DARK_GRAY, WHITE},
    prelude::*,
};
use bevy_mod_picking::{
    events::{Click, Pointer},
    prelude::{Highlight, HighlightKind, On},
    DefaultPickingPlugins, PickableBundle,
};

use crate::ui::{despawn_all_entities, UiInputsEvent};

#[allow(dead_code)]
pub fn add_3d_scratch(app: &mut App) {
    app.add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup_molecule)
        .add_systems(
            Update,
            setup_linear_alkane.run_if(run_if_carbon_count_changed),
        );
}

fn run_if_carbon_count_changed(mut events: EventReader<UiInputsEvent>) -> bool {
    for input in events.read() {
        println!("run_if_carbon_count_changed got an input: {:?}", input);
        if input.carbon_count_changed {
            return true;
        }
    }
    false
}

/// Used to tint the mesh instead of simply replacing the mesh's material with a single color. See
/// `tinted_highlight` for more details.
static HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(-0.5, -0.3, 0.9, 0.8), 0.5),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(-0.5, -0.3, 0.9, 0.8), 0.5),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(-0.4, -0.4, 0.8, 0.8), 0.5),

        ..matl.to_owned()
    })),
};

#[derive(Component, Default)]
pub struct MySphere;

#[derive(Component, Default)]
pub struct MyMolecule;

#[derive(Component, Default)]
pub struct MyParent;

#[derive(Component, Default)]
pub struct MyInterParentBond;

const ATOM_SCALE: f32 = 0.4;
const BOND_LENGTH: f32 = 1.0;
const BOND_DIAM: f32 = 0.01;

fn add_bond(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    molecule: Entity,
    atom1_loc: Vec3,
    atom2_loc: Vec3,
    is_inter_parent: bool,
) {
    let material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: DARK_GRAY.into(),
        ..default()
    });

    let bond = create_bond(meshes, &material, atom1_loc, atom2_loc);

    let entity = if is_inter_parent {
        commands.spawn((bond, MyInterParentBond))
    } else {
        commands.spawn(bond)
    }
    .id();
    commands.entity(molecule).push_children(&[entity]);
}

fn create_bond(
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    p1: Vec3,
    p2: Vec3,
) -> PbrBundle {
    let cuboid: Handle<Mesh> = meshes.add(Cuboid::default());

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
            scale: Vec3::new(BOND_DIAM, distance, BOND_DIAM),
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
    description: &str,
) {
    let debug_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: color,
        ..default()
    });

    let mesh = meshes.add(Sphere { ..default() }.mesh().uv(32, 18));

    let description_string = description.to_string();
    let sphere = (
        PbrBundle {
            mesh,
            material: debug_material.clone(),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::new(ATOM_SCALE, ATOM_SCALE, ATOM_SCALE)),
            ..default()
        },
        PickableBundle::default(),
        On::<Pointer<Click>>::target_commands_mut(move |_click, _target_commands| {
            println!("clicked! {description_string}")
        }),
        HIGHLIGHT_TINT.clone(),
        Shape,
    );

    let entity = commands.spawn(sphere).id();
    commands.entity(molecule).push_children(&[entity]);
}

fn setup_molecule(mut commands: Commands) {
    commands.spawn((
        Name::new("group"),
        MyMolecule,
        SpatialBundle { ..default() },
    ));
}

fn setup_linear_alkane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut molecule: Query<Entity, With<MyMolecule>>,
    parents: Query<Entity, With<MyParent>>,
    inter_parent_bonds: Query<Entity, With<MyInterParentBond>>,
    mut events: EventReader<UiInputsEvent>,
) {
    for input in events.read() {
        println!("rebuilding scene for {} carbons", input.carbon_count);

        despawn_all_entities(&mut commands, &parents);
        despawn_all_entities(&mut commands, &inter_parent_bonds);

        add_linear_alkane(
            &mut commands,
            &mut meshes,
            &mut materials,
            &mut molecule,
            Vec3::ZERO,
            // carbon_count.0,
            input.carbon_count,
        )
    }
}

fn add_linear_alkane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    molecule: &mut Query<Entity, With<MyMolecule>>,
    center_first_carbon: Vec3,
    carbons: u32,
) {
    if carbons == 0 {
        println!("n == 0, nothing to draw");
        return;
    }

    if let Ok(molecule) = molecule.get_single_mut() {
        let single = carbons == 1;
        let first_parent_rotation = Quat::from_rotation_z(if single {
            0.0_f32.to_radians()
        } else {
            -45.0_f32.to_radians()
        });

        // add parent wrapper entities to transform as a group
        let first_parent_trans = Vec3::new(0.0, 0.0, 0.0);
        let first_parent = commands
            .spawn((
                Name::new("first_parent"),
                SpatialBundle {
                    transform: Transform {
                        rotation: first_parent_rotation,
                        translation: first_parent_trans,
                        ..Default::default()
                    },
                    ..default()
                },
                MyParent,
            ))
            .id();
        commands.entity(molecule).push_children(&[first_parent]);
        add_outer_carbon(
            commands,
            meshes,
            materials,
            first_parent,
            center_first_carbon,
            single,
        );
        if single {
            return;
        }

        assert!(
            carbons >= 2,
            "programmatic error: should exit early if n < 2"
        );
        let inner_carbons = carbons - 2;

        let (last_parent_y, last_parent_z_rot) = if inner_carbons % 2 == 0 {
            (BOND_LENGTH, 135.0_f32.to_radians())
        } else {
            (0.0, 45.0_f32.to_radians())
        };
        let last_parent_trans =
            Vec3::new((inner_carbons + 1) as f32 * BOND_LENGTH, last_parent_y, 0.0);
        let last_parent = commands
            .spawn((
                Name::new("last_parent"),
                SpatialBundle {
                    transform: Transform {
                        rotation: Quat::from_rotation_z(last_parent_z_rot),
                        translation: last_parent_trans,
                        ..Default::default()
                    },
                    ..default()
                },
                MyParent,
            ))
            .id();

        commands.entity(molecule).push_children(&[last_parent]);
        add_outer_carbon(
            commands,
            meshes,
            materials,
            last_parent,
            center_first_carbon,
            false,
        );

        if inner_carbons == 0 {
            add_bond(
                commands,
                meshes,
                materials,
                molecule,
                last_parent_trans,
                first_parent_trans,
                true,
            );
            return;
        }

        let mut previous_inner_parent_trans = None;

        for i in 0..inner_carbons {
            let even = i % 2 == 0;
            let inner_parent_y = if even { BOND_LENGTH } else { 0.0 };
            let inner_parent_trans =
                Vec3::new(BOND_LENGTH * i as f32 + BOND_LENGTH, inner_parent_y, 0.0);
            if i == 0 {
                add_bond(
                    commands,
                    meshes,
                    materials,
                    molecule,
                    first_parent_trans,
                    inner_parent_trans,
                    true,
                );
            }
            let inner_parent = commands
                .spawn((
                    Name::new(format!("inner_parent_{i}")),
                    SpatialBundle {
                        transform: Transform {
                            rotation: if even {
                                Quat::from_euler(EulerRot::XYZ, PI, -PI / 4.0, 0.0)
                            } else {
                                Quat::from_euler(EulerRot::XYZ, 0.0, 135.0_f32.to_radians(), 0.0)
                            },
                            translation: inner_parent_trans,
                            ..Default::default()
                        },
                        ..default()
                    },
                    MyParent,
                ))
                .id();
            commands.entity(molecule).push_children(&[inner_parent]);
            add_inner_carbon(
                commands,
                meshes,
                materials,
                inner_parent,
                center_first_carbon,
            );

            if let Some(previous_trans) = previous_inner_parent_trans {
                add_bond(
                    commands,
                    meshes,
                    materials,
                    molecule,
                    inner_parent_trans,
                    previous_trans,
                    true,
                );
            }

            previous_inner_parent_trans = Some(inner_parent_trans);
        }

        if let Some(previous_trans) = previous_inner_parent_trans {
            add_bond(
                commands,
                meshes,
                materials,
                molecule,
                last_parent_trans,
                previous_trans,
                true,
            );
        }
    } else {
        println!("couldn't get molecule entity");
    }
}

/// the first or last carbon of the chain
fn add_outer_carbon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
    center: Vec3, // carbon center
    single: bool, // whether it's the only carbon in the molecule (methane)
) {
    // center carbon
    add_atom(
        commands,
        meshes,
        materials,
        parent,
        center,
        BLACK.into(),
        "C",
    );

    // tetrahedral angle
    // note that this is used for the angles with the center of the molecule as vertex,
    // the angle between the molecules forming a circle has to be 120° (360° / 3 molecules)
    let bond_angle = 109.5_f32.to_radians();

    let rot_x = Quat::from_rotation_x(bond_angle);
    let rot_y_angle = 120.0_f32.to_radians();
    let rot_y = Quat::from_rotation_y(rot_y_angle);

    let mut p1 = Vec3::new(0.0, BOND_LENGTH, 0.0);

    let mut p2 = (rot_y * rot_x * Vec3::Y) * BOND_LENGTH;

    let rot_y_neg = Quat::from_rotation_y(-rot_y_angle);
    let mut p3 = (rot_y_neg * rot_x * Vec3::Y) * BOND_LENGTH;

    let mut p4 = rot_x * Vec3::Y * BOND_LENGTH;

    p1 += center;
    p2 += center;
    p3 += center;
    p4 += center;

    add_atom(commands, meshes, materials, parent, p2, WHITE.into(), "H");

    add_atom(commands, meshes, materials, parent, p3, WHITE.into(), "H");

    add_atom(commands, meshes, materials, parent, p4, WHITE.into(), "H");

    // add bonds connecting atoms

    add_bond(commands, meshes, materials, parent, center, p2, false);
    add_bond(commands, meshes, materials, parent, center, p3, false);
    add_bond(commands, meshes, materials, parent, center, p4, false);

    if single {
        // p1 only shown when there's only 1 carbon, i.e. 4 bonds with hydrogen
        add_atom(commands, meshes, materials, parent, p1, WHITE.into(), "H");
        add_bond(commands, meshes, materials, parent, center, p1, false);
    }
}

fn add_inner_carbon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    parent: Entity,
    center: Vec3,
) {
    // center carbon
    add_atom(
        commands,
        meshes,
        materials,
        parent,
        center,
        BLACK.into(),
        "C",
    );

    // tetrahedral angle
    // note that this is used for the angles with the center of the molecule as vertex,
    // the angle between the molecules forming a circle has to be 120° (360° / 3 molecules)
    let bond_angle = 109.5_f32.to_radians();

    let rot_x = Quat::from_rotation_x(bond_angle);
    let rot_y_angle = 120.0_f32.to_radians();
    let rot_y = Quat::from_rotation_y(rot_y_angle);

    // first h
    let mut p2 = (rot_y * rot_x * Vec3::Y) * BOND_LENGTH;

    // second h
    let rot_y_neg = Quat::from_rotation_y(-rot_y_angle);
    let mut p3 = (rot_y_neg * rot_x * Vec3::Y) * BOND_LENGTH;

    p2 += center;
    p3 += center;

    add_atom(commands, meshes, materials, parent, p2, WHITE.into(), "H");
    add_atom(commands, meshes, materials, parent, p3, WHITE.into(), "H");
    add_bond(commands, meshes, materials, parent, center, p2, false);
    add_bond(commands, meshes, materials, parent, center, p3, false);
}
#[derive(Component)]
struct Shape;
