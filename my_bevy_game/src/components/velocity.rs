use bevy::prelude::*;

/// Component for entity movement velocity in 3D space
#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,  // Lateral movement (left-right)
    pub y: f32,  // Vertical movement (up-down)
    pub z: f32,  // Forward-backward movement
} 