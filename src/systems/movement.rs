use crate::components::*;
use bevy::prelude::*;

/// System that moves the player and mobs. For anything that has
/// the `Moving` state, update their position based on their speed.
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut CharState, &MovementSpeed, &mut Transform)>,
) {
    let delta_seconds = time.delta_seconds();

    for (mut state, speed, mut transform) in query.iter_mut() {
        if let CharState::Moving(destination) = *state {
            let direction = destination.0 - transform.translation;
            // if we are far away from the destination then keep going
            if direction.length() > 5.0 {
                transform.translation += speed.0 * delta_seconds * direction.normalize();
            } else {
                *state = CharState::Idle;
            }
        }
    }
}
