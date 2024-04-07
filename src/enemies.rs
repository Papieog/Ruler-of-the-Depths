//enemies.rs
use bevy::prelude::*;
use crate::physics::*;
use crate::helper_functions::*;
use crate::animation_linker::*;
use crate::enemy_ai::Targetable;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(PreStartup, setup_enemy_assets)
        .add_systems(Update, move_enemy)
        .add_systems(Update, look_where_u_going)
        .add_systems(PostUpdate, animation_speed)
        ;
    }
}

#[derive(Resource)]
pub struct EnemyAssets {
    pub fish_model: Handle<Scene>,
}
fn setup_enemy_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    
    let fish_model = asset_server.load("Fish.glb#Scene0");
    commands.insert_resource(EnemyAssets { fish_model });
}

#[derive(Component)]
pub struct Enemy{
    pub speed: f32,
    pub size: f32,
    pub target: Entity
}
#[derive(Component)]
pub struct EnemyModel;
pub fn spawn_fish(
    mut commands: Commands,
    enemy_model_assets: Res<EnemyAssets>,
    size: f32,
    position: Vec3,
    target: Entity,
    amount: i32
){
    for i in 0..amount{
        let enemy = (
            TransformBundle{
                local: Transform::from_translation(position+i as f32).
                with_scale(Vec3::new(size, size, size)),
                ..default()
            },
            Physics{
                velocity: Vec3::ZERO,
                collider: size,
                drag: 1.,
            },
            Enemy{
                speed: 5.0,
                size: size,
                target: target
            },
            <InheritedVisibility as std::default::Default>::default(),
        );

        let enemy_model = (SceneBundle {
            scene: enemy_model_assets.fish_model.clone(),
            transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 0.,3.14,0.)),
            ..default()
        },
        EnemyModel);

        commands.spawn(enemy).with_children(|parent|{
            parent.spawn(enemy_model);
        });
    }
}

pub fn move_enemy(
    time: Res<Time>,
    mut enemies: Query<(&mut Transform, &mut Physics, &Enemy), Without<Targetable>>,
    target_query: Query<&Transform, With<Targetable>>
) {
    for (transform, mut physics, stats) in enemies.iter_mut() {
        if let Ok(target_transform) = target_query.get(stats.target) {
            let temp = Transform::from_translation(transform.translation).looking_at(target_transform.translation, Vec3::Y);
            let enemy_movement = rotate_vector_by_quaternion(Vec3::new(0., 0., -stats.speed), temp.rotation);
            physics.velocity += enemy_movement * time.delta_seconds();
        }
        
    }
}

pub fn look_where_u_going(
    mut enemies: Query<&Physics, (With<Enemy>, Without<EnemyModel>)>,
    mut enemy_model: Query<(&mut Transform, &Parent), (With<EnemyModel>, Without<Enemy>)>,
){
    for (mut transform, parent) in enemy_model.iter_mut() {
        if let Ok(physics) = enemies.get_mut(**parent) {
            transform.look_to(-physics.velocity, Vec3::Y)
        }
    }
}

fn animation_speed(
    mut animation_players: Query<&mut AnimationPlayer>,
    enemy_query: Query<&Physics, With<Enemy>>,
    mut enemy_model: Query<(&AnimationEntityLink, &Parent), With<EnemyModel>>
){
    for (link, parent) in enemy_model.iter_mut(){
        if let Ok(physics) = enemy_query.get(**parent) {
            if let Ok(mut player) = animation_players.get_mut(link.0){
                player.set_speed(physics.velocity.length() / 5.);
            }
        }
        
    }

}