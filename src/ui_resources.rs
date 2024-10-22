use bevy::prelude::{Entity, Resource};

#[derive(Resource)]
pub struct UiInputSmiles(pub String);

#[derive(Resource)]
pub struct UiInputEntities {
    pub carbon_count: Entity,
    pub smiles: Entity,
}
