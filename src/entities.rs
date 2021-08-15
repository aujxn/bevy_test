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
pub struct DashBundle {
    ability: Dash,
    ability_class: CastAbility,
    cooldown: Cooldown,
    cooldown_timer: CooldownTimer,
    charges: Charges,
    max_charges: MaxCharges,
    cast_time: CastTime,
    cast_timer: CastTimer,
}

impl DashBundle {
    pub fn new() -> Self {
        let ability = Dash;
        let ability_class = CastAbility::Dash;
        let cooldown = Cooldown(3.0);
        let cast_time = CastTime(0.1);
        let charges = Charges(3);
        let max_charges = MaxCharges(3);

        let mut cooldown_timer = CooldownTimer(Timer::from_seconds(cooldown.0, true));
        cooldown_timer.0.pause();

        let mut cast_timer = CastTimer(Timer::from_seconds(cast_time.0, false));
        cast_timer.0.pause();

        Self {
            ability,
            ability_class,
            cooldown,
            cooldown_timer,
            charges,
            max_charges,
            cast_time,
            cast_timer,
        }
    }
}
