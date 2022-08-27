use bevy::prelude::*;
use inkling::{InklingError, error::variable::VariableError};

use crate::{
    audio::AudioSpiritVolume,
    ink::{
        ink_asset::InkAsset,
        ink_story::{InkStory, StoryEvent},
    },
    level::ActivationEvent,
    loading_state::LoadedAssets,
    states::{GameMode, States},
    theme::*,
};

pub struct InteractiveNarrativePlugin;

impl Plugin for InteractiveNarrativePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetCurrentKnotEvent>()
            .add_system(set_current_knot)
            .add_system_set(
                SystemSet::on_enter(States::InGame)
                    .with_system(start_narrative),
            )
            .add_system_set(
                SystemSet::on_exit(States::InGame)
                    .with_system(clear_narrative_root),
            )
            .add_system_set(
                SystemSet::on_update(GameMode::Conversation)
                    .with_system(display_current_narrative)
                    .with_system(button_system),
            );
    }
}

#[derive(Component)]
#[component(storage = "SparseSet")]
struct NarrativeDisplayRoot;

#[derive(Component)]
struct NarrativeChoiceButton {
    choice: usize,
}

pub struct SetCurrentKnotEvent(pub Option<String>);

fn set_current_knot(
    mut event_reader: EventReader<SetCurrentKnotEvent>,
    mut story: Option<ResMut<InkStory>>,
    mut event_writer: EventWriter<StoryEvent>,
    mut game_mode: ResMut<State<GameMode>>,
) {
    let event = event_reader.iter().last();
    if let (Some(story), Some(SetCurrentKnotEvent(target_knot))) =
        (&mut story, event)
    {
        if let Some(target_knot) = target_knot {
            bevy::log::info!("Setting story knot {}", &target_knot);
            story.move_to(target_knot, None);
            story.resume_story_with_event(&mut event_writer);
            game_mode.set(GameMode::Conversation);
        } else {
            bevy::log::info!("No story knot - setting to exploration mode");
            game_mode.set(GameMode::Exploration);
        }
    }
}

fn start_narrative(
    mut commands: Commands,
    ink_assets: Res<Assets<InkAsset>>,
    ink_story: Option<Res<InkStory>>,
    handles: Res<LoadedAssets>,
    mut event_writer: EventWriter<StoryEvent>,
) {
    if ink_story.is_some() {
        return;
    }
    if let Some(mut story) = InkStory::new(&handles.test_ink, &ink_assets) {
        story.resume_story_with_event(&mut event_writer);
        commands.insert_resource(story);
        bevy::log::info!("Loaded story");
    } else {
        bevy::log::error!("Couldn't load ink");
    }
}

fn clear_narrative_root(
    mut commands: Commands,
    narrative_root: Query<Entity, With<NarrativeDisplayRoot>>,
) {
    for entity in narrative_root.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn display_current_narrative(
    mut commands: Commands,
    mut events: EventReader<StoryEvent>,
    narrative_root: Query<Entity, With<NarrativeDisplayRoot>>,
    assets: Res<LoadedAssets>,
    mut audio_spirit_volume: ResMut<AudioSpiritVolume>,
    mut game_mode: ResMut<State<GameMode>>,
    mut state: ResMut<State<States>>,
    mut activation_event: EventWriter<ActivationEvent>,
    mut story: ResMut<InkStory>,
) {
    let event = events.iter().last();

    if let Some(event) = event {
        for entity in narrative_root.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let mut trigger_play = false;

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size {
                        height: Val::Auto,
                        width: Val::Percent(100.),
                    },
                    margin: UiRect::new(
                        Val::Percent(5.),
                        Val::Percent(5.),
                        Val::Auto,
                        Val::Percent(5.),
                    ),
                    padding: UiRect::all(Val::Px(10.)),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .insert(NarrativeDisplayRoot)
            .with_children(|parent| {
                for line in event.lines.iter() {
                    for tag in line.tags.iter() {
                        bevy::log::info!("Processing tag {}", &tag);
                        match tag.as_str() {
                            "start_audio" => {
                                audio_spirit_volume.0 = 1.;
                            }
                            "play" => {
                                if !trigger_play {
                                    trigger_play = true;
                                    game_mode.set(GameMode::Exploration);
                                }
                            }
                            _ => {
                                if tag.starts_with("activate:") {
                                    let target = tag.replace("activate:", "");
                                    activation_event
                                        .send(ActivationEvent(true, target));
                                } else if tag.starts_with("deactivate:") {
                                    let target = tag.replace("deactivate:", "");
                                    activation_event
                                        .send(ActivationEvent(false, target));
                                }
                            }
                        }
                    }
                    if line.text.starts_with("~") {
                        let line = line.text.replace("~", "");
                        let mut split = line.split("=");
                        let variable_name = split.next();
                        let variable_value = split.next();
                        if let (Some(variable_name), Some(variable_value)) =
                            (variable_name, variable_value)
                        {
                            let variable_name = variable_name.trim();
                            let variable_value = variable_value.trim();
                            if let Ok(variable) = story.get_variable(&variable_name) {
                                let result = match variable {
                                    inkling::Variable::Bool(_) => story.set_variable(&variable_name, variable_value == "true"),
                                    inkling::Variable::Float(_) => story.set_variable(&variable_name, variable_value.parse::<f32>().unwrap_or_default()),
                                    inkling::Variable::Int(_) => story.set_variable(&variable_name, variable_value.parse::<i32>().unwrap_or_default()),
                                    inkling::Variable::String(_) => story.set_variable(&variable_name, variable_value.trim_matches('"').to_owned()),
                                    _ => {Err(InklingError::VariableError(VariableError { variable: variable.clone(), kind: inkling::error::variable::VariableErrorKind::NonMatchingAssignment { other: inkling::Variable::from(variable_value) }}))}
                                };
                                if let Err(err) = result {
                                    bevy::log::error!(
                                        "Error parsing variable assignment {}",
                                        err
                                    );
                                } else {
                                    if let Ok(variable) = story.get_variable(&variable_name) {
                                        bevy::log::info!("Set variable: {} to {:?}", &variable_name, variable)
                                    }
                                }
                            }
                        }
                    } else if !trigger_play && line.text.trim() != "&nbsp;"{
                        parent.spawn_bundle(TextBundle::from_section(
                            &line.text,
                            TextStyle {
                                font: assets.font.clone(),
                                font_size: 26.0,
                                color: TEXT_COLOR,
                            },
                        ));
                    }
                }
                if !trigger_play {
                    match &event.prompt {
                        inkling::Prompt::Done => {
                            state.set(States::Menu).unwrap();
                        }
                        inkling::Prompt::Choice(choices) => {
                            for (index, choice) in choices.iter().enumerate() {
                                for tag in choice.tags.iter() {
                                    bevy::log::info!("Processing tag {}", &tag);
                                    match tag.as_str() {
                                        "play" => {
                                            if !trigger_play {
                                                trigger_play = true;
                                                game_mode
                                                    .set(GameMode::Exploration);
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                                if trigger_play {
                                    break;
                                } else {
                                    parent
                                        .spawn_bundle(ButtonBundle {
                                            style: Style {
                                                // center button
                                                margin: UiRect::all(Val::Px(
                                                    2.,
                                                )),
                                                padding: UiRect::all(Val::Px(
                                                    5.,
                                                )),
                                                // horizontally center child
                                                // text
                                                justify_content:
                                                    JustifyContent::Center,
                                                // vertically center child text
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            color: Color::rgb(0.2, 0.2, 0.2)
                                                .into(),
                                            ..default()
                                        })
                                        .insert(NarrativeChoiceButton {
                                            choice: index.to_owned(),
                                        })
                                        .with_children(|parent| {
                                            parent.spawn_bundle(
                                                TextBundle::from_section(
                                                    &choice.text,
                                                    TextStyle {
                                                        font: assets
                                                            .font
                                                            .clone(),
                                                        font_size: 20.0,
                                                        color: Color::rgb(
                                                            0.8, 0.8, 0.8,
                                                        ),
                                                    },
                                                ),
                                            );
                                        });
                                }
                            }
                        }
                    }
                }
            });
    }
}

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut UiColor,
            &Children,
            &NarrativeChoiceButton,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut event_writer: EventWriter<StoryEvent>,
    story: Option<ResMut<InkStory>>,
) {
    if let Some(mut story) = story {
        for (interaction, mut color, _children, choice) in
            &mut interaction_query
        {
            match *interaction {
                Interaction::Clicked => {
                    *color = Color::rgb(0.1, 0.1, 0.1).into();
                    story.make_choice(choice.choice.to_owned());
                    story.resume_story_with_event(&mut event_writer)
                }
                Interaction::Hovered => {
                    *color = Color::rgb(0.4, 0.4, 0.4).into();
                }
                Interaction::None => {
                    *color = Color::rgb(0.2, 0.2, 0.2).into();
                }
            }
        }
    }
}
