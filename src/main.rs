mod camera_controller;
mod defocus;
mod linear_alkane;
mod rotator;
mod system_3d;
mod ui;

use mol::game123;

// this setup of being both lib and bin crate isn't entirely valid (warnings) but it works for now
fn main() {
    game123();
}
