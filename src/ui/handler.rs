use std::cmp;

use crate::{
    load_mol2,
    smiles::process_smiles,
    ui::event::{LoadedMol2Event, PlusMinusInput, PlusMinusInputEvent, UiCarbonCountInputEvent},
    ui::helper::add_info_labels,
    ui::marker::{
        CarbonCountLabelMarker, CarbonCountMinusMarker, CarbonCountPlusMarker,
        LoadMol2ButtonMarker, RotXLabelMarker, RotYLabelMarker, RotZLabelMarker,
    },
    ui::resource::{UiInputEntities, UiInputSmiles},
};
use bevy::{
    color::palettes::css::{BLACK, BLUE, GRAY, GREEN},
    prelude::*,
};
use bevy_simple_text_input::{TextInputInactive, TextInputSubmitEvent};
use load_mol2::load_mol2;

use super::setup::CarbonCount;

/// processes the ui events
/// basically, maps events to state
// TODO error handling (show on ui)
#[allow(clippy::too_many_arguments)]
pub fn listen_ui_inputs(
    mut event: EventReader<UiCarbonCountInputEvent>,
    mut commands: Commands,
    carbon_count_query: Query<Entity, With<CarbonCount>>,
) {
    for input in event.read() {
        // ensure only 1 carbon count active at a time
        despawn_all_entities(&mut commands, &carbon_count_query);
        // spawn new level
        commands.spawn(CarbonCount(input.0));
    }
}

/// removes all entities matching a query (1 filter)
pub fn despawn_all_entities<T>(commands: &mut Commands, query: &Query<Entity, With<T>>)
where
    T: Component,
{
    for e in query.iter() {
        let entity = commands.entity(e);
        entity.despawn_recursive();
    }
}

/// handles interactions with plus button
/// it updates the button's appearance and sends an event
#[allow(clippy::type_complexity)]
pub fn plus_button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<CarbonCountPlusMarker>),
    >,
    mut event_writer: EventWriter<PlusMinusInputEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        plus_minus_button_handler(
            (interaction, &mut color, &mut border_color),
            &mut event_writer,
            PlusMinusInput::Plus,
        );
    }
}

/// handles interactions with minus button
/// it updates the button's appearance and sends an event
#[allow(clippy::type_complexity)]
pub fn minus_button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut BorderColor),
        (Changed<Interaction>, With<CarbonCountMinusMarker>),
    >,
    mut event_writer: EventWriter<PlusMinusInputEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        plus_minus_button_handler(
            (interaction, &mut color, &mut border_color),
            &mut event_writer,
            PlusMinusInput::Minus,
        );
    }
}

/// handles interactions with plus or minus button
/// it updates the button's appearance and sends an event
fn plus_minus_button_handler(
    interaction: (&Interaction, &mut BackgroundColor, &mut BorderColor),
    event_writer: &mut EventWriter<PlusMinusInputEvent>,
    plus_minus: PlusMinusInput,
) {
    let (interaction, color, border_color) = interaction;
    match *interaction {
        Interaction::Pressed => {
            *color = GREEN.into();
            border_color.0 = GREEN.into();
            println!("sending plus minus event: {:?}", plus_minus);
            event_writer.send(PlusMinusInputEvent { plus_minus });
        }
        Interaction::Hovered => {}
        Interaction::None => {
            *color = BLACK.into();
            border_color.0 = BLACK.into();
        }
    }
}

/// handles carbon count inputs
/// basically, we listen to clicks on the +/- buttons
/// then query the current carbon count, update it, and spawn the new value.
// TODO error handling (show on ui)
#[allow(clippy::too_many_arguments)]
pub fn listen_carbon_count_ui_inputs(
    mut events: EventReader<PlusMinusInputEvent>,
    mut commands: Commands,
    mut carbon_count_query: Query<&CarbonCount>,
    carbon_count_entity_query: Query<Entity, With<CarbonCount>>,
    mut event_writer: EventWriter<UiCarbonCountInputEvent>,
) {
    for input in events.read() {
        for e in carbon_count_query.iter_mut() {
            // println!("got carbon count: {:?}", e);
            // update
            let current = e.0;
            let increment: i32 = match input.plus_minus {
                PlusMinusInput::Plus => 1,
                PlusMinusInput::Minus => -1,
            };
            let new_i = current as i32 + increment;
            // pressing "-" at 0 stays at 0
            let new = cmp::max(0, new_i) as u32;

            // ensure only one carbon count at a time
            despawn_all_entities(&mut commands, &carbon_count_entity_query);
            // spawn updated carbon count
            let carbon_count = CarbonCount(new);
            commands.spawn(carbon_count);

            // send a new event reflecting the update
            event_writer.send(UiCarbonCountInputEvent(carbon_count.0));
        }
    }
}

/// updates the UI carbon count to reflect the current system entity
pub fn update_carbon_count_label(
    mut commands: Commands,
    carbon_count_query: Query<&CarbonCount>,
    input_entities: Res<UiInputEntities>,
    mut label_query: Query<(Entity, &mut Text), With<CarbonCountLabelMarker>>,
) {
    // current carbon count
    for carbon_count in carbon_count_query.iter() {
        // find the UI label
        let entity_id = commands.entity(input_entities.carbon_count).id();
        for (entity, mut text) in label_query.iter_mut() {
            if entity == entity_id {
                // update value
                text.sections[0].value = carbon_count.0.to_string();
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn load_file_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<LoadMol2ButtonMarker>)>,
    mut event_writer: EventWriter<LoadedMol2Event>,
) {
    for interaction in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            match load_mol2() {
                Ok(mol) => {
                    event_writer.send(LoadedMol2Event(mol));
                }
                Err(e) => {
                    println!("Error loading file: {:?}", e);
                }
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn rot_x_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RotXLabelMarker>)>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let q: Result<Mut<'_, Transform>, bevy::ecs::query::QuerySingleError> = query.get_single_mut();
    if let Ok(mut transform) = q {
        for interaction in &mut interaction_query {
            if interaction == &Interaction::Pressed {
                let rotation = 0.03;
                transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_euler(EulerRot::XYZ, rotation, 0.0, 0.0),
                );
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn rot_y_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RotYLabelMarker>)>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let q: Result<Mut<'_, Transform>, bevy::ecs::query::QuerySingleError> = query.get_single_mut();
    if let Ok(mut transform) = q {
        for interaction in &mut interaction_query {
            if interaction == &Interaction::Pressed {
                let rotation = 0.03;
                transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
                );
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn rot_z_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<RotZLabelMarker>)>,
    mut query: Query<&mut Transform, With<Camera>>,
) {
    let q: Result<Mut<'_, Transform>, bevy::ecs::query::QuerySingleError> = query.get_single_mut();
    if let Ok(mut transform) = q {
        for interaction in &mut interaction_query {
            if interaction == &Interaction::Pressed {
                let rotation = 0.03;
                transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_euler(EulerRot::XYZ, 0.0, 0.0, rotation),
                );
            }
        }
    }
}

pub fn setup_info_labels(commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");
    add_info_labels(commands, &font);
}

pub fn text_listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut input: ResMut<UiInputSmiles>,
    mut carbon_count_query: Query<&CarbonCount>,
    mut event_writer: EventWriter<UiCarbonCountInputEvent>,
    input_entities: Res<UiInputEntities>,
) {
    for text_input in events.read() {
        if text_input.entity == input_entities.smiles {
            println!("submitted smiles: {:?}", text_input.value);
            input.0 = text_input.value.clone();
            // TODO decouple from UI: trigger a new event with the string
            if let Err(e) =
                process_smiles(&mut carbon_count_query, &mut event_writer, input.0.clone())
            {
                println!("Error processing smiles: {e}")
            }
        } else {
            println!("unknown entity: {:?}", text_input.value);
        }
    }
}

pub fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BLUE.into();
                } else {
                    inactive.0 = true;
                    *border_color = GRAY.into();
                }
            }
        }
    }
}
