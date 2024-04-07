//main.rs
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_embedded_assets::PluginMode;
use bevy::window::WindowResolution;
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
        .add_plugins(EmbeddedAssetPlugin { mode: PluginMode::ReplaceDefault })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game".to_string(),
                resolution: WindowResolution::new(1920., 1080.).
                with_scale_factor_override(1.),
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
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