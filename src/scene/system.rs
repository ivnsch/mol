use super::{
    comp::sphere_pbr_bundle,
    component::{MyInterParentBond, MyMolecule, Shape},
    helper::add_mol,
    resource::{MolRender, MolScene, MolSceneContent, MolStyle},
};
use crate::{
    bounding_box::BoundingBox,
    debug::AddedBoundingBox,
    element::Element,
    mol2_asset_plugin::{bounding_box_for_mol, Mol2Molecule},
    scene::helper::add_outer_parent,
    ui::{
        component::TooltipMarker, event::UpdateSceneEvent, helper::add_tooltip,
        system::despawn_all_entities,
    },
};
use crate::{mol2_asset_plugin::Mol2Atom, scene::component::MyParent};
use bevy::{
    color::palettes::css::{BLACK, GREEN, LIGHT_CYAN, MAGENTA, ORANGE, RED, WHITE, YELLOW},
    prelude::*,
};
use bevy_mod_picking::{
    events::{Out, Over, Pointer},
    prelude::{Highlight, HighlightKind, On},
    PickableBundle,
};
use std::f32::consts::PI;

fn tooltip_descr(atom: &Mol2Atom) -> String {
    format!(
        "Id: {},\nname: {},\npos: {},\ntype: {},\nmol name: {}",
        atom.id,
        atom.name,
        atom.loc_vec3(),
        atom.type_,
        atom.mol_name
    )
}

pub fn handle_update_scene_event(
    mut event: EventReader<UpdateSceneEvent>,
    mut commands: Commands,
    molecule: Query<Entity, With<MyMolecule>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene: ResMut<MolScene>,
    assets: Res<Assets<Mol2Molecule>>,
    mut event_writer: EventWriter<AddedBoundingBox>,
) {
    for _ in event.read() {
        println!("got an update scene event!");
        update_scene(
            &mut commands,
            &molecule,
            &mut meshes,
            &mut materials,
            &mut scene,
            &assets,
            &mut event_writer,
        );
    }
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

pub fn check_file_loaded(
    mut commands: Commands,
    mol_query: Query<Entity, With<MyMolecule>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<Assets<Mol2Molecule>>,
    mut scene: ResMut<MolScene>,
    mut event_writer: EventWriter<AddedBoundingBox>,
) {
    if let MolSceneContent::Mol2 {
        handle,
        waiting_for_async_handle,
    } = &scene.content
    {
        if *waiting_for_async_handle {
            if let Some(mol) = assets.get(handle) {
                // got the molecule - set flag to false so this is not called again
                scene.content = MolSceneContent::Mol2 {
                    handle: handle.clone(),
                    waiting_for_async_handle: false,
                };

                let bounding_box = bounding_box_for_mol(mol);
                event_writer.send(AddedBoundingBox(bounding_box));

                println!("received loaded mol event, will rebuild");
                clear(&mut commands, &mol_query);

                draw_mol2_mol(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    mol,
                    &scene.style,
                    &scene.render,
                );
            }
        }
    }
}

pub fn handle_added_bounding_box(
    mut mol_query: Query<&mut Transform, With<MyMolecule>>,
    mut events: EventReader<AddedBoundingBox>,
) {
    if let Ok(mut transform) = mol_query.get_single_mut() {
        for e in events.read() {
            update_for_bounding_box(&mut transform, &e.0);
        }
    }
}

fn update_for_bounding_box(transform: &mut Transform, bounding_box: &BoundingBox) {
    transform.translation.x = -bounding_box.mid_x();
    transform.translation.y = -bounding_box.mid_y();
    transform.translation.z = -bounding_box.mid_z();
    println!(
        "new bounding box: {:?}, updated translation to: {:?}",
        bounding_box, transform.translation
    );
}

fn update_scene(
    commands: &mut Commands,
    mol_query: &Query<Entity, With<MyMolecule>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    scene: &mut ResMut<MolScene>,
    assets: &Res<Assets<Mol2Molecule>>,
    event_writer: &mut EventWriter<AddedBoundingBox>,
) {
    match &scene.content {
        MolSceneContent::Generated(carbon_count) => {
            add_linear_alkane(
                commands,
                meshes,
                materials,
                &scene.style,
                &scene.render,
                mol_query,
                Vec3::ZERO,
                carbon_count.0,
            );
        }
        MolSceneContent::Mol2 { handle, .. } => {
            if let Some(mol) = assets.get(handle) {
                clear(commands, &mol_query);

                // center molecule
                // we cleared the scene and will rebuild: we're adding a "new" bounding box
                let bounding_box = bounding_box_for_mol(mol);
                event_writer.send(AddedBoundingBox(bounding_box));

                // build scene
                draw_mol2_mol(
                    commands,
                    meshes,
                    materials,
                    mol,
                    &scene.style,
                    &scene.render,
                );
            } else {
                // when the user loads a file, there's *no* scene update event, so we shouldn't be here
                // this is for things like changing the rendering type: normally the file is already loaded
                println!(
                    "Warn: got update scene event of type Mol2 but file is not loaded (yet?)."
                );
            }
        }
    }
}

fn draw_mol2_mol(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mol: &Mol2Molecule,
    mol_style: &MolStyle,
    mol_render: &MolRender,
) {
    let mol_entity = add_mol(commands);

    let c_material = atom_material(materials, Element::C);
    let h_material = atom_material(materials, Element::H);
    let n_material = atom_material(materials, Element::N);
    let o_material = atom_material(materials, Element::O);
    let f_material = atom_material(materials, Element::F);
    let p_material = atom_material(materials, Element::P);
    let s_material = atom_material(materials, Element::S);
    let ca_material = atom_material(materials, Element::Ca);
    let atom_mesh: Handle<Mesh> = atom_mesh(meshes);
    let bond_material: Handle<StandardMaterial> = bond_material(materials);

    if *mol_render != MolRender::Stick {
        for atom in &mol.atoms {
            let material = match atom.element {
                Element::H => h_material.clone(),
                Element::C => c_material.clone(),
                Element::N => n_material.clone(),
                Element::O => o_material.clone(),
                Element::F => f_material.clone(),
                Element::P => p_material.clone(),
                Element::S => s_material.clone(),
                Element::Ca => ca_material.clone(),
            };

            add_atom(
                commands,
                mol_style,
                mol_render,
                mol_entity,
                atom.loc_vec3(),
                &atom.element,
                &tooltip_descr(atom),
                material,
                atom_mesh.clone(),
            );
        }
    }

    for bond in &mol.bonds {
        add_bond(
            commands,
            meshes,
            &bond_material,
            mol_style,
            mol_entity,
            // ASSUMPTION: atoms ordered by id, 1-indexed, no gaps
            // this seems to be always the case in mol2 files
            mol.atoms[bond.atom1 - 1].loc_vec3(),
            mol.atoms[bond.atom2 - 1].loc_vec3(),
            true,
        );
    }
}

fn clear(commands: &mut Commands, mol_query: &Query<Entity, With<MyMolecule>>) {
    despawn_all_entities(commands, mol_query);
}

/// each element has a unique color / material
fn atom_material(
    materials: &mut ResMut<Assets<StandardMaterial>>,
    element: Element,
) -> Handle<StandardMaterial> {
    let color = color_for_element(&element);
    let material = StandardMaterial {
        base_color: color.into(),
        ..default()
    };
    materials.add(material)
}

fn atom_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    meshes.add(Sphere { ..default() }.mesh().uv(32, 18))
}

fn bond_material(materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Srgba::new(0.4, 0.4, 0.4, 1.0).into(),
        ..default()
    })
}

#[allow(clippy::too_many_arguments)]
fn add_bond(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    mol_style: &MolStyle,
    parent: Entity,
    atom1_loc: Vec3,
    atom2_loc: Vec3,
    is_inter_parent: bool,
) {
    let bond = create_bond(meshes, material, mol_style, atom1_loc, atom2_loc);

    let entity = if is_inter_parent {
        commands.spawn((bond, MyInterParentBond))
    } else {
        commands.spawn(bond)
    }
    .id();
    commands.entity(parent).add_child(entity);
}

fn create_bond(
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    mol_style: &MolStyle,
    p1: Vec3,
    p2: Vec3,
) -> PbrBundle {
    let midpoint = (p1 + p2) / 2.0;

    let distance = p1.distance(p2);
    let direction = (p2 - p1).normalize();
    let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

    let mesh: Handle<Mesh> = meshes.add(Capsule3d {
        radius: mol_style.bond_diam,
        half_length: distance / 2.0,
    });

    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform {
            translation: midpoint,
            rotation,
            ..default()
        },
        ..default()
    }
}

fn color_for_element(element: &Element) -> Srgba {
    match element {
        Element::H => WHITE,
        Element::C => BLACK,
        Element::N => GREEN,
        Element::O => RED,
        Element::F => MAGENTA,
        Element::P => ORANGE,
        Element::S => YELLOW,
        Element::Ca => LIGHT_CYAN,
    }
}

#[allow(clippy::too_many_arguments)]
fn add_atom(
    commands: &mut Commands,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    parent: Entity,
    position: Vec3,
    element: &Element,
    description: &str,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
) {
    let pbr_bundle = sphere_pbr_bundle(
        position,
        sphere_scale(mol_render, mol_style, element),
        material,
        mesh,
    );
    let descr = description.to_string();

    let sphere = (
        pbr_bundle,
        PickableBundle::default(),
        On::<Pointer<Over>>::commands_mut(move |over, commands| {
            add_tooltip(commands, over.pointer_location.position, descr.clone());
        }),
        On::<Pointer<Out>>::run(
            |mut commands: Commands, tooltips_query: Query<Entity, With<TooltipMarker>>| {
                despawn_all_entities(&mut commands, &tooltips_query);
            },
        ),
        HIGHLIGHT_TINT.clone(),
        Shape,
    );

    let entity = commands.spawn(sphere).id();
    commands.entity(parent).add_child(entity);
}

fn sphere_scale(mol_render: &MolRender, mol_style: &MolStyle, element: &Element) -> f32 {
    let basic_scale = match mol_render {
        MolRender::BallStick => mol_style.atom_scale_ball_stick,
        MolRender::Ball => mol_style.atom_scale_ball,
        MolRender::Stick => mol_style.atom_scale_ball_stick, // sphere not added to scene - arbitrary
    };

    let van_der_waals_radius = match element {
        Element::H => 1.2,
        Element::C => 1.7,
        Element::N => 1.55,
        Element::O => 1.52,
        Element::F => 1.47,
        Element::P => 1.8,
        Element::S => 1.8,
        Element::Ca => 2.31,
    };
    let van_der_waals_scaling_factor = 1.;

    basic_scale * van_der_waals_radius * van_der_waals_scaling_factor
}

pub fn setup_molecule(mut commands: Commands) {
    add_mol(&mut commands);
}

pub fn trigger_init_scene_event(mut event: EventWriter<UpdateSceneEvent>) {
    event.send(UpdateSceneEvent);
}

#[allow(clippy::too_many_arguments)]
fn add_linear_alkane(
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
                h_material.clone(),
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
