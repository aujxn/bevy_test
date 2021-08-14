use crate::components::*;
use bevy::prelude::*;

pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut CharState, &MovementSpeed, &mut Transform)>,
) {
    let delta_seconds = time.delta_seconds();

    for (mut state, speed, mut transform) in query.iter_mut() {
        match *state {
            CharState::Moving(destination) => {
                let direction = destination - transform.translation;
                if direction.length() > 5.0 {
                    transform.translation += speed.0 * delta_seconds * direction.normalize();
                } else {
                    *state = CharState::Idle;
                }
            }
            CharState::Channeling => (),
            CharState::Casting(_) => (),
            CharState::Idle => (),
        }
    }
}
