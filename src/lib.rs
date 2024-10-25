mod bounding_box;
mod camera_controller;
#[cfg(test)]
mod chemcore_exploration_tests;
mod comp;
mod component;
mod debug;
mod defocus;
mod element;
mod embedded_asset_plugin;
mod event;
mod handler;
mod helper;
mod init_wasm_log;
mod marker;
mod mol2_asset_plugin;
mod resource;
mod rotator;
mod scene;
mod smiles;
mod system;
mod system_3d;
mod ui;

use bevy::app::App;
use system::add_mol_scene;
// use mol::add_mol_scene;
use system_3d::add_3d_space;
use ui::setup::add_ui;
// use ui::setup::add_ui;
use wasm_bindgen::prelude::wasm_bindgen;

// interop test - TODO remove
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// interop test - TODO remove
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn init_sim() {
    let app = &mut App::new();

    add_3d_space(app);

    // add_debug(app);
    add_mol_scene(app);
    add_ui(app);

    app.run();
}
