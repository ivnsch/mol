use bevy::{ecs::query::QueryData, prelude::Component};

#[derive(Component, Default, QueryData)]
pub struct RotXLabelMarker;
#[derive(Component, Default)]
pub struct RotYLabelMarker;
#[derive(Component, Default)]
pub struct RotZLabelMarker;

#[derive(Component, Default)]
pub struct SmilesInputMarker;

#[derive(Component, Default)]
pub struct LoadMol2ButtonMarker;

#[derive(Component, Default)]
pub struct TooltipMarker;

#[derive(Component, Default)]
pub struct StyleBallStickMarker;
#[derive(Component, Default)]
pub struct StyleBallMarker;
#[derive(Component, Default)]
pub struct StyleStickMarker;

#[derive(Component, Default)]
pub struct ControlsButtonMarker;

#[derive(Component, Default)]
pub struct PopupMarker;

#[derive(Component, Default)]
pub struct MolNameMarker;

#[derive(Component, Default)]
pub enum MolExampleFile {
    #[default]
    Benzene,
    _117,
    _1ubq,
    _2bbv,
}
