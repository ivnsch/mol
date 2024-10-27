mod comp;
pub mod component;
pub mod event;
pub mod helper;
pub mod resource;
pub mod system;

use self::{
    helper::{add_controls_row, add_style_row},
    system::{
        close_popup_on_esc, controls_button_handler, focus, style_ball_button_handler,
        style_ball_stick_button_handler, style_stick_button_handler,
    },
};
use crate::ui::{
    comp::generate_input_box,
    component::{LoadMol2ButtonMarker, SmilesInputMarker},
    event::PlusMinusInputEvent,
    helper::{add_button, add_carbons_value_row, add_header, add_spacer},
    resource::{UiInputEntities, UiInputSmiles},
    system::{
        listen_carbon_count_ui_inputs, load_file_button_handler, minus_button_handler,
        plus_button_handler, setup_info_labels, text_listener, update_carbon_count_label,
    },
};
use bevy::prelude::*;
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

pub fn add_ui(app: &mut App) {
    app.add_plugins(TextInputPlugin)
        .add_event::<PlusMinusInputEvent>()
        // .add_event::<LoadedMol2Event>()
        .insert_resource(UiInputSmiles("".to_string()))
        .add_systems(
            Update,
            (
                update_carbon_count_label,
                plus_button_handler,
                minus_button_handler,
                listen_carbon_count_ui_inputs,
                load_file_button_handler,
                style_ball_stick_button_handler,
                style_stick_button_handler,
                style_ball_button_handler,
                controls_button_handler,
                close_popup_on_esc,
            ),
        )
        .add_systems(Startup, (setup_ui, setup_info_labels))
        .add_systems(Update, text_listener.after(TextInputSystem))
        .add_systems(Update, focus.before(TextInputSystem));
}

/// adds right column with ui elements to scene
pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("embedded://mol/asset/fonts/FiraMono-Medium.ttf");
    let root = commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Px(130.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(10.0)),
            ..default()
        },
        background_color: BackgroundColor(Color::BLACK),
        ..default()
    });

    let root_id = root.id();

    add_header(&mut commands, root_id, &font, "Carbon count:");

    let carbon_count_value_label = add_carbons_value_row(&mut commands, &font, root_id);

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

    add_button(
        &mut commands,
        root_id,
        &font,
        "Load mol2",
        LoadMol2ButtonMarker,
    );

    add_spacer(&mut commands, root_id);
    add_header(&mut commands, root_id, &font, "Style");
    add_style_row(&mut commands, &font, root_id);

    add_spacer(&mut commands, root_id);
    add_controls_row(&mut commands, &font, root_id);

    commands.insert_resource(UiInputEntities {
        carbon_count: carbon_count_value_label,
        smiles: smiles_input,
    });

    // shouldn't be necessary with the new trigger_init_scene_event - TODO confirm and delete this
    // carbon_count_event_writer.send(UiCarbonCountInputEvent(carbon_count.0 .0));
}
