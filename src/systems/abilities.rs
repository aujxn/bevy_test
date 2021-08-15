use crate::components::*;
use crate::events::*;
use bevy::prelude::*;

pub fn charges_cooldown_system(
    time: Res<Time>,
    mut query: Query<(&mut CooldownTimer, &Cooldown, &mut Charges, &MaxCharges)>,
) {
    let delta = time.delta();
    for (mut timer, cooldown, mut charges, max_charges) in query.iter_mut() {
        if charges.0 < max_charges.0 {
            timer.0.tick(delta);
            if timer.0.finished() {
                charges.0 += 1;
                if charges.0 == max_charges.0 {
                    timer.0 = Timer::from_seconds(cooldown.0, true);
                    timer.0.pause();
                }
            }
        }
    }
}

pub fn dash(
    time: Res<Time>,
    mut player_query: Query<(&mut CharState, &mut Transform), With<Player>>,
    mut dash_query: Query<
        (&mut Charges, &mut CooldownTimer, &mut CastTimer, &CastTime),
        With<Dash>,
    >,
) {
    let dash_range = 400.0;
    if let Ok((mut state, mut transform)) = player_query.single_mut() {
        if let CharState::Casting(ability) = *state {
            if ability.0 == CastAbility::Dash {
                if let Ok((mut dash_charges, mut cooldown_timer, mut cast_timer, cast_time)) =
                    dash_query.single_mut()
                {
                    // if we don't have any charges then we cannot cast, so just
                    // keep moving towards the dash location
                    if dash_charges.0 == 0 {
                        *state = CharState::Moving(ability.1);
                    // if the cast timer is paused then we need to start casting
                    } else if cast_timer.0.paused() && dash_charges.0 > 0 {
                        dash_charges.0 -= 1;
                        cast_timer.0.unpause();
                        // cooldown timer might have been paused if we are at
                        // max charges so start that since we consumed one.
                        if cooldown_timer.0.paused() {
                            cooldown_timer.0.unpause();
                        }
                    } else if cast_timer.0.finished() {
                        // reset the cast timer for next time when done casting
                        cast_timer.0 = Timer::from_seconds(cast_time.0, false);
                        cast_timer.0.pause();
                        // perform the dash
                        let mouse_coords = ability.1 .0;
                        let direction = mouse_coords - transform.translation;
                        if direction.length() < dash_range {
                            transform.translation += direction;
                        } else {
                            transform.translation += dash_range * direction.normalize();
                        }
                        // keep moving toward dash destination
                        *state = CharState::Moving(ability.1);
                    // if the cast timer is running and not finished then continue
                    } else {
                        cast_timer.0.tick(time.delta());
                    }
                }
            }
        }
    }
}
