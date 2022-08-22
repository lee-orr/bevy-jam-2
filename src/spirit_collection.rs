use bevy::prelude::*;
use heron::{Velocity, RigidBody, CollisionLayers};
use leafwing_input_manager::prelude::*;

use crate::{
    player::{Action, PlayerControl},
    spirit::Spirit,
    states::States,
};

pub struct SpiritCollection;

impl Plugin for SpiritCollection {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(States::InGame)
                .with_system(collect_nearby_spirits)
                .with_system(collecting_spirits)
                .with_system(cleanup_collected),
        );
    }
}

#[derive(Component)]
pub struct Collecting;

#[derive(Component)]
pub struct Collected;

pub fn collect_nearby_spirits(
    mut commands: Commands,
    spirits: Query<
        (Entity, &Transform),
        (With<Spirit>, Without<Collecting>, Without<Collected>),
    >,
    players: Query<(&ActionState<Action>, &Transform), With<PlayerControl>>,
) {
    let target = players.get_single();

    if let Ok((action_state, player)) = target {
        if action_state.just_pressed(Action::Collect) {
            for (entity, spirit) in spirits.iter() {
                if (spirit.translation - player.translation).length() < 30. {
                    commands.entity(entity).insert(Collecting).insert(CollisionLayers::none());
                }
            }
        }
    }
}

pub fn collecting_spirits(
    mut commands: Commands,
    mut spirits: Query<
        (Entity, &Transform, &mut Velocity),
        (With<Spirit>, With<Collecting>),
    >,
    players: Query<&Transform, (Without<Spirit>, With<PlayerControl>)>,
) {
    let target = players.get_single();

    if let Ok(player) = target {
        for (entity, transform, mut velocity) in spirits.iter_mut() {
            let distance = player.translation - transform.translation;
            if distance.length() < 3. {
                commands
                    .entity(entity)
                    .remove::<Collecting>()
                    .insert(Collected);
            } else {
                velocity.linear += distance / 10.;
            }
        }
    }
}

pub fn cleanup_collected(
    mut commands: Commands,
    spirits: Query<Entity, With<Collected>>,
) {
    for entity in spirits.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
