use bevy::prelude::*;
use crate::components::Player;

/// System to make the camera follow the player in 3D space
pub fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    // Get player transform and camera transform
    if let (Ok(player_transform), Ok(mut camera_transform)) = (player_query.get_single(), camera_query.get_single_mut()) {
        // Target position for the camera
        let target_position = Vec3::new(
            player_transform.translation.x,
            player_transform.translation.y + 5.0, // Higher than the player
            player_transform.translation.z + 10.0, // Behind the player
        );
        
        // Smooth camera follow with lerp
        camera_transform.translation = camera_transform.translation.lerp(target_position, 0.1);
        
        // Make the camera look at the player
        let forward = (player_transform.translation - camera_transform.translation).normalize();
        camera_transform.look_to(forward, Vec3::Y);
    }
} 