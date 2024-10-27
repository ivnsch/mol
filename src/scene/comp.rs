use bevy::prelude::*;

pub fn outer_parent_spatial_bundle(rotation: Quat, translation: Vec3) -> SpatialBundle {
    SpatialBundle {
        transform: Transform {
            rotation,
            translation,
            ..Default::default()
        },
        ..default()
    }
}

pub fn sphere_pbr_bundle(
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    scale: f32,
    color: Srgba,
) -> PbrBundle {
    let debug_material: Handle<StandardMaterial> = materials.add(StandardMaterial {
        base_color: color.into(),
        ..default()
    });

    let mesh: Handle<Mesh> = meshes.add(Sphere { ..default() }.mesh().uv(32, 18));

    PbrBundle {
        mesh,
        material: debug_material.clone(),
        transform: Transform::from_translation(position).with_scale(Vec3::new(scale, scale, scale)),
        ..default()
    }
}
