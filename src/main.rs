mod camera_controller;
mod defocus;
mod rotator;
mod scratchpad_3d;
mod system_3d;

use bevy::app::App;
use scratchpad_3d::add_3d_scratch;
use system_3d::add_3d_space;

fn main() {
    let app = &mut App::new();

    add_3d_space(app);
    add_3d_scratch(app);

    app.run();
}
