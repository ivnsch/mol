use crate::load_mol2::Mol2Molecule;
use bevy::prelude::Resource;
use bevy::prelude::*;

/// event for when user clicked + or - on UI
#[derive(Event, Default, Debug)]
pub struct PlusMinusInputEvent {
    pub plus_minus: PlusMinusInput,
}

#[derive(Event, Default, Debug, Clone)]
pub struct LoadedMol2Event(pub Mol2Molecule);

#[derive(Event, Default, Debug)]
pub struct UiCarbonCountInputEvent(pub u32);

/// carried in the "clicked + or -" event
// TODO this probably doesn't need to be a resource
#[derive(Debug, Default, Clone, Copy, Resource)]
pub enum PlusMinusInput {
    #[default]
    Plus,
    Minus,
}
