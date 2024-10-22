use bevy::{
    color::palettes::css::{GRAY, WHITE},
    prelude::*,
};
use bevy_simple_text_input::{TextInputBundle, TextInputSettings};

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
            height: Val::Px(30.0),
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
    commands.entity(root_id).push_children(&[spawned_label]);

    let spawned_wrapper = commands.spawn(wrapper).id();
    commands.entity(root_id).push_children(&[spawned_wrapper]);

    let spawned_text_input_bundle = commands.spawn((marker, text_input_bundle)).id();
    commands
        .entity(spawned_wrapper)
        .push_children(&[spawned_text_input_bundle]);

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
