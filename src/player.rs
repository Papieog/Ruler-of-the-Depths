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
use crate::enemies::Enemy;
use rand::Rng;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(PreStartup, spawn_player)
        .add_systems(Update, camera_movement)
        .add_systems(Update, player_controller)
        .add_systems(Update, rotate_player)
        .add_systems(Update, eat_enemy)
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
        scene: asset_server.load("Shark.glb#Scene0"),
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.,3.14,0.)).with_scale(Vec3::splat(0.9)),
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
        transform: Transform::from_xyz(0.0, 2.0, -50.0).looking_at(Vec3::new(0., 0.5, 0.), Vec3::Y),
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
    mut query: Query<&mut Transform, (With<Camera3d>, Without<CameraTransform>, Without<Player>)>,
    mut query2: Query<&mut Transform, (With<CameraTransform>, Without<Camera3d>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<Camera3d>, Without<CameraTransform>)>,
    mut mouse_motion: EventReader<MouseMotion>,
    time: Res<Time>,
) {
    for ev in mouse_motion.read() {
        let rotation_left_right = Quat::from_axis_angle(Vec3::Y, ev.delta.x*time.delta_seconds()*-0.1);
        let player_transform = player_query.get_single().unwrap();
        for mut camera_transform in &mut query2 {
            camera_transform.rotate(rotation_left_right);
        }
        for mut camera_transform in &mut query {
            if camera_transform.translation.y < -40. {
                let rotation_up_down = Quat::from_axis_angle(Vec3::X, ev.delta.y.max(0.)*time.delta_seconds()*-0.1).inverse();
                camera_transform.rotate_around(Vec3::new(0.,0.,0.), rotation_up_down);
            }
            else if camera_transform.translation.y > 40. {
                let rotation_up_down = Quat::from_axis_angle(Vec3::X, ev.delta.y.min(0.)*time.delta_seconds()*-0.1).inverse();
                camera_transform.rotate_around(Vec3::new(0.,0.,0.), rotation_up_down);
            }
            else if camera_transform.translation.y + player_transform.translation.y < -98. {
                let rotation_up_down = Quat::from_axis_angle(Vec3::X, ev.delta.y.max(1.)*time.delta_seconds()*-0.1).inverse();
                camera_transform.rotate_around(Vec3::new(0.,0.,0.), rotation_up_down);
            }
            else if camera_transform.translation.y + player_transform.translation.y > 98. && player_transform.translation.y<100.{
                let rotation_up_down = Quat::from_axis_angle(Vec3::X, ev.delta.y.min(-1.)*time.delta_seconds()*-0.1).inverse();
                camera_transform.rotate_around(Vec3::new(0.,0.,0.), rotation_up_down);
            }
            else{
                let rotation_up_down = Quat::from_axis_angle(Vec3::X, ev.delta.y*time.delta_seconds()*-0.1).inverse();
                camera_transform.rotate_around(Vec3::new(0.,0.,0.), rotation_up_down);
                *camera_transform = camera_transform.looking_at(Vec3::new(0.,0.5,0.), Vec3::Y);
            }
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

pub fn eat_enemy(
    mut player_query: Query<(&mut Physics, &mut Transform, &mut Player), Without<Enemy>>,
    mut enemy_query: Query<(&mut Enemy, &mut Transform), Without<Player>>,
){
    let mut rng = rand::thread_rng();
    if let Ok((mut physics, mut player_transform, mut player)) = player_query.get_single_mut(){
        for (mut enemy, mut enemy_transform) in enemy_query.iter_mut(){
            if player_transform.translation.distance(enemy_transform.translation)< 6.0 * player.size && player.size>enemy.size{
                enemy.size *= 1.1;
                let x = rng.gen_range(-400.0..400.);
                let y = rng.gen_range(-100.0..100.);
                let z = rng.gen_range(-400.0..400.);
                let position = Vec3::new(x, y, z);
                enemy_transform.translation = position;
                player.size += 0.03*enemy.size/player.size;
            }
            if player_transform.translation.distance(enemy_transform.translation) < 5. * player.size && player.size<enemy.size{
                let direction = (player_transform.translation - enemy_transform.translation).normalize_or_zero();
                physics.velocity = direction*50.0;
                player_transform.translation += direction*3.;
                player.size = (player.size - 0.01).max(1.0);
            }
        }
        player_transform.scale = Vec3::splat(player.size);
        player.speed = 50.0 * player.size
    } 
}


