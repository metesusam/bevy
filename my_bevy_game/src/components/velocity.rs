use bevy::prelude::*;

/// Component for entity movement velocity
#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
} 