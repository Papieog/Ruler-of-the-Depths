//animations.rs

/* #region Setup */
use crate::animation_linker::*;
use crate::enemy_ai::*;
use crate::physics::*;
use crate::player::*;
use bevy::prelude::*;

#[derive(Resource)]
struct Animations {
    nemo: Handle<AnimationClip>,
    fish: Handle<AnimationClip>,
    manta: Handle<AnimationClip>,
    shark: Handle<AnimationClip>,
    purple: Handle<AnimationClip>,
    whale: Handle<AnimationClip>,
}
#[derive(Component)]

pub struct ModelAnimationPlugin;
impl Plugin for ModelAnimationPlugin {
    fn build(&self, app: &mut App) {
        let asset_server = app.world.get_resource::<AssetServer>().unwrap();
        app.insert_resource(Animations {
            nemo: asset_server.load("nemo.glb#Animation0"),
            fish: asset_server.load("Fish.glb#Animation0"),
            manta: asset_server.load("Manta.glb#Animation0"),
            shark: asset_server.load("Shark.glb#Animation0"),
            purple: asset_server.load("purple_fish.glb#Animation0"),
            whale: asset_server.load("Whale.glb#Animation0"),
        })
        .add_systems(Update, animation_func_player)
        .add_systems(
            Update,
            (
                animation_func_1,
                animation_func_2,
                animation_func_3,
                animation_func_4,
                animation_func_5,
            ),
        )
        .add_systems(PostUpdate, animation_speed)
        .add_systems(Last, link_animations)
        .add_systems(Last, link_animations_parent);
    }
}
/* #endregion */

/* #region animation init */
fn animation_func_player(
    animations: Res<Animations>,
    mut animation_players: Query<&mut AnimationPlayer>,
    mut player_model: Query<(&AnimationEntityPlayerLink, &PlayerModel)>,
) {
    for (link, _) in player_model.iter_mut() {
        if let Ok(mut player) = animation_players.get_mut(link.0) {
            player.play(animations.shark.clone_weak()).repeat();
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
            player.play(animations.purple.clone_weak()).repeat();
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
            player.play(animations.manta.clone_weak()).repeat();
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
            player.play(animations.fish.clone_weak()).repeat();
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
            player.play(animations.nemo.clone_weak()).repeat();
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
            player.play(animations.whale.clone_weak()).repeat();
        }
    }
}
/* #endregion */

/* #region animation funcs */
fn animation_speed(
    mut animation_players: Query<&mut AnimationPlayer>,
    player_query: Query<(&Physics, &Player), Without<PlayerModel>>,
    mut player_model: Query<(&AnimationEntityPlayerLink, &PlayerModel)>,
) {
    if let Ok((physics, player_stats)) = player_query.get_single() {
        for (link, _) in player_model.iter_mut() {
            if let Ok(mut player) = animation_players.get_mut(link.0) {
                player.set_speed(physics.velocity.length() / 10. / player_stats.size);
            }
        }
    }
}
/* #endregion */
