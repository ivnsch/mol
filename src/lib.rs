mod camera_controller;
mod defocus;
mod init_wasm_log;
mod linear_alkane;
mod rotator;
mod system_3d;
mod ui;

use bevy::app::App;
use linear_alkane::add_3d_scratch;
use system_3d::add_3d_space;
use ui::add_ui;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn game123() {
    let app = &mut App::new();

    add_3d_space(app);
    add_3d_scratch(app);
    add_ui(app);

    app.run();
}
