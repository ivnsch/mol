use crate::bounding_box::BoundingBox;
use crate::camera_controller::{CameraController, CameraControllerPlugin};
use crate::debug::AddedBoundingBox;
use crate::defocus::DefocusPlugin;
use crate::embedded_asset_plugin::EmbeddedAssetPlugin;
use crate::mol2_asset_plugin::Mol2AssetPlugin;
use crate::rotator::{Rotator, RotatorPlugin};
use bevy::math::bounding;
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
    .add_event::<AddedBoundingBox>()
    .add_systems(Startup, (setup_camera, setup_light))
    .add_systems(Update, handle_added_bounding_box);
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
    // mut perspective_query: Query<(&mut Transform, &PerspectiveProjection)>,
    mut events: EventReader<AddedBoundingBox>,
) {
    if let Ok((mut transform, mut perspective)) = camera_query.get_single_mut() {
        match *perspective {
            Projection::Perspective(ref mut perspective) => {
                // for (mut transform, perspective) in perspective_query.iter_mut() {
                // println!("fov: {:?}", perspective.fov);
                for e in events.read() {
                    println!(
                        "new bounding box: {:?}, max dist: {}",
                        e.0,
                        e.0.max_distance()
                    );

                    let distance = calculate_camera_distance(&e.0, perspective.fov);
                    println!(
                        "fov: {}, calculated distance: {}",
                        perspective.fov, distance
                    );
                    transform.translation.z = e.0.max_distance();
                }
                // }
            }
            Projection::Orthographic(ref mut orthographic_projection) => todo!(),
        };
    }
    // }
}

fn calculate_camera_distance(bounding_box: &BoundingBox, fov: f32) -> f32 {
    // basic trig: we divide the triangle formed by fov and the max distance line in 2 right triangles,
    // the distance we're looking for is the adjacent side of the triangle
    (bounding_box.max_distance() / 2.0) / (fov / 2.0).tan()
}
