//main.rs
use bevy::{prelude::*, window::CursorGrabMode};
mod helper_functions;
mod animation_linker;
mod enemy_ai;
mod animations;
use crate::animations::*;
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
        .add_plugins(ModelAnimationPlugin)

        .add_systems(Update, grab_mouse)
        .run();
}

fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}