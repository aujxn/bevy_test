use crate::components::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

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
    pub fn new(assest_server: &Res<AssetServer>) -> Self {
        let texture_handle = assest_server.load("player.png");
        Self {
            player: Player,
            health: Health(100),
            energy: Energy(100),
            experience: Experience(0),
            movement_speed: MovementSpeed(300.0),
            player_state: CharState::Idle,
            sprite: SpriteBundle {
                texture: texture_handle,
                transform: Transform::from_xyz(0.0, -50.0, 1.0).with_scale(Vec3::splat(0.25)),
                ..Default::default()
            },
        }
    }
}

#[derive(Bundle)]
pub struct MobBundle {
    mob_type: Eye,
    mob: Mob,
    health: Health,
    energy: Energy,
    movement_speed: MovementSpeed,
    mob_state: CharState,
    #[bundle]
    sprite: SpriteBundle,
    laser_entity: LaserEntity,
    cast_time: CastTime,
    cast_timer: CastTimer,
    channel_time: ChannelTime,
    channel_timer: ChannelTimer,
}

impl MobBundle {
    pub fn new(assest_server: &Res<AssetServer>, commands: &mut Commands) -> Self {
        let drawmode = DrawMode::Stroke(StrokeMode::new(Color::RED, 1.0));
        let laser_entity = LaserEntity(
            commands
                .spawn_bundle(GeometryBuilder::build_as(
                    &shapes::Line(Vec2::new(0.0, 0.0), Vec2::new(0.0, 100.0)),
                    drawmode,
                    Transform::default(),
                ))
                .id(),
        );
        let texture_handle = assest_server.load("eye.png");

        let channel_time = ChannelTime(3.0);
        let cast_time = CastTime(1.0);
        let mut channel_timer = ChannelTimer(Timer::from_seconds(channel_time.0, true));
        channel_timer.0.pause();
        let mut cast_timer = CastTimer(Timer::from_seconds(cast_time.0, false));
        cast_timer.0.pause();

        Self {
            mob_type: Eye,
            mob: Mob,
            health: Health(100),
            energy: Energy(100),
            movement_speed: MovementSpeed(300.0),
            mob_state: CharState::Idle,
            sprite: SpriteBundle {
                texture: texture_handle,
                transform: Transform::from_xyz(100.0, 100.0, 1.0),
                sprite: Sprite {
                    custom_size: Some(Vec2::splat(100.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
            laser_entity,
            channel_time,
            channel_timer,
            cast_time,
            cast_timer,
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
