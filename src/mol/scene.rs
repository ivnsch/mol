use super::resource::{MolRender, MolStyle};
use crate::{mol2_asset_plugin::Mol2Molecule, ui::resource::CarbonCount};
use bevy::{asset::Handle, prelude::Resource};

#[derive(Debug, Resource)]
pub struct MolScene {
    pub content: MolSceneContent,
    pub style: MolStyle,
    pub render: MolRender,
}

#[derive(Debug)]
pub enum MolSceneContent {
    Generated(CarbonCount),
    Mol2 {
        handle: Handle<Mol2Molecule>,
        waiting_for_async_handle: bool,
    },
}
