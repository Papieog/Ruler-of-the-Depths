//main.rs
use bevy::prelude::*;
mod helper_functions;
mod animation_linker;
mod enemy_ai;
use crate::enemy_ai::*;
mod player;
use crate::player::*;
mod enemies;
use crate::enemies::*;
mod world;
use crate::world::*;
mod physics;
use crate::physics::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(EnemyAIPlugin)
        .run();
}