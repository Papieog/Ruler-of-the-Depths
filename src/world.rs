//world.rs

/* #region init */
use crate::player::*;
use bevy::{pbr::NotShadowCaster, prelude::*};
use rand::Rng;
pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_light)
            .add_systems(Update, change_fog)
            .add_systems(Startup, spawn_particles)
            .add_systems(PreStartup, setup_particle_assets)
            .add_systems(Update, move_particles);
    }
}
/* #endregion */

/* #region spawn world */
#[derive(Component)]
pub struct Ground;
pub fn add_light(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let light = DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: false,
            ..default()
        },
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
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::new(100000., 200., 100000.)),
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
            transform: Transform::from_xyz(0., 199.9, 0.)
                .with_scale(Vec3::new(100000., 10000., 100000.)),
            ..default()
        },
        NotShadowCaster,
    ));


    let ground = (
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(10000.0, 10000.0)),
            transform: Transform::from_xyz(0., -99., 0.),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.96, 0.87, 0.7),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            ..default()
        },
        Ground,
    );
    commands.spawn(ground);
}
/* #endregion */

/* #region world func */
pub fn change_fog(
    mut fog: Query<(&mut FogSettings, &Transform), Without<Player>>,
    camera_transform_transform_query: Query<&Transform, (With<CameraTransform>, Without<Camera3d>, Without<Player>)>,
    player: Query<(&Transform, &Player), Without<FogSettings>>,
) {
    if let Ok((player_transform, player)) = player.get_single() {
        if let Ok(camera_transform_transform) = camera_transform_transform_query.get_single() {
            for (mut settings, camera_transform) in fog.iter_mut() {
                let total_translation =
                    player_transform.translation + camera_transform.translation * player.size * camera_transform_transform.scale.x;

                if total_translation.y > 100.0 {
                    settings.color = Color::rgba(0.6, 0.9, 1.0, 1.0); // Light blue
                    settings.directional_light_color = Color::rgba(0.9, 0.95, 1.0, 1.0); // Light white
                    settings.directional_light_exponent = 100.0;
                    settings.falloff = FogFalloff::from_visibility_colors(
                        10000.0,
                        Color::rgba(0.6, 0.9, 1.0, 1.0),  // Light blue
                        Color::rgba(0.9, 0.95, 1.0, 1.0), // Light white
                    );
                } else {
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
}
/* #endregion */

/* #region Particles */
#[derive(Resource)]
pub struct ParticleAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}
fn setup_particle_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::default());
    let material = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(23000.0, 9000.0, 3000.0),
        ..default()
    });

    commands.insert_resource(ParticleAssets { mesh, material });
}
#[derive(Component)]
pub struct Particles;
pub fn spawn_particles(mut commands: Commands, assets: Res<ParticleAssets>) {
    let mut rng = rand::thread_rng();
    for _ in 0..5000 {
        let x = rng.gen_range(-800.0..800.0);
        let y = rng.gen_range(-100.0..100.0);
        let z = rng.gen_range(-800.0..800.0);
        let particle = (
            PbrBundle {
                mesh: assets.mesh.clone(),
                material: assets.material.clone(),
                transform: Transform::from_xyz(x, y, z).with_scale(Vec3::splat(0.1)),
                ..default()
            },
            Particles,
        );
        commands.spawn(particle);
    }
}

pub fn move_particles(mut particles: Query<&mut Transform, With<Particles>>, time: Res<Time>) {
    for mut transform in particles.iter_mut() {
        transform.translation.y -= 2. * time.delta_seconds();
        if transform.translation.y < -99.9 {
            transform.translation.y = 100.0;
        }
    }
}

/* #endregion */
