use crate::events::{Action, PlayerAction};
use bevy::prelude::*;
use rustc_hash::FxHashMap;

// Label components
#[derive(Component)]
pub struct Player;
#[derive(Component)]
pub struct Mob;
#[derive(Component)]
pub struct Eye;
#[derive(Component)]
pub struct MainCamera;
#[derive(Component)]
pub struct Dash;
#[derive(Component)]
pub struct Cell;

// Player and mob components
#[derive(Component)]
pub struct Health(pub i64);
#[derive(Component)]
pub struct Energy(pub i64);
#[derive(Component)]
pub struct Experience(pub i64);
#[derive(Component)]
pub struct MovementSpeed(pub f32);

// Ability components
#[derive(PartialEq, Clone, Copy, Component)]
pub enum CastAbility {
    Dash,
    Shoot,
    EyeMobZap,
}

#[derive(PartialEq, Clone, Copy, Component)]
pub enum ChannelAbility {
    Laser,
    EyeMobTarget,
}

#[derive(Component)]
pub struct Charges(pub i64);
#[derive(Component)]
pub struct MaxCharges(pub i64);
#[derive(Component)]
pub struct CastTime(pub f32);
#[derive(Component)]
pub struct CastTimer(pub Timer);
#[derive(Component)]
pub struct ChannelTime(pub f32);
#[derive(Component)]
pub struct ChannelTimer(pub Timer);
#[derive(Component)]
pub struct Cooldown(pub f32);
#[derive(Component)]
pub struct CooldownTimer(pub Timer);
#[derive(Clone, Copy, Component)]
pub struct LaserEntity(pub Entity);

#[derive(PartialEq, Clone, Copy, Component)]
pub struct Coords(pub Vec3);

#[derive(PartialEq, Component)]
pub enum CharState {
    Casting((CastAbility, Coords)),

    // Coords marks the targeted destination of the
    // unit (player or mob) and if the pathing system
    // has run its path is saved in the vec with
    // move-tile coords.
    Moving(Coords, Option<std::collections::VecDeque<(i32, i32)>>),
    Channeling((ChannelAbility, Coords)),
    Idle,
}

impl CharState {
    pub fn can_cast(&self) -> bool {
        match self {
            Self::Casting(_) => false,
            Self::Moving(..) => true,
            Self::Channeling(_) => true,
            Self::Idle => true,
        }
    }

    pub fn can_move(&self) -> bool {
        match self {
            Self::Casting(_) => false,
            Self::Moving(..) => true,
            Self::Channeling(_) => false,
            Self::Idle => true,
        }
    }
}

impl From<PlayerAction> for CharState {
    fn from(action: PlayerAction) -> Self {
        match action.action {
            Action::Move => CharState::Moving(Coords(action.mouse_coords), None),
            Action::Cast(ability) => CharState::Casting((ability, Coords(action.mouse_coords))),
            Action::Channel(ability) => {
                CharState::Channeling((ability, Coords(action.mouse_coords)))
            }
        }
    }
}

// Map keyboard and mouse into player actions
#[derive(Component)]
pub struct UserControls {
    pub mouse: FxHashMap<MouseButton, Action>,
    pub keyboard: FxHashMap<KeyCode, Action>,
}

impl UserControls {
    pub fn new() -> Self {
        let mut mouse = FxHashMap::default();
        let mut keyboard = FxHashMap::default();

        mouse.insert(MouseButton::Right, Action::Move);
        keyboard.insert(KeyCode::Q, Action::Cast(CastAbility::Dash));
        keyboard.insert(KeyCode::W, Action::Cast(CastAbility::Shoot));
        keyboard.insert(KeyCode::E, Action::Channel(ChannelAbility::Laser));

        Self { mouse, keyboard }
    }
}
