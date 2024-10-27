pub mod component;
mod event;
mod helper;
mod marker;
pub mod resource;
mod system;

use self::{
    resource::{MolRender, MolScene, MolSceneContent, MolStyle},
    system::{
        check_file_loaded, handle_added_bounding_box, handle_update_scene_event, setup_molecule,
        trigger_init_scene_event,
    },
};
use crate::ui::{event::UpdateSceneEvent, resource::CarbonCount};
use bevy::app::{App, PostStartup, Startup, Update};
use bevy_mod_picking::DefaultPickingPlugins;

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
                handle_added_bounding_box,
            ),
        );
}
