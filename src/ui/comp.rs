use bevy::{
    color::palettes::css::{BLACK, GRAY, WHITE},
    prelude::*,
};
use bevy_simple_text_input::{TextInputBundle, TextInputSettings};

use super::marker::PopupMarker;

const ROW_HEIGHT: f32 = 30.;

pub fn spacer() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(20.0),
            ..default()
        },
        ..default()
    }
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

pub fn row() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Row,
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(ROW_HEIGHT),
            ..default()
        },
        ..default()
    }
}

pub fn bottom_row() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            flex_direction: FlexDirection::Row,
            bottom: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(ROW_HEIGHT),
            ..default()
        },
        ..default()
    }
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

pub fn square_button_bg() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            top: Val::Px(0.0),
            width: Val::Px(30.0),
            height: Val::Px(30.0),
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn square_button_text(font: &Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
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
    }
}

pub fn button_bg() -> ButtonBundle {
    ButtonBundle {
        style: Style {
            top: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            align_items: AlignItems::Center,
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn button_text(font: &Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
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
    }
}

pub fn tooltip(pos: Vec2, text: String) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            left: Val::Px(pos.x),
            top: Val::Px(pos.y),
            ..default()
        },
        text: Text::from_section(
            text,
            TextStyle {
                font_size: 12.0,
                color: Color::WHITE,
                ..default()
            },
        ),
        ..default()
    }
}

pub fn generate_info_label(font: &Handle<Font>, label: &str, top: f32) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(top),
            left: Val::Px(10.0),
            width: Val::Auto,
            height: Val::Auto,
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

pub fn generate_input_box<T>(
    font: &Handle<Font>,
    root_id: Entity,
    commands: &mut Commands,
    label: &str,
    marker: T,
    value: String,
) -> Entity
where
    T: Component,
{
    let label = generate_input_label(font, label);
    let wrapper = generate_input_wrapper();
    let text_input_bundle = generate_input(value);

    let spawned_label = commands.spawn(label).id();
    commands.entity(root_id).add_child(spawned_label);

    let spawned_wrapper = commands.spawn(wrapper).id();
    commands.entity(root_id).add_child(spawned_wrapper);

    let spawned_text_input_bundle = commands.spawn((marker, text_input_bundle)).id();
    commands
        .entity(spawned_wrapper)
        .add_child(spawned_text_input_bundle);

    spawned_text_input_bundle
}

pub fn generate_input_label(font: &Handle<Font>, label: &str) -> TextBundle {
    generate_label(font, label)
}

pub fn generate_label(font: &Handle<Font>, label: &str) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Auto,
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

pub fn generate_input_wrapper() -> NodeBundle {
    NodeBundle {
        style: Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.0),
            left: Val::Px(0.0),
            width: Val::Percent(100.0),
            height: Val::Px(30.0),
            margin: UiRect {
                bottom: Val::Px(20.0),
                ..default()
            },
            ..default()
        },
        ..default()
    }
}

pub fn generate_input(value: String) -> (NodeBundle, TextInputBundle) {
    let input = TextStyle {
        font_size: 14.,
        color: Color::WHITE,
        ..default()
    };

    (
        NodeBundle {
            style: Style {
                width: Val::Px(200.0),
                border: UiRect::all(Val::Px(1.0)),
                padding: UiRect::all(Val::Px(5.0)),
                ..default()
            },
            border_color: GRAY.into(),
            background_color: GRAY.into(),
            ..default()
        },
        TextInputBundle::default()
            .with_text_style(TextStyle {
                font_size: 40.,
                ..default()
            })
            .with_inactive(true)
            .with_value(value)
            .with_settings(TextInputSettings {
                retain_on_submit: true,
                ..default()
            })
            .with_text_style(input),
    )
}

pub fn add_controls_box(commands: &mut Commands, font: &Handle<Font>) {
    let fullscreen_parent = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_self: AlignSelf::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let parent = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            flex_direction: FlexDirection::Column,
            top: Val::Px(80.),
            left: Val::Auto,
            right: Val::Auto,
            width: Val::Px(250.),
            height: Val::Px(200.),
            ..default()
        },
        background_color: BLACK.into(),
        ..default()
    };

    let header_id = commands.spawn(generate_header(font, "Controls")).id();
    let label1_id = commands.spawn(control_row(font, "Move right: a")).id();
    let label2_id = commands.spawn(control_row(font, "Move left: d")).id();
    let label3_id = commands.spawn(control_row(font, "Zoom in: w")).id();
    let label4_id = commands.spawn(control_row(font, "Zoom out: s")).id();
    let label5_id = commands
        .spawn(control_row(font, "Rotate around z: z / shift-z"))
        .id();
    let label6_id = commands
        .spawn(control_row(font, "Rotate around y: y / shift-y"))
        .id();
    let label7_id = commands
        .spawn(control_row(font, "Rotate around x: x / shift-x"))
        .id();

    let full_screen_parent_id = commands.spawn((fullscreen_parent, PopupMarker)).id();
    let parent_id = commands.spawn(parent).id();

    commands.entity(full_screen_parent_id).add_child(parent_id);
    commands.entity(parent_id).push_children(&[
        header_id, label1_id, label2_id, label3_id, label4_id, label5_id, label6_id, label7_id,
    ]);
}

pub fn control_row(font: &Handle<Font>, text: &str) -> TextBundle {
    TextBundle {
        style: Style {
            position_type: PositionType::Relative,
            // left: Val::Auto,
            height: Val::Px(ROW_HEIGHT),
            ..default()
        },
        text: Text::from_section(
            text,
            TextStyle {
                font: font.clone(),
                font_size: 14.0,
                color: Color::WHITE,
            },
        ),
        ..default()
    }
}
