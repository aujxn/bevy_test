use crate::components::*;
use bevy::prelude::*;
/// System that moves the player and mobs. For anything that has
/// the `Moving` state, update their position based on their speed.
pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&mut CharState, &MovementSpeed, &mut Transform)>,
    q_graph: Query<&TileGraph>,
) {
    let delta_seconds = time.delta_seconds();
    for (mut state, speed, mut transform) in query.iter_mut() {
        if let CharState::Moving(destination) = *state {
            if let Ok(graph) = q_graph.single() {
                if let Some(destination) = graph.path(
                    (transform.translation.x, transform.translation.y),
                    (destination.0.x, destination.0.y),
                    &vec![],
                ) {
                    let direction = destination - transform.translation;
                    transform.translation += speed.0 * delta_seconds * direction.normalize();
                } else {
                    *state = CharState::Idle;
                }
            }
        }
    }
}
