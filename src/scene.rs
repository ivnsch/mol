use bevy::{asset::Handle, prelude::Resource};

use crate::{
    linear_alkane::{MolRender, MolStyle},
    mol2_asset_plugin::Mol2Molecule,
    ui::resource::CarbonCount,
};

#[derive(Debug, Resource)]
pub struct MolScene {
    pub content: MolSceneContent,
    pub style: MolStyle,
    pub render: MolRender,
}

#[derive(Debug)]
pub enum MolSceneContent {
    Generated(CarbonCount),
    Mol2(Option<Handle<Mol2Molecule>>),
}
