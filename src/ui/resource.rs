use bevy::{
    asset::Handle,
    prelude::{Entity, Resource},
};

use crate::mol2_asset_plugin::Mol2Molecule;

#[derive(Resource)]
pub struct UiInputSmiles(pub String);

#[derive(Resource)]
pub struct UiInputCarbonCount(pub CarbonCount);

#[derive(Resource)]
pub struct UiInputEntities {
    pub carbon_count: Entity,
    pub smiles: Entity,
}

#[derive(Debug, Clone, Copy)]
pub struct CarbonCount(pub u32);

#[derive(Resource)]
pub struct Mol2MoleculeRes(pub Option<Handle<Mol2Molecule>>);
