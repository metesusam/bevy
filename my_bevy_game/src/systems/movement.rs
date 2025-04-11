use bevy::prelude::*;
use crate::components::Velocity;

/// System to update entity positions based on their velocity
pub fn movement(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query {
        // Apply velocity to position (scaled by delta time)
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
} 