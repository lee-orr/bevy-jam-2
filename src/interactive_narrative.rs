use bevy::{ecs::event, prelude::*};

use crate::{
    ink::{
        ink_asset::InkAsset,
        ink_story::{InkStory, StoryEvent},
    },
    loading_state::LoadedAssets,
    states::States,
};

pub struct InteractiveNarrativePlugin;

impl Plugin for InteractiveNarrativePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(States::InGame).with_system(start_narrative),
        )
        .add_system_set(
            SystemSet::on_update(States::InGame)
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
    choice: usize
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

fn display_current_narrative(
    mut commands: Commands,
    mut events: EventReader<StoryEvent>,
    narrative_root: Query<Entity, With<NarrativeDisplayRoot>>,
    assets: Res<LoadedAssets>,
) {
    let event = events.iter().last();

    if let Some(event) = event {
        for entity in narrative_root.iter() {
            commands.entity(entity).despawn_recursive();
        }

        commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    margin: UiRect::new(
                        Val::Percent(5.),
                        Val::Auto,
                        Val::Auto,
                        Val::Percent(5.),
                    ),
                    padding: UiRect::all(Val::Px(10.)),
                    justify_content: JustifyContent::FlexEnd,
                    align_items: AlignItems::FlexStart,
                    flex_direction: FlexDirection::ColumnReverse,
                    ..default()
                },
                color: Color::rgb(0.15, 0.15, 0.15).into(),
                ..default()
            })
            .insert(NarrativeDisplayRoot)
            .with_children(|parent| {
                for line in event.lines.iter() {
                    parent.spawn_bundle(TextBundle::from_section(
                        &line.text,
                        TextStyle {
                            font: assets.font.clone(),
                            font_size: 26.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                }
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
                            parent.spawn_bundle(ButtonBundle {
                                style: Style {
                                    // center button
                                    margin: UiRect::all(Val::Px(2.)),
                                    padding: UiRect::all(Val::Px(5.)),
                                    // horizontally center child text
                                    justify_content: JustifyContent::Center,
                                    // vertically center child text
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                color: Color::rgb(0.2, 0.2, 0.2).into(),
                                ..default()
                            }
                            )
                            .insert(NarrativeChoiceButton { choice: index.to_owned() }).with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                &choice.text,
                                TextStyle {
                                    font: assets.font.clone(),
                                    font_size: 20.0,
                                    color: Color::rgb(0.8, 0.8, 0.8),
                                },
                            )); });
                        }
                    }
                }
            });
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children, &NarrativeChoiceButton),
        (Changed<Interaction>, With<Button>),
    >,
    mut event_writer: EventWriter<StoryEvent>,
    mut story: Option<ResMut<InkStory>>
) {
    if let Some(mut story) = story {
    for (interaction, mut color, children, choice) in &mut interaction_query {
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