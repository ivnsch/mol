mod camera_controller;
mod defocus;
mod rotator;
mod linear_alkane;
mod system_3d;

use bevy::app::App;
use linear_alkane::add_3d_scratch;
use system_3d::add_3d_space;

fn main() {
    let app = &mut App::new();

    add_3d_space(app);
    add_3d_scratch(app);

    app.run();
}
