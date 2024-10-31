use super::component::{MyMolecule, MyMoleculeWrapper};
use bevy::prelude::*;
use sim_controls::rotator::MouseController;

pub fn add_mol(commands: &mut Commands, parent: Entity) -> Entity {
    let mol = commands
        .spawn((
            Name::new("mol"),
            MyMolecule,
            MouseController::default(),
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
            MouseController::default(),
            SpatialBundle { ..default() },
        ))
        .id()
}
