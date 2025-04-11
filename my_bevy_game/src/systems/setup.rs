use bevy::prelude::*;
use crate::components::{Player, Velocity};

/// System to set up the game world, including camera and player
pub fn setup(mut commands: Commands) {
    // Spawn a 2D camera
    commands.spawn(Camera2d::default());

    // Spawn the player as a blue square
    commands.spawn((
        Sprite {
            color: Color::srgb(0.1, 0.4, 0.8),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        Visibility::default(),
        Player,
        Velocity::default(),
    ));
} 