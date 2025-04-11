use bevy::prelude::*;
use crate::components::{Player, Velocity};

/// System to handle player input (WASD movement)
pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    // Movement speed
    const SPEED: f32 = 200.0;

    // Get player velocity
    if let Ok(mut velocity) = query.get_single_mut() {
        // Reset velocity
        velocity.x = 0.0;
        velocity.y = 0.0;

        // Apply movement based on WASD keys
        if keys.pressed(KeyCode::KeyW) {
            velocity.y += SPEED;
        }
        if keys.pressed(KeyCode::KeyS) {
            velocity.y -= SPEED;
        }
        if keys.pressed(KeyCode::KeyA) {
            velocity.x -= SPEED;
        }
        if keys.pressed(KeyCode::KeyD) {
            velocity.x += SPEED;
        }
    }
} 