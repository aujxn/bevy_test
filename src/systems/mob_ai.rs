use crate::components::{ChannelAbility, CharState, Coords, Eye, LaserEntity, Player};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn eye_mob_ai_system(
    mut query: QuerySet<(
        Query<(&Transform, &mut CharState), With<Eye>>,
        Query<&Transform, With<Player>>,
    )>,
    mut commands: Commands,
) {
    let player_coords = query.q1().single().unwrap().translation;
    let player_coords_vec2 = Vec2::new(player_coords.x, player_coords.y);

    let range = 800.0;
    let buffer = 250.0;

    for (mob_transform, mut mob_state) in query.q0_mut().iter_mut() {
        match *mob_state {
            CharState::Moving(destination, _) => {
                let mob_coords =
                    Vec2::new(mob_transform.translation.x, mob_transform.translation.y);
                // when the mob is in range
                if (mob_transform.translation - player_coords).length() < range - buffer {
                    *mob_state =
                        CharState::Channeling((ChannelAbility::Lazer, Coords(player_coords)));
                }
                // if the player has moved far get a new path towards them
                else if (destination.0 - mob_transform.translation).length() > 100.0 {
                    *mob_state = CharState::Moving(Coords(player_coords), None);
                }
            }
            CharState::Casting(_) => {}
            CharState::Channeling(_) => {
                // out of range now
                if (mob_transform.translation - player_coords).length() > range {
                    *mob_state = CharState::Moving(Coords(player_coords), None);
                }
            }
            CharState::Idle => {
                *mob_state = CharState::Moving(Coords(player_coords), None);
            }
        }
    }
}

pub fn eye_lazer(
    mut query: QuerySet<(
        Query<(&Transform, &CharState, &LaserEntity), With<Eye>>,
        Query<&Transform, With<Player>>,
        Query<&mut Transform>,
    )>,
) {
    let player_coords = query.q1().single().unwrap().translation;

    let list: Vec<(Transform, Entity)> = query
        .q0()
        .iter()
        .filter(|(_, mob_state, _)| match mob_state {
            CharState::Channeling((ChannelAbility::Lazer, _)) => true,
            _ => false,
        })
        .map(|(transform, _, laser)| (*transform, laser.0))
        .collect();
    for (mob_transform, laser) in list {
        if let Ok(mut transform) = query.q2_mut().get_mut(laser) {
            let from = Vec3::new(0.0, 100.0, 0.0);
            let to = player_coords - mob_transform.translation;
            let quat = Quat::from_rotation_arc(from.normalize(), to.normalize());

            let scale = Vec3::new(
                1.0,
                (player_coords - mob_transform.translation).length() / 100.0,
                1.0,
            );
            let translation = 0.5 * (player_coords + mob_transform.translation);

            transform.scale = scale;
            transform.rotation = quat;
            transform.translation = mob_transform.translation;
        }
    }
}
