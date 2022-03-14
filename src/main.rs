mod components;
mod entities;
mod events;
mod resources;
mod systems;

use crate::events::*;
use crate::systems::*;
use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(WorldInspectorPlugin::new())
        .add_plugin(ShapePlugin)
        .add_startup_system(setup::setup_system)
        .add_system(player_action::player_action_system)
        .add_system(input::input_system)
        .add_system(movement::movement_system)
        .add_system(abilities::dash)
        .add_system(abilities::charges_cooldown_system)
        .add_system(mob_ai::eye_mob_ai_system)
        .add_system(mob_ai::eye_laser)
        .add_event::<crate::systems::mob_ai::LaserEvent>()
        .add_event::<PlayerAction>()
        .run();
}
