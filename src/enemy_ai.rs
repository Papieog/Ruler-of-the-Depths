//enemy_ai.rs
use bevy::prelude::*;
use crate::enemies::*;
use crate::player::*;
use rand::Rng;

#[derive(Component)]
pub struct Targetable;
pub struct EnemyAIPlugin;
impl Plugin for EnemyAIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, target_player)
        .add_systems(PostStartup, spawn_500_fish)
        .add_systems(Startup, spawn_target_0)
        ;
    }
}

pub fn target_player(
    mut fish_query: Query<&mut Enemy>,
    player_query: Query<(&Player, Entity)>,
){
    if let Ok((player,entity)) = player_query.get_single() {
        for mut fish in fish_query.iter_mut(){
            if fish.size>player.size{
                fish.target = entity;
            }
        }
    }
}

pub fn spawn_500_fish(
    commands: Commands,
    enemy_model_assets: Res<EnemyAssets>,
    target_query: Query<(Entity, &TargetZero)>,
){
    let mut rng = rand::thread_rng();
    let size = rng.gen_range(0.5..2.0);
    let x = rng.gen_range(-500.0..500.0);
    let y = rng.gen_range(-100.0..100.0);
    let z = rng.gen_range(-500.0..500.0);
    let position = Vec3::new(x, y, z);

    if let Ok((target, _)) = target_query.get_single(){
        println!("test");
        spawn_fish(commands, enemy_model_assets, size, position, target, 500);
    }
}

#[derive(Component)]
pub struct TargetZero;
pub fn spawn_target_0(
    mut commands: Commands,
){
    let target = (TransformBundle{
        local: Transform::from_xyz(0.,0.,0.),
        ..default()
    },
    TargetZero,
    Targetable);
    commands.spawn(target);
}