mod camera_controller;
#[cfg(test)]
mod chemcore_exploration_tests;
mod defocus;
mod linear_alkane;
mod load_mol2;
mod rotator;
mod smiles;
mod system_3d;
mod ui;
mod ui_comps;
mod ui_events;
mod ui_handlers;
mod ui_helpers;
mod ui_markers;
mod ui_resources;

use bevy::app::App;
use linear_alkane::add_3d_scratch;
use system_3d::add_3d_space;
use ui::add_ui;

fn main() {
    let app = &mut App::new();

    add_3d_space(app);
    add_3d_scratch(app);
    add_ui(app);

    app.run();
}
