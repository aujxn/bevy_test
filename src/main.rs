mod components;
mod entities;
mod events;
mod systems;

use crate::events::*;
use crate::systems::*;
use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup::setup_system.system())
        .add_system(player_action::player_action_system.system())
        .add_system(input::input_system.system())
        .add_system(movement::movement_system.system())
        .add_event::<(PlayerAction, Vec3)>()
        .run();
}
