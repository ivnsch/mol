use super::{
    comp::outer_parent_spatial_bundle,
    component::{MyMolecule, MyMoleculeWrapper, MyParent},
};
use crate::rotator::MolController;
use bevy::prelude::*;

pub fn add_mol(commands: &mut Commands, parent: Entity) -> Entity {
    let mol = commands
        .spawn((
            Name::new("mol"),
            MyMolecule,
            MolController::default(),
            SpatialBundle { ..default() },
        ))
        .id();
    commands.entity(parent).add_child(mol);
    mol
}

pub fn add_mol_wrapper(commands: &mut Commands) -> Entity {
    commands
        .spawn((
            Name::new("wrapper"),
            MyMoleculeWrapper,
            MolController::default(),
            SpatialBundle { ..default() },
        ))
        .id()
}

pub fn add_outer_parent(
    commands: &mut Commands,
    name: &str,
    rotation: Quat,
    translation: Vec3,
) -> Entity {
    commands
        .spawn((
            Name::new(name.to_string()),
            outer_parent_spatial_bundle(rotation, translation),
            MyParent,
        ))
        .id()
}
