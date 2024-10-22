use std::cmp;

use bevy::{
    color::palettes::css::{BLACK, GREEN},
    ecs::query::QueryData,
    prelude::*,
};
use bevy_simple_text_input::{TextInputPlugin, TextInputSubmitEvent, TextInputSystem};
use load_mol2::load_mol2;

use crate::{
    load_mol2::{self, Mol2Molecule},
    smiles::process_smiles,
    ui_comps::generate_input_box,
    ui_helpers::{
        add_button, add_carbons_value_row, add_header, add_info_labels, add_rotate_row, add_spacer,
    },
};

#[derive(Event, Default, Debug)]
pub struct UiCarbonCountInputEvent(pub u32);

#[derive(Resource)]
pub struct UiInputSmiles(String);

#[derive(Event, Default, Debug, Clone)]
pub struct LoadedMol2Event(pub Mol2Molecule);

#[derive(Resource)]
pub struct UiInputEntities {
    pub carbon_count: Entity,
    pub smiles: Entity,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CarbonCount(pub u32);

#[derive(Component, Default, QueryData)]
pub struct CarbonCountLabelMarker;
#[derive(Component, Default)]
pub struct CarbonCountPlusMarker;
#[derive(Component, Default)]
pub struct CarbonCountMinusMarker;

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

pub fn add_ui(app: &mut App) {
    app.add_plugins(TextInputPlugin)
        .add_event::<UiCarbonCountInputEvent>()
        .add_event::<PlusMinusInputEvent>()
        .add_event::<LoadedMol2Event>()
        .insert_resource(PlusMinusInput::Plus)
        .insert_resource(UiInputSmiles("".to_string()))
        .add_systems(
            Update,
            (
                listen_ui_inputs,
                update_carbon_count_label,
                plus_button_handler,
                minus_button_handler,
                listen_carbon_count_ui_inputs,
                rot_x_button_handler,
                rot_y_button_handler,
                rot_z_button_handler,
                load_file_button_handler,
            ),
        )
        .add_systems(Startup, (setup_ui, setup_info_labels))
        .add_systems(Update, text_listener.after(TextInputSystem));
}

/// adds right column with ui elements to scene
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut event_writer: EventWriter<UiCarbonCountInputEvent>,
) {
    let font = asset_server.load("fonts/FiraMono-Medium.ttf");

    let root = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Px(130.0),
            height: Val::Percent(100.0),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    });

    let root_id = root.id();

    add_header(&mut commands, root_id, &font, "Carbon count:");

    let init_carbon_count = CarbonCount(5);
    let carbon_count_value_label =
        add_carbons_value_row(&mut commands, &font, root_id, init_carbon_count);
    commands.spawn(init_carbon_count);

    add_spacer(&mut commands, root_id);

    let smiles_input = generate_input_box(
        &font,
        root_id,
        &mut commands,
        "Smiles",
        SmilesInputMarker,
        "".to_string(),
    );

    add_spacer(&mut commands, root_id);

    add_header(&mut commands, root_id, &font, "Rotate");
    add_rotate_row(&mut commands, &font, root_id);

    add_button(
        &mut commands,
        root_id,
        &font,
        "Load mol2",
        LoadMol2ButtonMarker,
    );

    commands.insert_resource(UiInputEntities {
        carbon_count: carbon_count_value_label,
        smiles: smiles_input,
    });

    // trigger initial render
    event_writer.send(UiCarbonCountInputEvent(init_carbon_count.0));
}

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

/// carried in the "clicked + or -" event
// TODO this probably doesn't need to be a resource
#[derive(Debug, Default, Clone, Copy, Resource)]
pub enum PlusMinusInput {
    #[default]
    Plus,
    Minus,
}

/// event for when user clicked + or - on UI
#[derive(Event, Default, Debug)]
pub struct PlusMinusInputEvent {
    pub plus_minus: PlusMinusInput,
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
