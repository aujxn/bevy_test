use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    health: Health,
    energy: Energy,
    experience: Experience,
    movement_speed: MovementSpeed,
    player_state: CharState,
    #[bundle]
    sprite: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(
        assest_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let texture_handle = assest_server.load("player.png");
        Self {
            player: Player,
            health: Health(100),
            energy: Energy(100),
            experience: Experience(0),
            movement_speed: MovementSpeed(300.0),
            player_state: CharState::Idle,
            sprite: SpriteBundle {
                material: materials.add(texture_handle.into()),
                transform: Transform::from_xyz(0.0, -50.0, 1.0),
                sprite: Sprite::new(Vec2::new(160.0, 120.0)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct MobBundle {
    mob: Mob,
    health: Health,
    energy: Energy,
    movement_speed: MovementSpeed,
    mob_state: CharState,
    #[bundle]
    sprite: SpriteBundle,
}

impl MobBundle {
    pub fn new(
        assest_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        let texture_handle = assest_server.load("eye.png");
        Self {
            mob: Mob,
            health: Health(100),
            energy: Energy(100),
            movement_speed: MovementSpeed(300.0),
            mob_state: CharState::Idle,
            sprite: SpriteBundle {
                material: materials.add(texture_handle.into()),
                transform: Transform::from_xyz(100.0, 100.0, 1.0),
                sprite: Sprite::new(Vec2::new(100.0, 100.0)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct AbilityBundle {
    ability: Ability,
    cooldown: Cooldown,
    charges: Charges,
    max_charges: MaxCharges,
    cast_time: CastTime,
}
