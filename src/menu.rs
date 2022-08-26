use crate::level::SetLevelEvent;
use crate::theme::*;
use bevy::prelude::*;

use crate::{loading_state::LoadedAssets, states::States};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(
            SystemSet::on_enter(States::Menu).with_system(setup),
        )
        .add_system_set(
            SystemSet::on_update(States::Menu).with_system(button_system),
        )
        .add_system_set(SystemSet::on_exit(States::Menu).with_system(cleanup))
        .add_system_set(
            SystemSet::on_enter(States::LoadingLevel)
                .with_system(display_loading),
        )
        .add_system_set(
            SystemSet::on_exit(States::LoadingLevel).with_system(cleanup),
        );
    }
}

fn setup(mut commands: Commands, assets: Res<LoadedAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                },
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: BACKGROUIND_COLOR.into(),
            ..default()
        })
        .with_children(|p| {
            p.spawn_bundle(TextBundle::from_section(
                LAUNCHER_TITLE,
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 150.,
                    color: TEXT_COLOR,
                },
            ));
            p.spawn_bundle(ButtonBundle {
                style: Style {
                    padding: UiRect::all(Val::Px(20.)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                color: NORMAL_BUTTON.into(),
                ..default()
            })
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle::from_section(
                    "Start Game",
                    TextStyle {
                        font: assets.font.clone(),
                        font_size: 40.0,
                        color: TEXT_COLOR,
                    },
                ));
            });
        });
}

fn display_loading(mut commands: Commands, assets: Res<LoadedAssets>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                // center button
                margin: UiRect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..default()
            },
            color: BACKGROUIND_COLOR.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Loading level...",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 40.0,
                    color: TEXT_COLOR,
                },
            ));
        });
}

fn cleanup(mut commands: Commands, q: Query<Entity, With<Node>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut event_writer: EventWriter<SetLevelEvent>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                event_writer.send(SetLevelEvent("Level_1".into()))
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
