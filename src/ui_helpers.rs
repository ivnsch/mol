use bevy::prelude::Commands;

use crate::{
    ui::CarbonCount,
    ui_comps::{
        button_bg, button_text, generate_button_label, generate_header, generate_info_label, row,
        spacer, square_button_bg, square_button_text, tooltip,
    },
    ui_markers::{
        CarbonCountLabelMarker, CarbonCountMinusMarker, CarbonCountPlusMarker, RotXLabelMarker,
        RotYLabelMarker, RotZLabelMarker, TooltipMarker,
    },
};
use bevy::prelude::*;

/// adds a generic vertical spacer element with fixed height
pub fn add_spacer(commands: &mut Commands, root_id: Entity) {
    let spacer_id = commands.spawn(spacer()).id();
    commands.entity(root_id).push_children(&[spacer_id]);
}

/// adds component to set carbon count
/// returns the label (entity) with the numeric value
pub fn add_carbons_value_row(
    commands: &mut Commands,
    font: &Handle<Font>,
    root_id: Entity,
    init_carbon_count: CarbonCount,
) -> Entity {
    let row = row();
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

pub fn add_rotate_row(commands: &mut Commands, font: &Handle<Font>, root_id: Entity) {
    let row = row();

    let row_id = commands.spawn(row).id();
    commands.entity(root_id).push_children(&[row_id]);

    add_square_button(commands, row_id, font, "x", RotXLabelMarker);
    add_square_button(commands, row_id, font, "y", RotYLabelMarker);
    add_square_button(commands, row_id, font, "z", RotZLabelMarker);
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
        .spawn((marker, square_button_bg()))
        .with_children(|parent| {
            parent.spawn(square_button_text(font, label));
        })
        .id();
    commands.entity(container_id).push_children(&[button]);
}

pub fn add_tooltip(commands: &mut Commands, pos: Vec2, text: String) {
    commands.spawn((tooltip(pos, text), TooltipMarker));
}

pub fn add_button<T>(
    commands: &mut Commands,
    container_id: Entity,
    font: &Handle<Font>,
    label: &str,
    marker: T,
) where
    T: Component,
{
    let button = commands
        .spawn((marker, button_bg()))
        .with_children(|parent| {
            parent.spawn(button_text(font, label));
        })
        .id();
    commands.entity(container_id).push_children(&[button]);
}

pub fn add_info_labels(mut commands: Commands, font: &Handle<Font>) {
    commands.spawn(generate_info_label(font, "move right: a", 0.0));
    commands.spawn(generate_info_label(font, "move left: d", 20.0));
    commands.spawn(generate_info_label(font, "zoom in: w", 40.0));
    commands.spawn(generate_info_label(font, "zoom out: s", 60.0));
    commands.spawn(generate_info_label(
        font,
        "rotate around z: i / shift-i",
        80.0,
    ));
    commands.spawn(generate_info_label(
        font,
        "rotate around y: o / shift-o",
        100.0,
    ));
    commands.spawn(generate_info_label(
        font,
        "rotate around x: p / shift-p",
        120.0,
    ));
}
