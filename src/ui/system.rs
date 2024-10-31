use crate::{
    mol2_asset_plugin::Mol2Molecule,
    scene::{
        event::UpdateSceneEvent,
        resource::{MolRender, MolScene, MolSceneContent},
    },
    ui::{component::LoadMol2ButtonMarker, helper::add_info_labels},
};
use bevy::{
    color::palettes::css::{BLUE, GRAY},
    prelude::*,
};
use bevy_simple_text_input::TextInputInactive;

use super::{
    comp::add_controls_box,
    component::{
        ControlsButtonMarker, MolExampleFile, MolNameMarker, PopupMarker, StyleBallMarker,
        StyleBallStickMarker, StyleStickMarker,
    },
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

#[allow(clippy::type_complexity)]
pub fn load_file_button_handler(
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<LoadMol2ButtonMarker>)>,
    asset_server: Res<AssetServer>,
    mut scene: ResMut<MolScene>,
) {
    for interaction in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            load_example_file(&asset_server, &mut scene, &MolExampleFile::Benzene);
        }
    }
}

fn load_example_file(
    asset_server: &Res<AssetServer>,
    scene: &mut ResMut<MolScene>,
    file: &MolExampleFile,
) {
    let file_name = match file {
        MolExampleFile::Benzene => "benzene.mol2",
        MolExampleFile::_117 => "117_ideal.mol2",
        MolExampleFile::_1ubq => "1ubq.mol2",
        MolExampleFile::_2bbv => "2bbv.mol2",
    };

    let path = format!("embedded://mol/asset/{}", file_name);
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

#[allow(clippy::type_complexity)]
pub fn controls_button_handler(
    mut commands: Commands,
    mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<ControlsButtonMarker>)>,
    asset_server: Res<AssetServer>,
) {
    for interaction in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            let font = asset_server.load("embedded://mol/asset/fonts/FiraMono-Medium.ttf");
            add_controls_box(&mut commands, &font);
        }
    }
}

pub fn close_popup_on_esc(
    mut commands: Commands,
    key_input: Res<ButtonInput<KeyCode>>,
    popup_query: Query<Entity, With<PopupMarker>>,
) {
    if key_input.pressed(KeyCode::Escape) {
        despawn_all_entities(&mut commands, &popup_query);
    }
}

pub fn update_ui_for_scene(
    scene: ResMut<MolScene>,
    mut mol_name_label: Query<&mut Text, With<MolNameMarker>>,
    assets: Res<Assets<Mol2Molecule>>,
) {
    if let Ok(mut label) = mol_name_label.get_single_mut() {
        match &scene.content {
            MolSceneContent::Empty => {
                label.sections[0].value = "Empty".to_string();
            }
            MolSceneContent::Mol2 { handle, .. } => {
                if let Some(mol) = assets.get(handle) {
                    label.sections[0].value = mol.name.to_string();
                }
                // don't do anything for other states
            }
        }
    }
}

#[allow(clippy::type_complexity)]
pub fn file_example_button_handler(
    mut interaction_query: Query<
        (&Interaction, &MolExampleFile),
        (Changed<Interaction>, With<MolExampleFile>),
    >,
    asset_server: Res<AssetServer>,
    mut scene: ResMut<MolScene>,
) {
    for (interaction, file) in &mut interaction_query {
        if interaction == &Interaction::Pressed {
            load_example_file(&asset_server, &mut scene, file);
        }
    }
}
