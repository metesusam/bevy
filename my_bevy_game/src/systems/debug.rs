use bevy::prelude::*;
use bevy::asset::LoadState;
use crate::components::Player;

/// Debug system to check if the model asset is loading correctly
pub fn debug_model_loading(
    asset_server: Res<AssetServer>,
    player_query: Query<&SceneRoot, With<Player>>,
) {
    if let Ok(scene_root) = player_query.get_single() {
        // Check the load state of the model
        match asset_server.get_load_state(scene_root.0.id()) {
            Some(LoadState::NotLoaded) => println!("Model: Not loaded"),
            Some(LoadState::Loading) => println!("Model: Loading..."),
            Some(LoadState::Loaded) => println!("Model: Successfully loaded!"),
            Some(LoadState::Failed(err)) => println!("Model: Failed to load! Error: {:?}", err),
            None => println!("Model: Unknown load state"),
        }
    } else {
        println!("Player entity with SceneRoot not found");
    }
}

/// Debug system to print player position
pub fn debug_player_position(
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(transform) = player_query.get_single() {
        println!("Player position: {:?}", transform.translation);
    } else {
        println!("Player entity with Transform not found");
    }
}

/// Debug system to check scene hierarchy
pub fn debug_scene_hierarchy(
    _scene_spawner: Res<SceneSpawner>,
    children_query: Query<&Children>,
    name_query: Query<&Name>,
    player_query: Query<Entity, With<Player>>,
    parent_query: Query<&Parent>,
) {
    if let Ok(player_entity) = player_query.get_single() {
        println!("Player entity exists: {:?}", player_entity);
        
        // Print basic scene information instead of using the private method
        println!("Scene hierarchy around player:");
        
        // Check if player is a child of any entity
        if let Ok(parent) = parent_query.get(player_entity) {
            println!("Player is a child of: {:?}", parent.get());
            
            // Try to get name of parent
            if let Ok(name) = name_query.get(parent.get()) {
                println!("Parent name: {}", name);
            }
        } else {
            println!("Player is not a child of any entity");
        }
        
        // Check if player has children
        if let Ok(children) = children_query.get(player_entity) {
            println!("Player has {} children", children.len());
            for child in children.iter() {
                if let Ok(name) = name_query.get(*child) {
                    println!("  Child: {:?} with name: {}", child, name);
                } else {
                    println!("  Child: {:?} (no name)", child);
                }
            }
        } else {
            println!("Player has no children");
        }
    }
} 