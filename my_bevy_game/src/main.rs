use bevy::prelude::*;

mod components;
mod systems;

fn main() {
    App::new()
        // Add default Bevy plugins
        .add_plugins(DefaultPlugins)
        // Window setup
        .insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.3))) // Dark blue skybox
        // Setup system (runs once at startup)
        .add_systems(Startup, systems::setup)
        // Game systems that run every frame
        .add_systems(Update, (
            systems::handle_input,
            systems::movement,
            systems::camera_follow, // Camera follow system is now enabled
        ))
        // Debug systems - uncomment as needed to troubleshoot model loading issues
        // .add_systems(Update, systems::debug_model_loading) // Temporarily disabled
        .add_systems(Update, systems::debug_player_position)
        .add_systems(Update, systems::debug_scene_hierarchy)
        .run();
}