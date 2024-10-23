use super::{
    handler::focus,
    resource::{CarbonCount, UiInputCarbonCount},
};
use crate::{
    ui::comp::generate_input_box,
    ui::event::{PlusMinusInputEvent, UiCarbonCountInputEvent},
    ui::handler::{
        listen_carbon_count_ui_inputs, load_file_button_handler, minus_button_handler,
        plus_button_handler, rot_x_button_handler, rot_y_button_handler, rot_z_button_handler,
        setup_info_labels, text_listener, update_carbon_count_label,
    },
    ui::helper::{add_button, add_carbons_value_row, add_header, add_rotate_row, add_spacer},
    ui::marker::{LoadMol2ButtonMarker, SmilesInputMarker},
    ui::resource::{UiInputEntities, UiInputSmiles},
};
use bevy::prelude::*;
use bevy_simple_text_input::{TextInputPlugin, TextInputSystem};

pub fn add_ui(app: &mut App) {
    app.add_plugins(TextInputPlugin)
        .add_event::<UiCarbonCountInputEvent>()
        .add_event::<PlusMinusInputEvent>()
        // .add_event::<LoadedMol2Event>()
        .insert_resource(UiInputSmiles("".to_string()))
        .insert_resource(UiInputCarbonCount(CarbonCount(5)))
        .add_systems(
            Update,
            (
                update_carbon_count_label,
                plus_button_handler,
                minus_button_handler,
                listen_carbon_count_ui_inputs,
                rot_x_button_handler,
                rot_y_button_handler,
                rot_z_button_handler,
                load_file_button_handler,
                // handle_mol2_file_events, // see comment on fn
            ),
        )
        .add_systems(Startup, (setup_ui, setup_info_labels))
        .add_systems(Update, text_listener.after(TextInputSystem))
        .add_systems(Update, focus.before(TextInputSystem));
}

/// adds right column with ui elements to scene
pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    carbon_count: Res<UiInputCarbonCount>,
    mut carbon_count_event_writer: EventWriter<UiCarbonCountInputEvent>,
) {
    let font = asset_server.load("embedded://mol/asset/fonts/FiraMono-Medium.ttf");
    println!("loaded from embedded..");
    // let shader = asset_server.load::<Shader>("embedded://bevy_rock/render/rock.wgsl");

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

    let carbon_count_value_label =
        add_carbons_value_row(&mut commands, &font, root_id, carbon_count.0);

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

    carbon_count_event_writer.send(UiCarbonCountInputEvent(carbon_count.0 .0));
}
