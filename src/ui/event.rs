use bevy::prelude::*;

/// event for when user clicked + or - on UI
#[derive(Event, Default, Debug)]
pub struct PlusMinusInputEvent {
    pub plus_minus: PlusMinusInput,
}

// see note on handle_mol2_file_events
// #[derive(Event, Default, Debug, Clone)]
// pub struct LoadedMol2Event(pub Mol2Molecule);

/// carried in the "clicked + or -" event
#[derive(Debug, Default, Clone, Copy)]
pub enum PlusMinusInput {
    #[default]
    Plus,
    Minus,
}

#[derive(Event, Debug)]
pub struct UpdateSceneEvent;
