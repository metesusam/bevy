use bevy::prelude::*;
use crate::components::{Player, Velocity};

/// System to set up the game world, including camera, player, lighting, and ground
pub fn setup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn a 3D camera
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 5.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add directional light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 10000.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Add ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    // Create ground plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Load the GLB model
    let glb_handle = asset_server.load("models/Untitled.glb");
    
    // Spawn the player using the GLB model
    commands.spawn((
        SceneRoot(glb_handle),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Player,
        Velocity::default(),
    ));
} 