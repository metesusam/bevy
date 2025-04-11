use bevy::prelude::*;
use crate::components::{Player, Velocity};

/// System to handle player input (WASD movement) in 3D space
pub fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    // Movement speed
    const SPEED: f32 = 5.0; // Adjusted for 3D scale

    // Get player velocity
    if let Ok(mut velocity) = query.get_single_mut() {
        // Reset velocity
        velocity.x = 0.0;
        velocity.z = 0.0;
        // Note: we're not resetting Y, as that would be for vertical movement (jumping/falling)

        // Apply movement based on WASD keys or arrow keys
        // W/S or Up/Down now move along the Z axis (forward/backward)
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            velocity.z -= SPEED; // Negative Z is forward in Bevy's coordinate system
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            velocity.z += SPEED; // Positive Z is backward
        }
        // A/D or Left/Right continue to move along the X axis (left/right)
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            velocity.x -= SPEED; // Negative X is left
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            velocity.x += SPEED; // Positive X is right
        }
        
        // Simple jump mechanism (Space key)
        if keys.just_pressed(KeyCode::Space) {
            velocity.y = 8.0; // Jump impulse
        }
        
        // Apply simple gravity
        velocity.y -= 0.2; // Constant gravity force
    }
} 