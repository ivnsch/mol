use crate::bounding_box::BoundingBox;
use bevy::prelude::Event;

#[derive(Event, Debug)]
pub struct UpdateSceneEvent;

/// a new bounding box (from a molecule) was added to the scene
#[derive(Event, Debug)]
pub struct AddedBoundingBox(pub BoundingBox);
