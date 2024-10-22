use crate::{
    ui_comps::generate_input_box,
    ui_events::{LoadedMol2Event, PlusMinusInput, PlusMinusInputEvent, UiCarbonCountInputEvent},
    ui_handlers::{
        listen_carbon_count_ui_inputs, listen_ui_inputs, load_file_button_handler,
        minus_button_handler, plus_button_handler, rot_x_button_handler, rot_y_button_handler,
        rot_z_button_handler, setup_info_labels, text_listener, update_carbon_count_label,
    },
    ui_helpers::{add_button, add_carbons_value_row, add_header, add_rotate_row, add_spacer},
};
use bevy::{ecs::query::QueryData, prelude::*};
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

#[derive(Resource)]
pub struct UiInputSmiles(pub String);

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
