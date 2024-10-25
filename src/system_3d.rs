use crate::camera_controller::{CameraController, CameraControllerPlugin};
use crate::debug::FocusBoundingBox;
use crate::defocus::DefocusPlugin;
use crate::embedded_asset_plugin::EmbeddedAssetPlugin;
use crate::mol2_asset_plugin::Mol2AssetPlugin;
use crate::rotator::{Rotator, RotatorPlugin};
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
    .add_event::<FocusBoundingBox>()
    .add_systems(Startup, (setup_camera, setup_light))
    .add_systems(Update, handle_focus_bounding_box);
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
            transform: Transform::from_xyz(0., 1.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
        Rotator::default(),
    ));
}

fn handle_focus_bounding_box(
    mut camera_query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
    mut events: EventReader<FocusBoundingBox>,
) {
    if let Ok((mut transform, mut controller)) = camera_query.get_single_mut() {
        for e in events.read() {
            println!(
                "new bounding box: {:?}, max dist: {}",
                e.0,
                e.0.max_distance()
            );
            transform.translation.z = e.0.max_distance();
        }
    }
}
