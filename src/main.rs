mod components;
mod entities;
mod events;
mod systems;

use crate::events::*;
use crate::systems::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 8 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup::setup_system.system())
        .add_system(player_action::player_action_system.system())
        .add_system(input::input_system.system())
        .add_system(movement::movement_system.system())
        .add_system(abilities::dash.system())
        .add_system(abilities::charges_cooldown_system.system())
        .add_system(mob_ai::eye_mob_ai_system.system())
        .add_event::<PlayerAction>()
        .run();
}
