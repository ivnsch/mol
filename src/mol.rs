use std::f32::consts::PI;

use bevy::{
    color::palettes::css::{BLACK, GREEN, LIGHT_CYAN, MAGENTA, ORANGE, RED, WHITE, YELLOW},
    prelude::*,
};
use bevy_mod_picking::{
    events::{Out, Over, Pointer},
    prelude::{Highlight, HighlightKind, On},
    DefaultPickingPlugins, PickableBundle,
};

use crate::{
    debug::FocusBoundingBox,
    element::Element,
    mol2_asset_plugin::{bounding_box_for_mol, Mol2Atom, Mol2Molecule},
    scene::{MolScene, MolSceneContent},
    ui::{
        event::UpdateSceneEvent, handler::despawn_all_entities, helper::add_tooltip,
        marker::TooltipMarker, resource::CarbonCount,
    },
};

#[allow(dead_code)]
pub fn add_mol_scene(app: &mut App) {
    app.add_plugins(DefaultPickingPlugins)
        .insert_resource(MolScene {
            content: MolSceneContent::Generated(CarbonCount(5)),
            style: MolStyle {
                atom_scale_ball_stick: 0.3,
                bond_len: 0.6,
                bond_diam: 0.07,
                atom_scale_ball: 1.8,
            },
            render: MolRender::BallStick,
        })
        .add_event::<UpdateSceneEvent>()
        .add_systems(Startup, setup_molecule)
        .add_systems(PostStartup, (trigger_init_scene_event,)) // TODO maybe it works in startup? test
        .add_systems(
            Update,
            (
                handle_update_scene_event,
                check_file_loaded,
                handle_focus_bounding_box,
            ),
        );
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
pub struct MyMolecule;

#[derive(Component, Default)]
pub struct MyParent;

#[derive(Component, Default)]
pub struct MyInterParentBond;

#[derive(Resource, Debug)]
pub struct MolStyle {
    atom_scale_ball_stick: f32,
    bond_len: f32,
    bond_diam: f32,
    atom_scale_ball: f32,
}

#[derive(Resource, PartialEq, Eq, Debug)]
pub enum MolRender {
    BallStick,
    #[allow(unused)]
    Stick,
    #[allow(unused)]
    // just a quick experiment - larger sphere scale
    Ball,
}

fn add_mol(commands: &mut Commands) -> Entity {
    commands
        .spawn((Name::new("mol"), MyMolecule, SpatialBundle { ..default() }))
        .id()
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

fn handle_update_scene_event(
    mut event: EventReader<UpdateSceneEvent>,
    mut commands: Commands,
    molecule: Query<Entity, With<MyMolecule>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut scene: ResMut<MolScene>,
    assets: Res<Assets<Mol2Molecule>>,
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
        );
    }
}

fn check_file_loaded(
    mut commands: Commands,
    mol_query: Query<Entity, With<MyMolecule>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<Assets<Mol2Molecule>>,
    mut scene: ResMut<MolScene>,
    mut event_writer: EventWriter<FocusBoundingBox>,
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
                event_writer.send(FocusBoundingBox(bounding_box));

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

fn handle_focus_bounding_box(
    mut mol_query: Query<&mut Transform, With<MyMolecule>>,
    mut events: EventReader<FocusBoundingBox>,
) {
    if let Ok(mut transform) = mol_query.get_single_mut() {
        for e in events.read() {
            let bb = &e.0;
            transform.translation.x = -bb.mid_x();
            transform.translation.y = -bb.mid_y();
            transform.translation.z = -bb.mid_z();
            println!(
                "new bounding box: {:?}, updated translation to: {:?}",
                bb, transform.translation
            );
        }
    }
}

fn update_scene(
    commands: &mut Commands,
    mol_query: &Query<Entity, With<MyMolecule>>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    scene: &mut ResMut<MolScene>,
    assets: &Res<Assets<Mol2Molecule>>,
) {
    println!("will update scene");
    match &scene.content {
        MolSceneContent::Generated(carbon_count) => {
            println!("scene render: {:?}", scene.render);
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
                println!("received loaded mol event, will rebuild");
                clear(commands, &mol_query);

                draw_mol2_mol(
                    // TODO replace parameter mol2_mol resource (remove resource) with mol2_molecule, and rename in mol2_mol
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

    if *mol_render != MolRender::Stick {
        for atom in &mol.atoms {
            add_atom(
                commands,
                meshes,
                materials,
                mol_style,
                mol_render,
                mol_entity,
                atom.loc_vec3(),
                color_for_element(&atom.element),
                &tooltip_descr(atom),
            );
        }
    }

    for bond in &mol.bonds {
        add_bond(
            commands,
            meshes,
            materials,
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

#[allow(clippy::too_many_arguments)]
fn add_bond(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mol_style: &MolStyle,
    parent: Entity,
    atom1_loc: Vec3,
    atom2_loc: Vec3,
    is_inter_parent: bool,
) {
    let material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: Srgba::new(0.4, 0.4, 0.4, 1.0).into(),
        ..default()
    });

    let bond = create_bond(meshes, &material, mol_style, atom1_loc, atom2_loc);

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
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    parent: Entity,
    position: Vec3,
    color: Srgba,
    description: &str,
) {
    let debug_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: color.into(),
        ..default()
    });

    let mesh = meshes.add(Sphere { ..default() }.mesh().uv(32, 18));

    let description_string = description.to_string();
    let scale = match mol_render {
        MolRender::BallStick => mol_style.atom_scale_ball_stick,
        MolRender::Ball => mol_style.atom_scale_ball,
        MolRender::Stick => mol_style.atom_scale_ball_stick, // sphere not added to scene - arbitrary
    };

    let sphere = (
        PbrBundle {
            mesh,
            material: debug_material.clone(),
            transform: Transform::from_translation(position)
                .with_scale(Vec3::new(scale, scale, scale)),
            ..default()
        },
        PickableBundle::default(),
        On::<Pointer<Over>>::commands_mut(move |click, target_commands| {
            add_tooltip(
                target_commands,
                click.pointer_location.position,
                description_string.clone(),
            );
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

fn setup_molecule(mut commands: Commands) {
    commands.spawn((
        Name::new("group"),
        MyMolecule,
        SpatialBundle { ..default() },
    ));
}

fn trigger_init_scene_event(mut event: EventWriter<UpdateSceneEvent>) {
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
    commands.entity(molecule).add_child(first_parent);
    add_outer_carbon(
        commands,
        meshes,
        materials,
        mol_style,
        mol_render,
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
        (mol_style.bond_len, 135.0_f32.to_radians())
    } else {
        (0.0, 45.0_f32.to_radians())
    };
    let last_parent_trans = Vec3::new(
        (inner_carbons + 1) as f32 * mol_style.bond_len,
        last_parent_y,
        0.0,
    );
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

    commands.entity(molecule).add_child(last_parent);
    add_outer_carbon(
        commands,
        meshes,
        materials,
        mol_style,
        mol_render,
        last_parent,
        center_first_carbon,
        false,
    );

    if inner_carbons == 0 {
        add_bond(
            commands,
            meshes,
            materials,
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
                materials,
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
            materials,
            mol_style,
            mol_render,
            inner_parent,
            center_first_carbon,
        );

        if let Some(previous_trans) = previous_inner_parent_trans {
            add_bond(
                commands,
                meshes,
                materials,
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
            materials,
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
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    parent: Entity,
    center: Vec3, // carbon center
    single: bool, // whether it's the only carbon in the molecule (methane)
) {
    if *mol_render != MolRender::Stick {
        // center carbon
        add_atom(
            commands,
            meshes,
            materials,
            mol_style,
            mol_render,
            parent,
            center,
            color_for_element(&Element::C),
            "C",
        );
    }

    // tetrahedral angle
    // note that this is used for the angles with the center of the molecule as vertex,
    // the angle between the molecules forming a circle has to be 120° (360° / 3 molecules)
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
    let h_color = color_for_element(&Element::H);

    if *mol_render != MolRender::Stick {
        add_atom(
            commands, meshes, materials, mol_style, mol_render, parent, p2, h_color, h_descr,
        );

        add_atom(
            commands, meshes, materials, mol_style, mol_render, parent, p3, h_color, h_descr,
        );

        add_atom(
            commands, meshes, materials, mol_style, mol_render, parent, p4, h_color, h_descr,
        );
    }

    // add bonds connecting atoms

    add_bond(
        commands, meshes, materials, mol_style, parent, center, p2, false,
    );
    add_bond(
        commands, meshes, materials, mol_style, parent, center, p3, false,
    );
    add_bond(
        commands, meshes, materials, mol_style, parent, center, p4, false,
    );

    if single {
        // p1 only shown when there's only 1 carbon, i.e. 4 bonds with hydrogen
        if *mol_render != MolRender::Stick {
            add_atom(
                commands, meshes, materials, mol_style, mol_render, parent, p1, WHITE, "H",
            );
        }
        add_bond(
            commands, meshes, materials, mol_style, parent, center, p1, false,
        );
    }
}

fn add_inner_carbon(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    mol_style: &MolStyle,
    mol_render: &MolRender,
    parent: Entity,
    center: Vec3,
) {
    if *mol_render != MolRender::Stick {
        // center carbon
        add_atom(
            commands,
            meshes,
            materials,
            mol_style,
            mol_render,
            parent,
            center,
            color_for_element(&Element::C),
            "C",
        );
    }

    // tetrahedral angle
    // note that this is used for the angles with the center of the molecule as vertex,
    // the angle between the molecules forming a circle has to be 120° (360° / 3 molecules)
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
    let h_color = color_for_element(&Element::H);

    if *mol_render != MolRender::Stick {
        add_atom(
            commands, meshes, materials, mol_style, mol_render, parent, p2, h_color, h_descr,
        );
        add_atom(
            commands, meshes, materials, mol_style, mol_render, parent, p3, h_color, h_descr,
        );
        add_bond(
            commands, meshes, materials, mol_style, parent, center, p2, false,
        );
        add_bond(
            commands, meshes, materials, mol_style, parent, center, p3, false,
        );
    }
}
#[derive(Component)]
struct Shape;
