use std::cmp;

use bevy::{
    color::palettes::css::{BLACK, GREEN, WHITE},
    ecs::query::QueryData,
    prelude::*,
};

#[derive(Event, Default, Debug)]
pub struct UiInputsEvent {
    pub carbon_count: String,
}

#[derive(Resource)]
pub struct UiInputEntities {
    pub carbon_count: Entity,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct CarbonCount(pub u32);

#[derive(Component, Default, QueryData)]
pub struct CarbonCountLabelMarker;
#[derive(Component, Default)]
pub struct CarbonCountPlusMarker;
#[derive(Component, Default)]
pub struct CarbonCountMinusMarker;

pub fn add_ui(app: &mut App) {
    app.add_event::<UiInputsEvent>()
        .add_event::<PlusMinusInputEvent>()
        .insert_resource(PlusMinusInput::Plus)
        .add_systems(
            Update,
            (
                listen_ui_inputs,
                update_carbon_count_label,
                plus_button_handler,
                minus_button_handler,
                listen_carbon_count_ui_inputs,
            ),
        )
        .add_systems(Startup, setup_ui);
}

/// adds right column with ui elements to scene
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    commands.insert_resource(UiInputEntities {
        carbon_count: carbon_count_value_label,
    });
}

/// adds component to set carbon count
/// returns the label (entity) with the numeric value
pub fn add_carbons_value_row(
    commands: &mut Commands,
    font: &Handle<Font>,
    root_id: Entity,
    init_carbon_count: CarbonCount,
) -> Entity {
    let row = NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Row,
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            ..default()
        },
        ..default()
    };

    let row_id = commands.spawn(row).id();
    commands.entity(root_id).push_children(&[row_id]);

    let carbon_count_value_entity = add_button_label_with_marker(
        commands,
        row_id,
        font,
        &init_carbon_count.0.to_string(),
        CarbonCountLabelMarker,
    );

    add_square_button(commands, row_id, font, "-", CarbonCountMinusMarker);
    add_square_button(commands, row_id, font, "+", CarbonCountPlusMarker);

    carbon_count_value_entity
}

/// generates a column header styled text
pub fn generate_header(font: &Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Auto,
            margin: UiRect {
                bottom: Val::Px(10.0),
                ..default()
            },
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}

/// adds a label with a given marker
/// used for when we want to change the label dynamically
// is this specific to buttons? needs more generic name I think
pub fn add_button_label_with_marker<T>(
    commands: &mut Commands,
    row_id: Entity,
    font: &Handle<Font>,
    label: &str,
    marker: T,
) -> Entity
where
    T: Component,
{
    let label = generate_button_label(font, label);
    let spawned_label = commands.spawn((marker, label)).id();
    commands.entity(row_id).push_children(&[spawned_label]);
    spawned_label
}

/// generates a text label
/// meant to be added to a button (button-related dimensions)
// this obviously needs improvement, we should have a button component etc..
// but bevy's ui is very wip currently so keeping implementation low effort
pub fn generate_button_label(font: &Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(10.0),
            width: Val::Px(30.0),
            height: Val::Auto,
            align_self: AlignSelf::Center,
            ..default()
        },
        text: Text::from_section(
            label.to_string(),
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}

/// adds header to container
pub fn add_header(
    commands: &mut Commands,
    container_id: Entity,
    font: &Handle<Font>,
    label: &str,
) -> Entity {
    let label = generate_header(font, label);
    let spawned_label = commands.spawn(label).id();
    commands
        .entity(container_id)
        .push_children(&[spawned_label]);
    spawned_label
}

/// adds a square button to container
pub fn add_square_button<T>(
    commands: &mut Commands,
    container_id: Entity,
    font: &Handle<Font>,
    label: &str,
    marker: T,
) where
    T: Component,
{
    let button = commands
        .spawn((
            marker,
            ButtonBundle {
                style: Style {
                    top: Val::Px(0.0),
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text::from_section(
                    label.to_string(),
                    TextStyle {
                        font: font.clone(),
                        font_size: 14.0,
                        color: WHITE.into(),
                    }
                    .clone(),
                ),
                ..Default::default()
            });
        })
        .id();
    commands.entity(container_id).push_children(&[button]);
}

/// processes the ui events
/// basically, maps events to state
// TODO error handling (show on ui)
#[allow(clippy::too_many_arguments)]
pub fn listen_ui_inputs(
    mut events: EventReader<UiInputsEvent>,
    mut commands: Commands,
    carbon_count_query: Query<Entity, With<CarbonCount>>,
) {
    for input in events.read() {
        match parse_i32(&input.carbon_count) {
            Ok(i) => {
                // ensure only 1 carbon count active at a time
                despawn_all_entities(&mut commands, &carbon_count_query);
                // spawn new level
                commands.spawn(CarbonCount(i));
            }
            Err(err) => println!("error: {}", err),
        }
    }
}

pub fn parse_i32(str: &str) -> Result<u32, String> {
    let f = str.parse::<u32>();
    match f {
        Ok(i) => Ok(i),
        Err(e) => Err(format!("Failed to parse u32: {}", e)),
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
    mut my_events: EventWriter<PlusMinusInputEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        plus_minus_button_handler(
            (interaction, &mut color, &mut border_color),
            &mut my_events,
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
    mut my_events: EventWriter<PlusMinusInputEvent>,
) {
    for (interaction, mut color, mut border_color) in &mut interaction_query {
        plus_minus_button_handler(
            (interaction, &mut color, &mut border_color),
            &mut my_events,
            PlusMinusInput::Minus,
        );
    }
}

/// handles interactions with plus or minus button
/// it updates the button's appearance and sends an event
fn plus_minus_button_handler(
    interaction: (&Interaction, &mut BackgroundColor, &mut BorderColor),
    my_events: &mut EventWriter<PlusMinusInputEvent>,
    plus_minus: PlusMinusInput,
) {
    let (interaction, color, border_color) = interaction;
    match *interaction {
        Interaction::Pressed => {
            *color = GREEN.into();
            border_color.0 = GREEN.into();
            println!("sending plus minus event: {:?}", plus_minus);
            my_events.send(PlusMinusInputEvent { plus_minus });
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