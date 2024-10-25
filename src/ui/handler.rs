use crate::{
    linear_alkane::MolRender,
    mol2_asset_plugin::Mol2Molecule,
    scene::{MolScene, MolSceneContent},
    smiles::process_smiles,
    ui::{
        event::{PlusMinusInput, PlusMinusInputEvent},
        helper::add_info_labels,
        marker::{
            CarbonCountLabelMarker, CarbonCountMinusMarker, CarbonCountPlusMarker,
            LoadMol2ButtonMarker, RotXLabelMarker, RotYLabelMarker, RotZLabelMarker,
        },
        resource::{UiInputEntities, UiInputSmiles},
    },
};
use bevy::{
    color::palettes::css::{BLACK, BLUE, GRAY, GREEN},
    prelude::*,
};
use bevy_simple_text_input::{TextInputInactive, TextInputSubmitEvent};
use std::cmp;

use super::{
    event::UpdateSceneEvent,
    marker::{StyleBallMarker, StyleBallStickMarker, StyleStickMarker},
    resource::CarbonCount,
};

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
/// then update the scene accordingly
// TODO error handling (show on ui)
#[allow(clippy::too_many_arguments)]
pub fn listen_carbon_count_ui_inputs(
    mut events: EventReader<PlusMinusInputEvent>,
    mut event_writer: EventWriter<UpdateSceneEvent>,
    mut scene: ResMut<MolScene>,
) {
    for input in events.read() {
        // update
        let current = match scene.content {
            MolSceneContent::Generated(carbon_count) => carbon_count,
            // if currently not displaying the generator, start a new one with 5 (just some number) carbons
            MolSceneContent::Mol2 { .. } => CarbonCount(5),
        };
        let increment: i32 = match input.plus_minus {
            PlusMinusInput::Plus => 1,
            PlusMinusInput::Minus => -1,
        };

        // TODO replace this with update the scene and send rebuild scene event!

        let new_i = current.0 as i32 + increment;
        // pressing "-" at 0 stays at 0
        let new = cmp::max(0, new_i) as u32;

        // we generate a new scene with the new carbon count
        let scene_content = MolSceneContent::Generated(CarbonCount(new));
        scene.content = scene_content;
        event_writer.send(UpdateSceneEvent);
    }
}

/// Updates carbon count label to reflect scene state
pub fn update_carbon_count_label(
    mut commands: Commands,
    input_entities: Res<UiInputEntities>,
    mut label_query: Query<(Entity, &mut Text), With<CarbonCountLabelMarker>>,
    scene: Res<MolScene>,
) {
    let entity_id = commands.entity(input_entities.carbon_count).id();
    for (entity, mut text) in label_query.iter_mut() {
        if entity == entity_id {
            // update value
            text.sections[0].value = match &scene.content {
                MolSceneContent::Generated(carbon_count) => carbon_count.0.to_string(),
                MolSceneContent::Mol2 { .. } => "".to_string(),
            };
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn load_file_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<LoadMol2ButtonMarker>)>,
    asset_server: Res<AssetServer>,
    mut scene: ResMut<MolScene>,
) {
    for interaction in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            // let path = "embedded://mol/asset/benzene.mol2";
            let path = "embedded://mol/asset/117_ideal.mol2";
            // let path = "embedded://mol/asset/2bbv.mol2";
            let handle: Handle<Mol2Molecule> = asset_server.load(path);

            scene.content = MolSceneContent::Mol2 {
                handle,
                // don't trigger update scene as the file may not be ready
                // an Update system polls the handle instead
                // this flag is set back to false when the file is ready
                // the file stays in the scene state to be available for other re-building events
                // (like changing the mol rendering type)
                waiting_for_async_handle: true,
            };
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

#[allow(clippy::type_complexity)]
pub fn style_ball_stick_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StyleBallStickMarker>)>,
    mut scene: ResMut<MolScene>,
    mut event_writer: EventWriter<UpdateSceneEvent>,
) {
    for interaction in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            println!("setting render to ball stick");
            scene.render = MolRender::BallStick;
            event_writer.send(UpdateSceneEvent);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn style_stick_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StyleStickMarker>)>,
    mut scene: ResMut<MolScene>,
    mut event_writer: EventWriter<UpdateSceneEvent>,
) {
    for interaction in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            println!("setting render to stick");
            scene.render = MolRender::Stick;
            event_writer.send(UpdateSceneEvent);
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn style_ball_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<StyleBallMarker>)>,
    mut scene: ResMut<MolScene>,
    mut event_writer: EventWriter<UpdateSceneEvent>,
) {
    for interaction in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            println!("setting render to ball");
            scene.render = MolRender::Ball;
            event_writer.send(UpdateSceneEvent);
        }
    }
}

pub fn setup_info_labels(commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("embedded://mol/asset/fonts/FiraMono-Medium.ttf");
    add_info_labels(commands, &font);
}

pub fn text_listener(
    mut events: EventReader<TextInputSubmitEvent>,
    mut input: ResMut<UiInputSmiles>,
    mut event_writer: EventWriter<UpdateSceneEvent>,
    mut scene: ResMut<MolScene>,
    input_entities: Res<UiInputEntities>,
) {
    for text_input in events.read() {
        if text_input.entity == input_entities.smiles {
            println!("submitted smiles: {:?}", text_input.value);
            input.0 = text_input.value.clone();
            match process_smiles(input.0.clone()) {
                Ok(carbon_count) => {
                    let scene_content = MolSceneContent::Generated(carbon_count);
                    scene.content = scene_content;
                    event_writer.send(UpdateSceneEvent);
                }
                Err(e) => {
                    println!("Error processing smiles: {e}")
                }
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
