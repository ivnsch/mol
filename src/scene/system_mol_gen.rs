use std::f32::consts::PI;

use super::{
    component::{MyInterParentBond, MyMolecule},
    helper::add_mol,
    resource::{MolRender, MolStyle, PreloadedAssets},
    system::{add_atom, clear},
};
use crate::scene::{component::MyParent, system::add_bond};
use crate::{element::Element, scene::helper::add_outer_parent};
use bevy::prelude::*;

#[allow(clippy::too_many_arguments)]
pub fn add_linear_alkane(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    mol_query: &Query<Entity, With<MyMolecule>>,
    center_first_carbon: Vec3,
    carbons: u32,
    preloaded_assets: &Res<PreloadedAssets>,
    query: &mut Query<&mut Transform, With<MyInterParentBond>>,
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
        mol_style,
        mol_render,
        mol,
        center_first_carbon,
        carbons,
        preloaded_assets,
        query,
    )
}

#[allow(clippy::too_many_arguments)]
fn add_linear_alkane_with_mol(
    commands: &mut Commands,
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    molecule: Entity,
    center_first_carbon: Vec3,
    carbons: u32,
    preloaded_assets: &Res<PreloadedAssets>,
    query: &mut Query<&mut Transform, With<MyInterParentBond>>,
) {
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
        preloaded_assets,
        query,
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
        &mut meshes,
        mol_style,
        mol_render,
        last_parent,
        center_first_carbon,
        false,
        preloaded_assets,
        query,
    );

    if inner_carbons == 0 {
        add_bond(
            commands,
            &mut meshes,
            &preloaded_assets.bond_mat,
            mol_style,
            mol_render,
            molecule,
            last_parent_trans,
            first_parent_trans,
            true,
            preloaded_assets,
            query,
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
                &mut meshes,
                &preloaded_assets.bond_mat,
                mol_style,
                mol_render,
                molecule,
                first_parent_trans,
                inner_parent_trans,
                true,
                preloaded_assets,
                query,
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
            &mut meshes,
            mol_style,
            mol_render,
            inner_parent,
            center_first_carbon,
            preloaded_assets,
            query,
        );

        if let Some(previous_trans) = previous_inner_parent_trans {
            add_bond(
                commands,
                &mut meshes,
                &preloaded_assets.bond_mat,
                mol_style,
                mol_render,
                molecule,
                inner_parent_trans,
                previous_trans,
                true,
                preloaded_assets,
                query,
            );
        }

        previous_inner_parent_trans = Some(inner_parent_trans);
    }

    if let Some(previous_trans) = previous_inner_parent_trans {
        add_bond(
            commands,
            &mut meshes,
            &preloaded_assets.bond_mat,
            mol_style,
            mol_render,
            molecule,
            last_parent_trans,
            previous_trans,
            true,
            preloaded_assets,
            query,
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
    assets: &Res<PreloadedAssets>,
    query: &mut Query<&mut Transform, With<MyInterParentBond>>,
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
            &assets.c_mat,
            &assets.atom_mesh,
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
            &assets.h_mat,
            &assets.atom_mesh,
        );

        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            p3,
            &Element::H,
            h_descr,
            &assets.h_mat,
            &assets.atom_mesh,
        );

        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            p4,
            &Element::H,
            h_descr,
            &assets.h_mat,
            &assets.atom_mesh,
        );
    }

    // add bonds connecting atoms

    add_bond(
        commands,
        meshes,
        &assets.bond_mat,
        mol_style,
        mol_render,
        parent,
        center,
        p2,
        false,
        assets,
        query,
    );
    add_bond(
        commands,
        meshes,
        &assets.bond_mat,
        mol_style,
        mol_render,
        parent,
        center,
        p3,
        false,
        assets,
        query,
    );
    add_bond(
        commands,
        meshes,
        &assets.bond_mat,
        mol_style,
        mol_render,
        parent,
        center,
        p4,
        false,
        assets,
        query,
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
                &assets.c_mat,
                &assets.atom_mesh,
            );
        }
        add_bond(
            commands,
            meshes,
            &assets.bond_mat,
            mol_style,
            mol_render,
            parent,
            center,
            p1,
            false,
            assets,
            query,
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
    assets: &Res<PreloadedAssets>,
    query: &mut Query<&mut Transform, With<MyInterParentBond>>,
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
            &assets.c_mat,
            &assets.atom_mesh,
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
            &assets.h_mat,
            &assets.atom_mesh,
        );
        add_atom(
            commands,
            mol_style,
            mol_render,
            parent,
            p3,
            &Element::H,
            h_descr,
            &assets.h_mat,
            &assets.atom_mesh,
        );
        add_bond(
            commands,
            meshes,
            &assets.bond_mat,
            mol_style,
            mol_render,
            parent,
            center,
            p2,
            false,
            assets,
            query,
        );
        add_bond(
            commands,
            meshes,
            &assets.bond_mat,
            mol_style,
            mol_render,
            parent,
            center,
            p3,
            false,
            assets,
            query,
        );
    }
}
