use bevy::prelude::*;
use crate::components::Velocity;

/// System to update entity positions based on their velocity in 3D space
pub fn movement(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query {
        // Apply velocity to position (scaled by delta time)
        let delta = time.delta_secs();
        
        // Update all three axes of movement
        transform.translation.x += velocity.x * delta;
        transform.translation.y += velocity.y * delta; // For jumping/falling (not used yet)
        transform.translation.z += velocity.z * delta;
    }
} 