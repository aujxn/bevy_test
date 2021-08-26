use crate::components::{
    CastAbility, CastTime, CastTimer, ChannelAbility, ChannelTime, ChannelTimer, CharState, Coords,
    Eye, LaserEntity, Player,
};
use bevy::prelude::*;

/// System that controls the AI for the first mob. AI for mob right now:
/// 1. If further away than (range - buffer) from player move towards them
/// 2. Once within (range - buffer) from player change to targeting, indicated
/// with red line from mob to player
/// 3. After channeling for (channel_time) seconds, switch to casting. If the
/// player moves out of range while targeting go back to 1.
/// 4. Once casting the target laser locks in place for one second then flashes
/// (TODO flashing not done yet)
/// 5. If still in range start targeting again otherwise go to 1.
pub fn eye_mob_ai_system(
    mut query: QuerySet<(
        Query<
            (
                &Transform,
                &mut CharState,
                &ChannelTime,
                &mut ChannelTimer,
                &CastTime,
                &mut CastTimer,
                &LaserEntity,
            ),
            With<Eye>,
        >,
        Query<&Transform, With<Player>>,
    )>,
    time: Res<Time>,
    mut laser_event_writer: EventWriter<LaserEvent>,
) {
    let player_coords = query.q1().single().unwrap().translation;

    let range = 800.0;
    let buffer = 250.0;

    for (
        mob_transform,
        mut mob_state,
        channel_time,
        mut channel_timer,
        cast_time,
        mut cast_timer,
        laser_entity,
    ) in query.q0_mut().iter_mut()
    {
        match *mob_state {
            CharState::Moving(destination, _) => {
                // when the mob is in range
                if (mob_transform.translation - player_coords).length() < range - buffer {
                    // start the channel timer and change the mob state to channeling the laser
                    channel_timer.0 = Timer::from_seconds(channel_time.0, false);
                    *mob_state = CharState::Channeling((
                        ChannelAbility::EyeMobTarget,
                        Coords(player_coords),
                    ));
                }
                // if the player has moved far get a new path towards them
                else if (destination.0 - mob_transform.translation).length() > 100.0 {
                    *mob_state = CharState::Moving(Coords(player_coords), None);
                }
            }
            CharState::Casting(_) => {
                cast_timer.0.tick(time.delta());
                if cast_timer.0.finished() {
                    //TODO zap stuff
                    // after zap if mob is out of range now then move to player
                    if (mob_transform.translation - player_coords).length() > range {
                        *mob_state = CharState::Moving(Coords(player_coords), None);
                        laser_event_writer.send(LaserEvent::Off(*laser_entity));
                    // otherwise go back to targeting
                    } else {
                        channel_timer.0 = Timer::from_seconds(channel_time.0, false);
                        *mob_state = CharState::Channeling((
                            ChannelAbility::EyeMobTarget,
                            Coords(player_coords),
                        ));
                    }
                }
            }
            CharState::Channeling(_) => {
                // out of range now
                if (mob_transform.translation - player_coords).length() > range {
                    *mob_state = CharState::Moving(Coords(player_coords), None);
                    laser_event_writer.send(LaserEvent::Off(*laser_entity));
                } else {
                    channel_timer.0.tick(time.delta());
                    if channel_timer.0.finished() {
                        cast_timer.0 = Timer::from_seconds(cast_time.0, false);
                        *mob_state =
                            CharState::Casting((CastAbility::EyeMobZap, Coords(player_coords)));
                    }
                    laser_event_writer.send(LaserEvent::On(
                        *laser_entity,
                        Coords(mob_transform.translation),
                        Coords(player_coords),
                    ));
                }
            }
            CharState::Idle => {
                *mob_state = CharState::Moving(Coords(player_coords), None);
            }
        }
    }
}

pub enum LaserEvent {
    On(LaserEntity, Coords, Coords),
    Off(LaserEntity),
}

/// This system updates the tracking laser from the eye mob to the player. The eye mob system
/// sends LaserEvents when the mob changes between moving and channeling/targeting.
pub fn eye_laser(
    mut query: Query<(&mut Transform, &mut Visible)>,
    mut laser_event_reader: EventReader<LaserEvent>,
) {
    for laser_event in laser_event_reader.iter() {
        match laser_event {
            LaserEvent::On(LaserEntity(id), Coords(mob_coords), Coords(player_coords)) => {
                if let Ok((mut transform, mut visible)) = query.get_mut(*id) {
                    let from = Vec3::new(0.0, 100.0, 0.0);
                    let to = *player_coords - *mob_coords;
                    let quat = Quat::from_rotation_arc(from.normalize(), to.normalize());

                    let scale =
                        Vec3::new(1.0, (*player_coords - *mob_coords).length() / 100.0, 1.0);

                    transform.scale = scale;
                    transform.rotation = quat;
                    transform.translation = *mob_coords;
                    *visible = Visible {
                        is_visible: true,
                        is_transparent: false,
                    };
                }
            }
            LaserEvent::Off(LaserEntity(id)) => {
                if let Ok((_transform, mut visible)) = query.get_mut(*id) {
                    *visible = Visible {
                        is_visible: false,
                        is_transparent: false,
                    };
                }
            }
        }
    }
}
