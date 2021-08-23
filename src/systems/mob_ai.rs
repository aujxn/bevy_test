use crate::components::{ChannelAbility, CharState, Coords, Eye, Player};
use bevy::prelude::*;

pub fn eye_mob_ai_system(
    mut query: QuerySet<(
        Query<(&Transform, &mut CharState), With<Eye>>,
        Query<&Transform, With<Player>>,
    )>,
) {
    let player_coords = query.q1().single().unwrap().translation;

    let range = 800.0;
    let buffer = 250.0;

    for (mob_transform, mut mob_state) in query.q0_mut().iter_mut() {
        match *mob_state {
            CharState::Moving(destination, _) => {
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
