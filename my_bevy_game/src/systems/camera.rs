use bevy::prelude::*;
use crate::components::Player;

/// System to make the camera follow the player
/// Currently just a skeleton - not active in the base implementation
pub fn camera_follow(
    _player_query: Query<&Transform, With<Player>>,
    _camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    // This is a placeholder for future camera follow logic
    // For now, we're using a fixed camera
    
    // Uncomment and adapt this when you want the camera to follow the player:
    /*
    if let (Ok(player_transform), Ok(mut camera_transform)) = (_player_query.get_single(), _camera_query.get_single_mut()) {
        // Follow with some smoothing or offset if desired
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
        // Keep the camera's z position
    }
    */
} 