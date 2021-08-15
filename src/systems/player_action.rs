use crate::components::*;
use crate::events::*;
use bevy::prelude::*;

/// System that waits for player actions from the input system.
/// Checks the player state to see if the action is allowed.
/// For example, if they player is currently casting then the
/// action is ignored.
pub fn player_action_system(
    mut action_event: EventReader<PlayerAction>,
    mut query: Query<&mut CharState, With<Player>>,
) {
    if let Ok(mut state) = query.single_mut() {
        for player_action in action_event.iter() {
            match player_action.action {
                Action::Move => {
                    if state.can_move() {
                        *state = CharState::from(*player_action)
                    }
                }
                Action::Cast(_) => {
                    if state.can_cast() {
                        *state = CharState::from(*player_action)
                    }
                }
                Action::Channel(_) => (),
            }
        }
    }
}
