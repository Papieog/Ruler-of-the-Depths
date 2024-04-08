use bevy::prelude::*;
use crate::player::Player;
use bevy::input::keyboard::KeyCode;
pub struct UiPlugin;
impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, ui_system)
        .add_systems(Update, update_ui)
            ;
    }
}

#[derive(Component)]
pub struct UiSize;
#[derive(Component)]
pub struct Tutorial;
pub fn ui_system(
    mut commands: Commands,
){
    commands.spawn((
        TextBundle::from("Size: 1.0m").with_style(
            Style {
                position_type: PositionType::Relative,
                top: Val::Px(11.0),
                left: Val::Px(20.0),
                ..default()
            },
        ),
        UiSize)
    );
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new(
                "Controls:\n",
                TextStyle{
                    font_size: 25.0,
                    ..default()
                }
            ),
            TextSection::new(
                "WASD to move horizontally\n",
                TextStyle{
                    font_size: 25.0,
                    ..default()
                }
            ),
            TextSection::new(
                "Space, Shift to move up and down",
                TextStyle{
                    font_size: 25.0,
                    ..default()
                }
            )
        ]).with_style(
            Style{
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                right: Val::Px(20.0),
                ..default()
            },
        ).with_text_justify(JustifyText::Right),
        Tutorial));
}

pub fn update_ui(
    mut query: Query<&mut Text, (With<UiSize>, Without<Tutorial>)>,
    mut tutorial_query: Query<&mut Text, (With<Tutorial>, Without<UiSize>)>,
    player_size: Query<&Player>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if let Ok(mut text) = query.get_single_mut() {
        if let Ok(player) = player_size.get_single() {
            text.sections[0].value = format!("Size: {:.2}m", player.size);
            text.sections[0].style.font_size = 50.0;
        }
    }
    if let Ok(mut text) = tutorial_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::KeyD){
            text.sections[1].value = "".to_string();
        }
        if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::ShiftLeft){
            text.sections[2].value = "".to_string();
        }
        if text.sections[1].value == text.sections[2].value{
            text.sections[0].value = "".to_string();
        }
    }
}
