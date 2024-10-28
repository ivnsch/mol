use super::{
    comp::sphere_pbr_bundle,
    component::{MyInterParentBond, MyMolecule, MyMoleculeWrapper, Shape},
    event::{AddedBoundingBox, UpdateSceneEvent},
    helper::{add_mol, add_mol_wrapper},
    resource::{MolRender, MolScene, MolSceneContent, MolStyle, PreloadedAssets},
    system_mol_gen::add_linear_alkane,
};
use crate::mol2_asset_plugin::Mol2Atom;
use crate::{
    bounding_box::BoundingBox,
    element::Element,
    mol2_asset_plugin::{bounding_box_for_mol, Mol2Molecule},
    ui::{component::TooltipMarker, helper::add_tooltip, system::despawn_all_entities},
};
use bevy::{
    color::palettes::css::{BLACK, GREEN, LIGHT_CYAN, MAGENTA, ORANGE, RED, WHITE, YELLOW},
    prelude::*,
};
use bevy_mod_picking::{
    events::{Out, Over, Pointer},
    prelude::{Highlight, HighlightKind, On},
    PickableBundle,
};

const SPHERE_LAT: usize = 32;
const SPHERE_LON: usize = 18;
const CAPSULE_LAT: usize = 32;
const CAPSULE_LON: usize = 16;

pub fn preload_item_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut preloaded_assets: ResMut<PreloadedAssets>,
) {
    // This is slightly wasteful, as we don't need all the materials/elements for all the molecules
    // but cost is negligible. This is also executed only once in the lifetime of the app.
    // And performance should be "ideal" anyway for complex molecules / when using all the materials.
    let materials = &mut materials;
    let c_mat = atom_material(materials, Element::C);
    let h_mat = atom_material(materials, Element::H);
    let n_mat = atom_material(materials, Element::N);
    let o_mat = atom_material(materials, Element::O);
    let f_mat = atom_material(materials, Element::F);
    let p_mat = atom_material(materials, Element::P);
    let s_mat = atom_material(materials, Element::S);
    let ca_mat = atom_material(materials, Element::Ca);
    let atom_mesh: Handle<Mesh> = atom_mesh(&mut meshes);
    let bond_mat: Handle<StandardMaterial> = bond_material(materials);
    let bond_cyl_mesh: Handle<Mesh> = bond_cylinder_mesh(&mut meshes, 0.07);

    *preloaded_assets = PreloadedAssets {
        h_mat,
        c_mat,
        o_mat,
        n_mat,
        f_mat,
        p_mat,
        s_mat,
        ca_mat,
        atom_mesh,
        bond_mat,
        bond_cyl_mesh,
    };
}

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
    mut scene: ResMut<MolScene>,
    assets: Res<Assets<Mol2Molecule>>,
    preloaded_assets: Res<PreloadedAssets>,
    mut bond_query: Query<&mut Transform, With<MyInterParentBond>>,
    wrapper_query: Query<Entity, With<MyMoleculeWrapper>>,
) {
    for _ in event.read() {
        println!("got an update scene event!");

        update_scene(
            &mut commands,
            &molecule,
            &mut meshes,
            &mut scene,
            &assets,
            &preloaded_assets,
            &mut bond_query,
            &wrapper_query,
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
    assets: Res<Assets<Mol2Molecule>>,
    mut scene: ResMut<MolScene>,
    mut event_writer: EventWriter<AddedBoundingBox>,
    preloaded_assets: Res<PreloadedAssets>,
    mut bond_query: Query<&mut Transform, With<MyInterParentBond>>,
    wrapper_query: Query<Entity, With<MyMoleculeWrapper>>,
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
                    mol,
                    &scene.style,
                    &scene.render,
                    &preloaded_assets,
                    &mut bond_query,
                    &wrapper_query,
                );
            }
        }
    }
}

pub fn handle_added_bounding_box(
    mut mol_query: Query<&mut Transform, With<MyMoleculeWrapper>>,
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
    scene: &mut ResMut<MolScene>,
    assets: &Res<Assets<Mol2Molecule>>,
    preloaded_assets: &Res<PreloadedAssets>,
    bond_query: &mut Query<&mut Transform, With<MyInterParentBond>>,
    wrapper_query: &Query<Entity, With<MyMoleculeWrapper>>,
) {
    match &scene.content {
        MolSceneContent::Generated(carbon_count) => {
            add_linear_alkane(
                commands,
                meshes,
                &scene.style,
                &scene.render,
                mol_query,
                Vec3::ZERO,
                carbon_count.0,
                preloaded_assets,
                bond_query,
                wrapper_query,
            );
        }
        MolSceneContent::Mol2 { handle, .. } => {
            if let Some(mol) = assets.get(handle) {
                clear(commands, &mol_query);

                // build scene
                draw_mol2_mol(
                    commands,
                    meshes,
                    mol,
                    &scene.style,
                    &scene.render,
                    preloaded_assets,
                    bond_query,
                    wrapper_query,
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
    mol: &Mol2Molecule,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    assets: &Res<PreloadedAssets>,
    bond_query: &mut Query<&mut Transform, With<MyInterParentBond>>,
    wrapper_query: &Query<Entity, With<MyMoleculeWrapper>>,
) {
    if let Ok(wrapper) = wrapper_query.get_single() {
        let mol_entity = add_mol(commands, wrapper);

        if *mol_render != MolRender::Stick {
            for atom in &mol.atoms {
                let material = match atom.element {
                    Element::H => assets.h_mat.clone(),
                    Element::C => assets.c_mat.clone(),
                    Element::N => assets.n_mat.clone(),
                    Element::O => assets.o_mat.clone(),
                    Element::F => assets.f_mat.clone(),
                    Element::P => assets.p_mat.clone(),
                    Element::S => assets.s_mat.clone(),
                    Element::Ca => assets.ca_mat.clone(),
                };

                add_atom(
                    commands,
                    mol_style,
                    mol_render,
                    mol_entity,
                    atom.loc_vec3(),
                    &atom.element,
                    &tooltip_descr(atom),
                    &material,
                    &assets.atom_mesh,
                );
            }
        }

        if *mol_render != MolRender::Ball {
            for bond in &mol.bonds {
                add_bond(
                    commands,
                    meshes,
                    &assets.bond_mat,
                    mol_style,
                    mol_render,
                    mol_entity,
                    // ASSUMPTION: atoms ordered by id, 1-indexed, no gaps
                    // this seems to be always the case in mol2 files
                    mol.atoms[bond.atom1 - 1].loc_vec3(),
                    mol.atoms[bond.atom2 - 1].loc_vec3(),
                    true,
                    assets,
                    bond_query,
                );
            }
        }
    } else {
        eprintln!("No mol wrapper found, can't add mol.");
    }
}

pub fn clear(commands: &mut Commands, mol_query: &Query<Entity, With<MyMolecule>>) {
    despawn_all_entities(commands, mol_query);
}

/// each element has a unique color / material
pub fn atom_material(
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

pub fn atom_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    meshes.add(Sphere { ..default() }.mesh().uv(SPHERE_LAT, SPHERE_LON))
}

pub fn bond_material(materials: &mut ResMut<Assets<StandardMaterial>>) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: Srgba::new(0.4, 0.4, 0.4, 1.0).into(),
        ..default()
    })
}

pub fn bond_cylinder_mesh(meshes: &mut ResMut<Assets<Mesh>>, radius: f32) -> Handle<Mesh> {
    meshes.add(
        Cylinder {
            radius,
            half_height: 0.5,
        }
        .mesh(),
    )
}

#[allow(clippy::too_many_arguments)]
pub fn add_bond(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    parent: Entity,
    atom1_loc: Vec3,
    atom2_loc: Vec3,
    is_inter_parent: bool,
    preloaded_assets: &Res<PreloadedAssets>,
    bond_query: &mut Query<&mut Transform, With<MyInterParentBond>>,
) {
    let bond = create_bond(
        meshes,
        material,
        mol_style,
        mol_render,
        atom1_loc,
        atom2_loc,
        &preloaded_assets.bond_cyl_mesh,
    );

    let entity = if is_inter_parent {
        commands.spawn((bond, MyInterParentBond))
    } else {
        commands.spawn(bond)
    }
    .id();
    commands.entity(parent).add_child(entity);

    // Only BallStick uses cylinders (instead of Capsule3d or nothing)
    if *mol_render == MolRender::BallStick {
        // set bond length via transform (instead of directly on the cylinder, which would require loading separate meshes),
        // for better performance
        let distance = atom1_loc.distance(atom2_loc);
        for mut transform in bond_query.iter_mut() {
            transform.scale = Vec3::new(1.0, distance, 1.0);
        }
    }
}

fn create_bond(
    meshes: &mut ResMut<Assets<Mesh>>,
    material: &Handle<StandardMaterial>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    p1: Vec3,
    p2: Vec3,
    cylinder_mesh: &Handle<Mesh>,
) -> PbrBundle {
    let midpoint = (p1 + p2) / 2.0;

    let distance = p1.distance(p2);
    let direction = (p2 - p1).normalize();
    let rotation = Quat::from_rotation_arc(Vec3::Y, direction);

    let radius = mol_style.bond_diam;
    let half_length = distance / 2.0;

    let mesh = match mol_render {
        // Just clone mesh. Length will be adjusted via transform for better performance
        MolRender::BallStick | MolRender::Ball => cylinder_mesh.clone(),
        MolRender::Stick => meshes.add(
            Capsule3d {
                radius,
                half_length,
            }
            .mesh()
            .latitudes(CAPSULE_LAT)
            .longitudes(CAPSULE_LON),
        ),
    };

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
pub fn add_atom(
    commands: &mut Commands,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    parent: Entity,
    position: Vec3,
    element: &Element,
    description: &str,
    material: &Handle<StandardMaterial>,
    mesh: &Handle<Mesh>,
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
    let wrapper = add_mol_wrapper(&mut commands);
    add_mol(&mut commands, wrapper);
}

pub fn trigger_init_scene_event(mut event: EventWriter<UpdateSceneEvent>) {
    event.send(UpdateSceneEvent);
}
