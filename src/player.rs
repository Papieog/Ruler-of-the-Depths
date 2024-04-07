//player.rs
use bevy::{
    core_pipeline::{
        bloom::BloomSettings,
        tonemapping::Tonemapping,
    },
    prelude::*,
};


use crate::physics::*;
use crate::helper_functions::*;
use crate::enemy_ai::Targetable;
use bevy::input::keyboard::KeyCode;
use bevy::input::mouse::MouseMotion;
use bevy::input::mouse::MouseWheel;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(PreStartup, spawn_player)
        .add_systems(Update, camera_movement)
        .add_systems(Update, player_controller)
        .add_systems(Update, rotate_player)
        ;
    }
}
#[derive(Component)]
pub struct Player{
    pub size: f32,
    pub speed: f32,
    pub health: f32,
}
#[derive(Component)]
pub struct PlayerModel;
#[derive(Component)]
pub struct CameraTransform;
pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){

    let player = (
    TransformBundle{
        local: Transform::from_xyz(0.,0.5,0.),
        ..default()
    },
    <InheritedVisibility as std::default::Default>::default(),
    Physics{
        velocity: Vec3::ZERO,
        collider: 1.0,
        drag: 1.,
    },
    Player{
        size: 1.0,
        speed: 50.0,
        health: 100.0,
    },
    Targetable
    );

    let player_model = (SceneBundle {
        scene: asset_server.load("nemo.glb#Scene0"),
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.,3.14,0.)),
        ..default()
    },
    PlayerModel);

    let camera_transform = (
        TransformBundle{
            ..default()
        }, CameraTransform);
    let camera = (Camera3dBundle{
        camera: Camera{
            hdr: true,
            ..default()
        },
        tonemapping: Tonemapping::TonyMcMapface,
        transform: Transform::from_xyz(0.0, 2.0, -35.0).looking_at(Vec3::new(0., 0.5, 0.), Vec3::Y),
        ..default()
    },
    BloomSettings{
        intensity: 0.3,
        ..default()
    },
    FogSettings {
        color: Color::rgba(0.0, 0.3, 0.7, 1.0), // Dark blue for the underwater environment
        directional_light_color: Color::rgba(0.0, 0.1, 0.2, 1.0), // Adjusted directional light color
        directional_light_exponent: 100.0, // Increase light exponent for more scattering
        falloff: FogFalloff::from_visibility_colors(
            100.0, // Reduce visibility distance underwater
            Color::rgb(0.0, 0.2, 0.8), // Deeper blue for absorption
            Color::rgb(0.3, 0.4, 0.6), // Light blue for scattering
        ),
    });

    
    commands.spawn(player).with_children(|parent|{
        parent.spawn(camera_transform).with_children(|subparent|{
            subparent.spawn(camera);
        });
        parent.spawn(player_model);
    });
}

pub fn camera_movement(
    mut query: Query<&mut Transform, (With<Camera3d>, Without<CameraTransform>)>,
    mut query2: Query<&mut Transform, (With<CameraTransform>, Without<Camera3d>)>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    for ev in mouse_motion.read() {
        let rotation_left_right = Quat::from_axis_angle(Vec3::Y, ev.delta.x*time.delta_seconds()*-0.1);
        let rotation_up_down = Quat::from_axis_angle(Vec3::X, ev.delta.y*time.delta_seconds()*-0.1).inverse();
        for mut camera_transform in &mut query2 {
            camera_transform.rotate(rotation_left_right);
        }
        for mut camera_transform in &mut query {
            camera_transform.rotate_around(Vec3::new(0.,0.,0.), rotation_up_down);
            *camera_transform = camera_transform.looking_at(Vec3::new(0.,0.5,0.), Vec3::Y);
        }
    }
}


pub fn player_controller(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Physics, &Player, &Transform), Without<CameraTransform>>,
    query_camera: Query<&Transform, (With<CameraTransform>, Without<Player>)>,
) {
    for (mut physics, stats, player_transform) in query.iter_mut() {
        for camera_transform in query_camera.iter() {
            let camera_rotation = camera_transform.rotation;
            let mut translation_value = Vec3::new(0., 0., 0.);

            if keyboard_input.pressed(KeyCode::KeyW) {
                translation_value += Vec3::new(0.,0.,1.);
            }
            if keyboard_input.pressed(KeyCode::KeyS) {
                translation_value += Vec3::new(0.,0.,-1.);
            }
            if keyboard_input.pressed(KeyCode::KeyA) {
                translation_value += Vec3::new(1.,0.,0.);
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                translation_value += Vec3::new(-1.,0.,0.);
            }
            if keyboard_input.pressed(KeyCode::Space) {
                translation_value += Vec3::new(0.,1.,0.);
            }
            if keyboard_input.pressed(KeyCode::ShiftLeft) {
                translation_value += Vec3::new(0.,-1.,0.);
            }
            if translation_value.length() > 0.0 && player_transform.translation.y < 100.{
                let translation = rotate_vector_by_quaternion(translation_value.normalize()*stats.speed*time.delta_seconds(), camera_rotation);
                physics.velocity.x += translation.x;
                physics.velocity.y += translation.y;
                physics.velocity.z += translation.z;
            }
            
        }
    }
}

pub fn rotate_player(
    player_query: Query<&Physics, (With<Player>, Without<PlayerModel>)>,
    mut player_model_query: Query<&mut Transform, (With<PlayerModel>, Without<Player>)>,
) {
    for physics in player_query.iter() {
        
        for mut transform in player_model_query.iter_mut() {

            if physics.velocity.length() > 1.0 {
                transform.look_to(-physics.velocity, Vec3::Y)
            }
        }
    }
}


