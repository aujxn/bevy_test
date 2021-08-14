use bevy::prelude::*;

// Label components
pub struct Player;
pub struct Mob;
pub struct Ability;
pub struct Projectile;
pub struct MainCamera;

// Player and mob components
pub struct Health(pub i64);
pub struct Energy(pub i64);
pub struct Experience(pub i64);
pub struct MovementSpeed(pub f32);

// Ability components
pub struct Cooldown(pub f32);
pub struct Charges(pub i64);
pub struct MaxCharges(pub i64);
pub struct CastTime(pub f32);

#[derive(PartialEq)]
pub enum CharState {
    // Seconds left in a cast
    Casting(f32),
    // Destination coords of movement
    Moving(Vec3),
    Channeling,
    Idle,
}
