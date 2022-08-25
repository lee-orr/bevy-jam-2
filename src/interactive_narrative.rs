use bevy::{ecs::event, prelude::*};

use crate::{
    audio::AudioSpiritVolume,
    ink::{
        ink_asset::InkAsset,
        ink_story::{InkStory, StoryEvent},
    },
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
    if let (Some(story), Some(SetCurrentKnotEvent(Some(target_knot)))) =
        (&mut story, event)
    {
        bevy::log::info!("Setting story knot {}", &target_knot);
        story.move_to(target_knot, None);
        story.resume_story_with_event(&mut event_writer);
        game_mode.set(GameMode::Conversation);
    }
}

fn start_narrative(
    mut commands: Commands,
    ink_assets: Res<Assets<InkAsset>>,
    handles: Res<LoadedAssets>,
    mut event_writer: EventWriter<StoryEvent>,
) {
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
                                trigger_play = true;
                                game_mode.set(GameMode::Exploration);
                            }
                            _ => {}
                        }
                    }
                    if !trigger_play {
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
                            parent.spawn_bundle(TextBundle::from_section(
                                "The End!",
                                TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 15.0,
                                    color: Color::rgb(0.7, 0.7, 0.7),
                                },
                            ));
                        }
                        inkling::Prompt::Choice(choices) => {
                            for (index, choice) in choices.iter().enumerate() {
                                parent
                                    .spawn_bundle(ButtonBundle {
                                        style: Style {
                                            // center button
                                            margin: UiRect::all(Val::Px(2.)),
                                            padding: UiRect::all(Val::Px(5.)),
                                            // horizontally center child text
                                            justify_content:
                                                JustifyContent::Center,
                                            // vertically center child text
                                            align_items: AlignItems::Center,
                                            ..default()
                                        },
                                        color: Color::rgb(0.2, 0.2, 0.2).into(),
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
                                                    font: assets.font.clone(),
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
    mut story: Option<ResMut<InkStory>>,
) {
    if let Some(mut story) = story {
        for (interaction, mut color, children, choice) in &mut interaction_query
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
