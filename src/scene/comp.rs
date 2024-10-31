use bevy::prelude::*;

pub fn sphere_pbr_bundle(
    position: Vec3,
    scale: f32,
    material: &Handle<StandardMaterial>,
    mesh: &Handle<Mesh>,
) -> PbrBundle {
    PbrBundle {
        mesh: mesh.clone(),
        material: material.clone(),
        transform: Transform::from_translation(position).with_scale(Vec3::new(scale, scale, scale)),
        ..default()
    }
}
