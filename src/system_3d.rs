use std::f32::consts::PI;

use crate::bounding_box::BoundingBox;
use crate::defocus::DefocusPlugin;
use crate::embedded_asset_plugin::EmbeddedAssetPlugin;
use crate::mol2_asset_plugin::Mol2AssetPlugin;
use crate::rotator::RotatorPlugin;
use crate::scene::event::AddedBoundingBox;
use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;
use sim_controls::{
    camera_controller::{CameraController, CameraControllerPlugin},
    rotator::Rotator,
};

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
    .add_event::<AddedBoundingBox>()
    .add_systems(Startup, (setup_camera, setup_light))
    .add_systems(Update, handle_added_bounding_box);
}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1.0,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            // transform: Transform::from_xyz(0., 1.5, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            transform: Transform::from_xyz(0., 0., 8.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        CameraController::default(),
        Rotator::default(),
    ));
}

fn handle_added_bounding_box(
    mut camera_query: Query<(&mut Transform, &mut Projection), With<Camera>>,
    mut events: EventReader<AddedBoundingBox>,
) {
    if let Ok((mut transform, mut perspective)) = camera_query.get_single_mut() {
        match *perspective {
            Projection::Perspective(ref mut perspective) => {
                for e in events.read() {
                    // println!(
                    //     "new bounding box: {:?}, max dist: {}",
                    //     e.0,
                    //     e.0.max_distance()
                    // );

                    let distance = calculate_camera_distance(&e.0, perspective.fov);
                    // println!(
                    //     "fov: {}, calculated distance: {}",
                    //     perspective.fov, distance
                    // );
                    *transform = Transform::IDENTITY;
                    transform.translation.z = distance;
                }
            }
            Projection::Orthographic(_) => {}
        };
    }
}

fn calculate_camera_distance(bounding_box: &BoundingBox, fov: f32) -> f32 {
    // basic trig: we divide the triangle formed by fov and the max distance line in 2 right triangles,
    // the distance we're looking for is the adjacent side of the triangle
    (bounding_box.max_distance() / 2.0) / (fov / 2.0).tan()
}
