use crate::components::*;
use crate::events::PlayerAction;
use bevy::prelude::*;

pub fn player_action_system(
    mut action_event: EventReader<(PlayerAction, Vec3)>,
    mut query: Query<&mut CharState, With<Player>>,
) {
    if let Ok(mut state) = query.single_mut() {
        for (action, mouse_coords) in action_event.iter() {
            match *action {
                PlayerAction::Move => *state = CharState::Moving(*mouse_coords),
                PlayerAction::Dash => (),
                PlayerAction::Bow => (),
            }
        }
    }
}
