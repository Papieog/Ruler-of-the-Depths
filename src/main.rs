//main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
/* #region imports */
use bevy::window::WindowMode;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_embedded_assets::PluginMode;
mod animation_linker;
mod animations;
mod enemy_ai;
mod ui;
use crate::ui::*;
mod helper_functions;
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
/* #endregion */

/*#region main*/
fn main() {
    App::new()
        .add_plugins(EmbeddedAssetPlugin {mode: PluginMode::ReplaceDefault,})
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: true,
                mode: WindowMode::BorderlessFullscreen,
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
        .add_plugins(UiPlugin)
        .add_systems(PreStartup, play_music)
        .add_systems(Update, grab_mouse)
        .run();
}
/*#endregion*/

/* #region func */
fn grab_mouse(
    mut windows: Query<&mut Window>,
    mouse: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if mouse.just_pressed(MouseButton::Left) {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.mode = WindowMode::BorderlessFullscreen
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
        window.mode = WindowMode::Windowed
    }
}

pub fn play_music(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
){
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/Juhani_Junkala_Ending.mp3"),
        settings: PlaybackSettings{
            mode: bevy::audio::PlaybackMode::Loop,
            ..default()
        },
        ..default()
    });
}
/* #endregion */

