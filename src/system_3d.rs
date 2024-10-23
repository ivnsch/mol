use crate::camera_controller::{CameraController, CameraControllerPlugin};
use crate::defocus::DefocusPlugin;
use crate::embedded_asset_plugin::EmbeddedAssetPlugin;
use crate::mol2_asset_plugin::Mol2AssetPlugin;
use crate::rotator::{Rotator, RotatorPlugin};
use crate::ui::resource::Mol2MoleculeRes;
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
    .insert_resource(Mol2MoleculeRes(None))
    .add_systems(Startup, (setup_camera, setup_light));
    // .add_systems(Update, setup_global_axes);
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

#[allow(dead_code)]
fn setup_global_axes(mut gizmos: Gizmos) {
    let size = 2.0;
    let zero = 0.0;
    // x
    gizmos.line(
        Vec3 {
            x: -size,
            y: zero,
            z: zero,
        },
        Vec3 {
            x: size,
            y: zero,
            z: zero,
        },
        GREEN,
    );
    // y
    gizmos.line(
        Vec3 {
            x: zero,
            y: -size,
            z: zero,
        },
        Vec3 {
            x: zero,
            y: size,
            z: zero,
        },
        RED,
    );
    // z
    gizmos.line(
        Vec3 {
            x: zero,
            y: zero,
            z: -size,
        },
        Vec3 {
            x: zero,
            y: zero,
            z: size,
        },
        BLUE,
    );
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 1.5, 8.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
        Rotator::default(),
    ));
}
