use bevy::prelude::*;
use crate::animation_linker::*;
use crate::player::*;
use crate::physics::*;
use crate::enemies::*;
use crate::enemy_ai::*;

#[derive(Resource)]
struct Animations(Vec<Handle<AnimationClip>>);
#[derive(Component)]

pub struct ModelAnimationPlugin;
impl Plugin for ModelAnimationPlugin {
    fn build(&self, app: &mut App) {
        let asset_server = app.world.get_resource::<AssetServer>().unwrap();
        app.insert_resource(Animations(vec![
            asset_server.load("nemo.glb#Animation0"),
            asset_server.load("Fish.glb#Animation0"),
            asset_server.load("Manta ray.glb#Animation0"),
            asset_server.load("Shark.glb#Animation0"),
            asset_server.load("purple_fish.glb#Animation0"),
            asset_server.load("Whale.glb#Animation0"),
        ]))
        .add_systems(Update, animation_func_player)
        .add_systems(Update, (animation_func_1, animation_func_2, animation_func_3, animation_func_4, animation_func_5))
        .add_systems(PostUpdate, animation_speed)
        .add_systems(Last, link_animations)
        .add_systems(Last, link_animations_parent);
    }
}

fn animation_func_player(
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut player_model: Query<(&AnimationEntityPlayerLink, &PlayerModel)>,
) {
    for (link, _) in player_model.iter_mut() {
        if let Ok(mut player) = animation_players.get_mut(link.0) {
            player.play(animations.0[0].clone_weak()).repeat();
        }
    }
}

fn animation_func_1(
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut player_model: Query<(&AnimationEntityLink, &TargetOneComp)>,
) {
    for (link, _) in player_model.iter_mut() {
        if let Ok(mut player) = animation_players.get_mut(link.0) {
            player.play(animations.0[1].clone_weak()).repeat();
        }
    }
}
fn animation_func_2(
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut player_model: Query<(&AnimationEntityLink, &TargetTwoComp)>,
) {
    for (link, _) in player_model.iter_mut() {
        if let Ok(mut player) = animation_players.get_mut(link.0) {
            player.play(animations.0[2].clone_weak()).repeat();
        }
    }
}
fn animation_func_3(
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut player_model: Query<(&AnimationEntityLink, &TargetThreeComp)>,
) {
    for (link, _) in player_model.iter_mut() {
        if let Ok(mut player) = animation_players.get_mut(link.0) {
            player.play(animations.0[0].clone_weak()).repeat();
        }
    }
}
fn animation_func_4(
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut player_model: Query<(&AnimationEntityLink, &TargetFourComp)>,
) {
    for (link, _) in player_model.iter_mut() {
        if let Ok(mut player) = animation_players.get_mut(link.0) {
            player.play(animations.0[0].clone_weak()).repeat();
        }
    }
}
fn animation_func_5(
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut player_model: Query<(&AnimationEntityLink, &TargetFiveComp)>,
) {
    for (link, _) in player_model.iter_mut() {
        if let Ok(mut player) = animation_players.get_mut(link.0) {
            player.play(animations.0[5].clone_weak()).repeat();
        }
    }
}

fn animation_speed(
    mut animation_players: Query<&mut AnimationPlayer>,
    player_query: Query<&Physics, (With<Player>, Without<PlayerModel>)>,
    mut player_model: Query<(&AnimationEntityPlayerLink, &PlayerModel)>,
) {
    if let Ok(physics) = player_query.get_single() {
        for (link, _) in player_model.iter_mut() {
            if let Ok(mut player) = animation_players.get_mut(link.0) {
                player.set_speed(physics.velocity.length() / 5.);
            }
        }
    }
}