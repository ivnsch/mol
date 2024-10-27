use std::f32::consts::PI;

use super::{
    component::MyMolecule,
    helper::add_mol,
    resource::{MolRender, MolStyle},
    system::{add_atom, clear},
};
use crate::scene::{
    component::MyParent,
    system::{add_bond, atom_material, atom_mesh, bond_material},
};
use crate::{element::Element, scene::helper::add_outer_parent};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn add_linear_alkane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    mol_query: &Query<Entity, With<MyMolecule>>,
    center_first_carbon: Vec3,
    carbons: u32,
) {
    clear(commands, mol_query);

    if carbons == 0 {
        println!("n == 0, nothing to draw");
        return;
    }

    let mol = add_mol(commands);

    add_linear_alkane_with_mol(
        commands,
        meshes,
        materials,
        mol_style,
        mol_render,
        mol,
        center_first_carbon,
        carbons,
    )
}

#[allow(clippy::too_many_arguments)]
fn add_linear_alkane_with_mol(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    molecule: Entity,
    center_first_carbon: Vec3,
    carbons: u32,
) {
    let c_material = atom_material(materials, Element::C);
    let h_material = atom_material(materials, Element::H);
    let bond_material = bond_material(materials);
    let atom_mesh: Handle<Mesh> = atom_mesh(meshes);

    let single = carbons == 1;
    let first_parent_rotation = Quat::from_rotation_z(if single {
        0.0_f32.to_radians()
    } else {
        -45.0_f32.to_radians()
    });

    // add parent wrapper entities to transform as a group
    let first_parent_trans = Vec3::new(0.0, 0.0, 0.0);
    let first_parent = add_outer_parent(
        commands,
        "first_parent",
        first_parent_rotation,
        first_parent_trans,
    );

    commands.entity(molecule).add_child(first_parent);
    add_outer_carbon(
        commands,
        meshes,
        mol_style,
        mol_render,
        first_parent,
        center_first_carbon,
        single,
        &c_material,
        &h_material,
        &bond_material,
        &atom_mesh,
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
        (mol_style.bond_len, 135.0_f32.to_radians())
    } else {
        (0.0, 45.0_f32.to_radians())
    };
    let last_parent_trans = Vec3::new(
        (inner_carbons + 1) as f32 * mol_style.bond_len,
        last_parent_y,
        0.0,
    );
    let last_parent = add_outer_parent(
        commands,
        "last_parent",
        Quat::from_rotation_z(last_parent_z_rot),
        last_parent_trans,
    );

    commands.entity(molecule).add_child(last_parent);
    add_outer_carbon(
        commands,
        meshes,
        mol_style,
        mol_render,
        last_parent,
        center_first_carbon,
        false,
        &c_material,
        &h_material,
        &bond_material,
        &atom_mesh,
    );

    if inner_carbons == 0 {
        add_bond(
            commands,
            meshes,
            &bond_material,
            mol_style,
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
        let inner_parent_y = if even { mol_style.bond_len } else { 0.0 };
        let inner_parent_trans = Vec3::new(
            mol_style.bond_len * i as f32 + mol_style.bond_len,
            inner_parent_y,
            0.0,
        );
        if i == 0 {
            add_bond(
                commands,
                meshes,
                &bond_material,
                mol_style,
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
        commands.entity(molecule).add_child(inner_parent);
        add_inner_carbon(
            commands,
            meshes,
            mol_style,
            mol_render,
            inner_parent,
            center_first_carbon,
            &c_material,
            &h_material,
            &atom_mesh,
            &bond_material,
        );

        if let Some(previous_trans) = previous_inner_parent_trans {
            add_bond(
                commands,
                meshes,
                &bond_material,
                mol_style,
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
            &bond_material,
            mol_style,
            molecule,
            last_parent_trans,
            previous_trans,
            true,
        );
    }
}

#[allow(clippy::too_many_arguments)]
/// the first or last carbon of the chain
fn add_outer_carbon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    parent: Entity,
    center: Vec3, // carbon center
    single: bool, // whether it's the only carbon in the molecule (methane)
    c_material: &Handle<StandardMaterial>,
    h_material: &Handle<StandardMaterial>,
    bond_material: &Handle<StandardMaterial>,
    atom_mesh: &Handle<Mesh>,
) {
    if *mol_render != MolRender::Stick {
        // center carbon
        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            center,
            &Element::C,
            "C",
            c_material.clone(),
            atom_mesh.clone(),
        );
    }

    // tetrahedral angle
    // note that this is used for the angles with the center of the molecule as vertex,
    // the angle between the atoms forming a circle has to be 120° (360° / 3 atoms)
    let bond_angle = 109.5_f32.to_radians();

    let rot_x = Quat::from_rotation_x(bond_angle);
    let rot_y_angle = 120.0_f32.to_radians();
    let rot_y = Quat::from_rotation_y(rot_y_angle);

    let mut p1 = Vec3::new(0.0, mol_style.bond_len, 0.0);

    let mut p2 = (rot_y * rot_x * Vec3::Y) * mol_style.bond_len;

    let rot_y_neg = Quat::from_rotation_y(-rot_y_angle);
    let mut p3 = (rot_y_neg * rot_x * Vec3::Y) * mol_style.bond_len;

    let mut p4 = rot_x * Vec3::Y * mol_style.bond_len;

    p1 += center;
    p2 += center;
    p3 += center;
    p4 += center;

    let h_descr = "H";

    if *mol_render != MolRender::Stick {
        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            p2,
            &Element::H,
            h_descr,
            h_material.clone(),
            atom_mesh.clone(),
        );

        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            p3,
            &Element::H,
            h_descr,
            h_material.clone(),
            atom_mesh.clone(),
        );

        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            p4,
            &Element::H,
            h_descr,
            h_material.clone(),
            atom_mesh.clone(),
        );
    }

    // add bonds connecting atoms

    add_bond(
        commands,
        meshes,
        bond_material,
        mol_style,
        parent,
        center,
        p2,
        false,
    );
    add_bond(
        commands,
        meshes,
        bond_material,
        mol_style,
        parent,
        center,
        p3,
        false,
    );
    add_bond(
        commands,
        meshes,
        bond_material,
        mol_style,
        parent,
        center,
        p4,
        false,
    );

    if single {
        // p1 only shown when there's only 1 carbon, i.e. 4 bonds with hydrogen
        if *mol_render != MolRender::Stick {
            add_atom(
                commands,
                mol_style,
                mol_render,
                parent,
                p1,
                &Element::H,
                "H",
                c_material.clone(),
                atom_mesh.clone(),
            );
        }
        add_bond(
            commands,
            meshes,
            bond_material,
            mol_style,
            parent,
            center,
            p1,
            false,
        );
    }
}

fn add_inner_carbon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    parent: Entity,
    center: Vec3,
    c_material: &Handle<StandardMaterial>,
    h_material: &Handle<StandardMaterial>,
    atom_mesh: &Handle<Mesh>,
    bond_material: &Handle<StandardMaterial>,
) {
    if *mol_render != MolRender::Stick {
        // center carbon
        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            center,
            &Element::C,
            "C",
            c_material.clone(),
            atom_mesh.clone(),
        );
    }

    // tetrahedral angle
    // note that this is used for the angles with the center of the molecule as vertex,
    // the angle between the atoms forming a circle has to be 120° (360° / 3 atoms)
    let bond_angle = 109.5_f32.to_radians();

    let rot_x = Quat::from_rotation_x(bond_angle);
    let rot_y_angle = 120.0_f32.to_radians();
    let rot_y = Quat::from_rotation_y(rot_y_angle);

    // first h
    let mut p2 = (rot_y * rot_x * Vec3::Y) * mol_style.bond_len;

    // second h
    let rot_y_neg = Quat::from_rotation_y(-rot_y_angle);
    let mut p3 = (rot_y_neg * rot_x * Vec3::Y) * mol_style.bond_len;

    p2 += center;
    p3 += center;

    let h_descr = "H";

    if *mol_render != MolRender::Stick {
        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            p2,
            &Element::H,
            h_descr,
            h_material.clone(),
            atom_mesh.clone(),
        );
        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            p3,
            &Element::H,
            h_descr,
            h_material.clone(),
            atom_mesh.clone(),
        );
        add_bond(
            commands,
            meshes,
            bond_material,
            mol_style,
            parent,
            center,
            p2,
            false,
        );
        add_bond(
            commands,
            meshes,
            bond_material,
            mol_style,
            parent,
            center,
            p3,
            false,
        );
    }
}
