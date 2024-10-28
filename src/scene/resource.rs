use crate::{mol2_asset_plugin::Mol2Molecule, ui::resource::CarbonCount};
use bevy::{
    asset::Handle,
    pbr::StandardMaterial,
    prelude::{Mesh, Resource},
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
    Mol2 {
        handle: Handle<Mol2Molecule>,
        waiting_for_async_handle: bool,
    },
}

#[derive(Resource, Debug)]
pub struct MolStyle {
    pub atom_scale_ball_stick: f32,
    pub bond_len: f32,
    pub bond_diam: f32,
    pub atom_scale_ball: f32,
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

#[derive(Resource, Debug, Default)]
pub struct PreloadedAssets {
    pub h_mat: Handle<StandardMaterial>,
    pub c_mat: Handle<StandardMaterial>,
    pub o_mat: Handle<StandardMaterial>,
    pub n_mat: Handle<StandardMaterial>,
    pub f_mat: Handle<StandardMaterial>,
    pub p_mat: Handle<StandardMaterial>,
    pub s_mat: Handle<StandardMaterial>,
    pub ca_mat: Handle<StandardMaterial>,
    pub atom_mesh: Handle<Mesh>,
    pub bond_mat: Handle<StandardMaterial>,
}

// impl Default for P
