use bevy::prelude::Component;

#[derive(Component, Default)]
pub struct MyMolecule;

#[derive(Component, Default)]
pub struct MyMoleculeWrapper;

#[derive(Component, Default)]
pub struct MyParent;

#[derive(Component, Default)]
pub struct MyBond {
    pub length: f32,
}

#[derive(Component)]
pub struct Shape;
