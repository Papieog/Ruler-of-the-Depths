//world.rs
use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
};
use crate::player::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, add_light)
        .add_systems(Update, change_fog)
        ;
    }
}
#[derive(Component)]
pub struct Ground;
pub fn add_light(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){


    let light = DirectionalLightBundle{
        directional_light: DirectionalLight{
            illuminance: 5000.0,
            shadows_enabled: false,
            ..default()},
        transform: Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    };
    commands.spawn(light);
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0., 0.2, 0.8, 0.5),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(10000., 200., 10000.)),
            ..default()
        },
        NotShadowCaster,
    ));
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0., 0.77, 1., 0.5),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_xyz(0., 199.9, 0.).with_scale(Vec3::new(10000., 200., 10000.)),
            ..default()
        },
        NotShadowCaster,
    ));

    let texture_handle = asset_server.load("sand.png");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let ground = (PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0)),
        transform: Transform::from_xyz(0.,-99.,0.,),
        material: material_handle,
        ..default()
    },
    Ground);
    commands.spawn(ground);

}

pub fn change_fog(
    mut fog: Query<(&mut FogSettings, &Transform), Without<Player>>,
    player: Query<&Transform, (With<Player>, Without<FogSettings>)>
){
    if let Ok(player_transform) = player.get_single(){
        for (mut settings, camera_transform) in fog.iter_mut(){
            let total_translation = player_transform.translation + camera_transform.translation;
            
            if total_translation.y > 100.0 {
                settings.color = Color::rgba(0.6, 0.9, 1.0, 1.0); // Light blue
                settings.directional_light_color = Color::rgba(0.9, 0.95, 1.0, 1.0); // Light white
                settings.directional_light_exponent = 100.0;
                settings.falloff = FogFalloff::from_visibility_colors(
                    10000.0,
                    Color::rgba(0.6, 0.9, 1.0, 1.0), // Light blue
                    Color::rgba(0.9, 0.95, 1.0, 1.0), // Light white
                );
            }
            else{
                settings.color = Color::rgba(0.0, 0.3, 0.7, 1.0);
                settings.directional_light_color = Color::rgba(0.0, 0.5, 0.8, 1.0);
                settings.directional_light_exponent = 100.0;
                settings.falloff = FogFalloff::from_visibility_colors(
                    100.0,
                    Color::rgb(0.0, 0.2, 0.8),
                    Color::rgb(0.3, 0.4, 0.6),
                );
            }
        }
    }
}