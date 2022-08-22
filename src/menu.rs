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

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn setup(mut commands: Commands, assets: Res<LoadedAssets>) {
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: UiRect::all(Val::Auto),
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
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
            ));
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
            color: NORMAL_BUTTON.into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle::from_section(
                "Loading level...",
                TextStyle {
                    font: assets.font.clone(),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
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
    mut app_state: ResMut<State<States>>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
                app_state.set(States::LoadingLevel).unwrap();
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
