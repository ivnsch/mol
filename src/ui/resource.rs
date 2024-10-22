use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct UiInputSmiles(pub String);

#[derive(Resource)]
pub struct UiInputCarbonCount(pub CarbonCount);

#[derive(Resource)]
pub struct UiInputEntities {
    pub carbon_count: Entity,
    pub smiles: Entity,
}

#[derive(Debug, Clone, Copy)]
pub struct CarbonCount(pub u32);
