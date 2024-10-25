use bevy::prelude::Resource;

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
