use crate::camera_controller::{CameraController, CameraControllerPlugin};
use crate::defocus::DefocusPlugin;
use crate::embedded_asset_plugin::EmbeddedAssetPlugin;
use crate::mol2_asset_plugin::Mol2AssetPlugin;
use crate::rotator::{Rotator, RotatorPlugin};
use bevy::color::palettes::css::{BLUE, GREEN, RED};
use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_3d_space(app: &mut App) {
    app.add_plugins((
        DefaultPlugins,
        EmbeddedAssetPlugin,
        Mol2AssetPlugin,
        CameraControllerPlugin,
        RotatorPlugin,
        DefocusPlugin,
    ))
    .add_systems(Startup, (setup_camera, setup_light));
}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 0.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
        Rotator::default(),
    ));
}
