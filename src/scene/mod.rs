mod comp;
pub mod component;
pub mod event;
mod helper;
pub mod resource;
mod system;

use self::{
    resource::{MolRender, MolScene, MolSceneContent, MolStyle},
    system::{
        check_file_loaded, handle_added_bounding_box, handle_update_scene_event, setup_molecule,
        trigger_init_scene_event,
    },
};
use bevy::app::{App, PostStartup, Startup, Update};
use bevy_mod_picking::DefaultPickingPlugins;
use event::UpdateSceneEvent;
use resource::PreloadedAssets;
use system::preload_item_assets;

#[allow(dead_code)]
pub fn add_mol_scene(app: &mut App) {
    app.add_plugins(DefaultPickingPlugins)
        .insert_resource(MolScene {
            content: MolSceneContent::Empty,
            style: MolStyle {
                atom_scale_ball_stick: 0.3,
                bond_diam: 0.07,
                atom_scale_ball: 1.8,
            },
            render: MolRender::BallStick,
        })
        .insert_resource(PreloadedAssets::default())
        .add_event::<UpdateSceneEvent>()
        .add_systems(Startup, preload_item_assets)
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
