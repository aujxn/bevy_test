use crate::components::*;
use bevy::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub struct PlayerAction {
    pub action: Action,
    pub mouse_coords: Vec3,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Action {
    Move,
    Cast(CastAbility),
    Channel(ChannelAbility),
}

impl PlayerAction {
    pub fn new(action: Action, mouse_coords: Vec3) -> Self {
        Self {
            action,
            mouse_coords,
        }
    }
}
