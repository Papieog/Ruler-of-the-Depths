//enemy_ai.rs

/* #region init */
use crate::enemies::*;
use crate::helper_functions::rotate_vector_by_quaternion;
use crate::physics::Physics;
use crate::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct Targetable;
pub struct EnemyAIPlugin;
impl Plugin for EnemyAIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_fish_1,))
            .add_systems(PostStartup, add_component_1)
            .add_systems(PreStartup, spawn_target_1)
            .add_systems(Update, (move_target_1, collision_1))

            .add_systems(Startup, spawn_fish_2)
            .add_systems(PostStartup, add_component_2)
            .add_systems(PreStartup, spawn_target_2)
            .add_systems(Update, (move_target_2, collision_2))

            .add_systems(Startup, spawn_fish_3)
            .add_systems(PostStartup, add_component_3)
            .add_systems(Update, collision_3)

            .add_systems(Startup, spawn_fish_4)
            .add_systems(PostStartup, add_component_4)
            .add_systems(Update, collision_4)

            .add_systems(Startup, spawn_fish_5)
            .add_systems(PostStartup, add_component_5)
            .add_systems(PreStartup, spawn_target_5)
            .add_systems(Update, (move_target_5, collision_5))

            .add_systems(Startup, spawn_fish_6)
            .add_systems(PostStartup, add_component_6)
            .add_systems(PreStartup, spawn_target_6)
            .add_systems(Update, (move_target_6, collision_6))

            .add_systems(Startup, (spawn_fish_7,))
            .add_systems(PostStartup, add_component_7)
            .add_systems(PreStartup, spawn_target_7)
            .add_systems(Update, move_target_7);
    }
}
/* #endregion */

/*#region fish_1*/
pub fn spawn_fish_1(
    commands: Commands,
    enemy_model_assets: Res<EnemyAssets>,
    target_query: Query<(Entity, &TargetOne)>,
) {
    let size = 0.9;
    let position = Vec3::new(100., 0., 100.);

    if let Ok((target, _)) = target_query.get_single() {
        spawn_fish(
            commands,
            enemy_model_assets.purple_model.clone(),
            size,
            position,
            target,
            40,
            5.0,
            0.6,
        );
    }
}

#[derive(Component)]
pub struct TargetOne;
pub fn spawn_target_1(mut commands: Commands) {
    let target = (
        TransformBundle {
            local: Transform::from_xyz(100., 0., 100.),
            ..default()
        },
        TargetOne,
        Targetable,
    );
    commands.spawn(target);
}

pub fn move_target_1(mut target: Query<&mut Transform, With<TargetOne>>, time: Res<Time>) {
    if let Ok(mut transform) = target.get_single_mut() {
        transform.look_at(Vec3::ZERO, Vec3::Y);
        let rotation = transform.rotation;
        transform.translation +=
            rotate_vector_by_quaternion(Vec3::X, rotation) * time.delta_seconds() * 8.;
    }
}

#[derive(Component)]
pub struct TargetOneComp;
pub fn add_component_1(
    mut commands: Commands,
    fish_query: Query<(&Enemy, Entity)>,
    target_entity: Query<Entity, With<TargetOne>>,
) {
    if let Ok(entity) = target_entity.get_single() {
        for (fish, fish_entity) in fish_query.iter() {
            if fish.target == entity {
                commands.entity(fish_entity).insert(TargetOneComp);
            }
        }
    }
}

pub fn collision_1(
    mut query: Query<(&mut Physics, &mut Transform), With<TargetOneComp>>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut physics, transform), (mut other_physics, other_transform)]) =
        combinations.fetch_next()
    {
        if transform.translation.distance(other_transform.translation)
            < (physics.collider + other_physics.collider) * 10.
        {
            let distance = transform.translation.distance(other_transform.translation);
            let direction = transform.translation - other_transform.translation;
            physics.velocity +=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
            other_physics.velocity -=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
        }
        else{
            let direction = transform.translation - other_transform.translation;
            physics.velocity -= direction.normalize_or_zero() * time.delta_seconds() * 0.1;
            other_physics.velocity += direction.normalize_or_zero() * time.delta_seconds() * 0.1;
        }
    }
}
/*#endregion*/

/*#region fish_2*/
pub fn spawn_fish_2(
    commands: Commands,
    enemy_model_assets: Res<EnemyAssets>,
    target_query: Query<(Entity, &TargetTwo)>,
) {
    let size = 1.8;
    let position = Vec3::new(-120., -50., -120.);

    if let Ok((target, _)) = target_query.get_single() {
        spawn_fish(
            commands,
            enemy_model_assets.manta_model.clone(),
            size,
            position,
            target,
            25,
            7.0,
            1.6,
        );
    }
}

#[derive(Component)]
pub struct TargetTwo;
pub fn spawn_target_2(mut commands: Commands) {
    let target = (
        TransformBundle {
            local: Transform::from_xyz(-120., -50., -120.),
            ..default()
        },
        TargetTwo,
        Targetable,
    );
    commands.spawn(target);
}

pub fn move_target_2(mut target: Query<&mut Transform, With<TargetTwo>>, time: Res<Time>) {
    if let Ok(mut transform) = target.get_single_mut() {
        transform.look_at(Vec3::ZERO, Vec3::Y);
        let rotation = transform.rotation;
        transform.translation +=
            rotate_vector_by_quaternion(Vec3::X, rotation) * time.delta_seconds() * 10.;
    }
}

#[derive(Component)]
pub struct TargetTwoComp;
pub fn add_component_2(
    mut commands: Commands,
    fish_query: Query<(&Enemy, Entity)>,
    target_entity: Query<Entity, With<TargetTwo>>,
) {
    if let Ok(entity) = target_entity.get_single() {
        for (fish, fish_entity) in fish_query.iter() {
            if fish.target == entity {
                commands.entity(fish_entity).insert(TargetTwoComp);
            }
        }
    }
}

pub fn collision_2(
    mut query: Query<(&mut Physics, &mut Transform), With<TargetTwoComp>>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut physics, transform), (mut other_physics, other_transform)]) =
        combinations.fetch_next()
    {
        if transform.translation.distance(other_transform.translation)
            < (physics.collider + other_physics.collider) * 15.
        {
            let distance = transform.translation.distance(other_transform.translation);
            let direction = transform.translation - other_transform.translation;
            physics.velocity +=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
            other_physics.velocity -=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
        }
        else{
            let direction = transform.translation - other_transform.translation;
            physics.velocity -= direction.normalize_or_zero() * time.delta_seconds() * 0.1;
            other_physics.velocity += direction.normalize_or_zero() * time.delta_seconds() * 0.1;
        }
    }
}
/*#endregion*/

/*#region fish_3*/

pub fn spawn_fish_3(
    commands: Commands,
    enemy_model_assets: Res<EnemyAssets>,
    target_query: Query<(Entity, &Player)>,
) {
    let size = 2.0;
    let position = Vec3::ZERO;

    if let Ok((target, _)) = target_query.get_single() {
        spawn_fish(
            commands,
            enemy_model_assets.fish_model.clone(),
            size,
            position,
            target,
            10,
            5.0,
            1.3,
        );
    }
}

#[derive(Component)]
pub struct TargetThree;
#[derive(Component)]
pub struct TargetThreeComp;
pub fn add_component_3(
    mut commands: Commands,
    fish_query: Query<(&Enemy, Entity)>,
    target_entity: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = target_entity.get_single() {
        for (fish, fish_entity) in fish_query.iter() {
            if fish.target == entity && fish.speed == 5.0 {
                commands.entity(fish_entity).insert(TargetThreeComp);
            }
        }
    }
}

pub fn collision_3(
    mut query: Query<(&mut Physics, &mut Transform), With<TargetThreeComp>>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut physics, transform), (mut other_physics, other_transform)]) =
        combinations.fetch_next()
    {
        if transform.translation.distance(other_transform.translation)
            < (physics.collider + other_physics.collider) * 5.
        {
            let distance = transform.translation.distance(other_transform.translation);
            let direction = transform.translation - other_transform.translation;
            physics.velocity +=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
            other_physics.velocity -=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
        }
        else{
            let direction = transform.translation - other_transform.translation;
            physics.velocity -= direction.normalize_or_zero() * time.delta_seconds() * 0.1;
            other_physics.velocity += direction.normalize_or_zero() * time.delta_seconds() * 0.1;
        }
    }
}

/*#endregion*/

/*#region fish_4*/

pub fn spawn_fish_4(
    commands: Commands,
    enemy_model_assets: Res<EnemyAssets>,
    target_query: Query<(Entity, &Player)>,
) {
    let size = 0.5;
    let position = Vec3::ZERO;

    if let Ok((target, _)) = target_query.get_single() {
        spawn_fish(
            commands,
            enemy_model_assets.nemo_model.clone(),
            size,
            position,
            target,
            100,
            15.0,
            0.9,
        );
    }
}

#[derive(Component)]
pub struct TargetFour;
#[derive(Component)]
pub struct TargetFourComp;
pub fn add_component_4(
    mut commands: Commands,
    fish_query: Query<(&Enemy, Entity)>,
    target_entity: Query<Entity, With<Player>>,
) {
    if let Ok(entity) = target_entity.get_single() {
        for (fish, fish_entity) in fish_query.iter() {
            if fish.target == entity && fish.speed == 15.0 {
                commands.entity(fish_entity).insert(TargetFourComp);
            }
        }
    }
}

pub fn collision_4(
    mut query: Query<(&mut Physics, &mut Transform), With<TargetFourComp>>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut physics, transform), (mut other_physics, other_transform)]) =
        combinations.fetch_next()
    {
        if transform.translation.distance(other_transform.translation)
            < (physics.collider + other_physics.collider) * 5.
        {
            let distance = transform.translation.distance(other_transform.translation);
            let direction = transform.translation - other_transform.translation;
            physics.velocity +=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
            other_physics.velocity -=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
        }
        else{
            let direction = transform.translation - other_transform.translation;
            physics.velocity -= direction.normalize_or_zero() * time.delta_seconds() * 0.1;
            other_physics.velocity += direction.normalize_or_zero() * time.delta_seconds() * 0.1;
        }
    }
}

/*#endregion*/

/*#region fish_5*/
pub fn spawn_fish_5(
    commands: Commands,
    enemy_model_assets: Res<EnemyAssets>,
    target_query: Query<(Entity, &TargetFive)>,
) {
    let size = 6.0;
    let position = Vec3::new(400., 0., 400.);

    if let Ok((target, _)) = target_query.get_single() {
        spawn_fish(
            commands,
            enemy_model_assets.whale_model.clone(),
            size,
            position,
            target,
            20,
            8.1,
            1.0,
        );
    }
}

#[derive(Component)]
pub struct TargetFive;
pub fn spawn_target_5(mut commands: Commands) {
    let target = (
        TransformBundle {
            local: Transform::from_xyz(400., 0., 400.),
            ..default()
        },
        TargetFive,
        Targetable,
    );
    commands.spawn(target);
}

pub fn move_target_5(mut target: Query<&mut Transform, With<TargetFive>>, time: Res<Time>) {
    if let Ok(mut transform) = target.get_single_mut() {
        transform.look_at(Vec3::ZERO, Vec3::Y);
        let rotation = transform.rotation;
        transform.translation +=
            rotate_vector_by_quaternion(Vec3::X, rotation) * time.delta_seconds() * 12.;
    }
}

#[derive(Component)]
pub struct TargetFiveComp;
pub fn add_component_5(
    mut commands: Commands,
    fish_query: Query<(&Enemy, Entity)>,
    target_entity: Query<Entity, With<TargetFive>>,
) {
    if let Ok(entity) = target_entity.get_single() {
        for (fish, fish_entity) in fish_query.iter() {
            if fish.target == entity {
                commands.entity(fish_entity).insert(TargetFiveComp);
            }
        }
    }
}

pub fn collision_5(
    mut query: Query<(&mut Physics, &mut Transform), With<TargetFiveComp>>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut physics, transform), (mut other_physics, other_transform)]) =
        combinations.fetch_next()
    {
        if transform.translation.distance(other_transform.translation)
            < (physics.collider + other_physics.collider) * 25.
        {
            let distance = transform.translation.distance(other_transform.translation);
            let direction = transform.translation - other_transform.translation;
            physics.velocity +=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
            other_physics.velocity -=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
        }
        else{
            let direction = transform.translation - other_transform.translation;
            physics.velocity -= direction.normalize_or_zero() * time.delta_seconds() * 0.1;
            other_physics.velocity += direction.normalize_or_zero() * time.delta_seconds() * 0.1;
        }
    }
}
/*#endregion*/

/*#region fish_6*/
pub fn spawn_fish_6(
    commands: Commands,
    enemy_model_assets: Res<EnemyAssets>,
    target_query: Query<(Entity, &TargetSix)>,
) {
    let size = 0.9;
    let position = Vec3::new(-120., -50., 120.);

    if let Ok((target, _)) = target_query.get_single() {
        spawn_fish(
            commands,
            enemy_model_assets.purple_model.clone(),
            size,
            position,
            target,
            40,
            5.0,
            0.6,
        );
    }
}

#[derive(Component)]
pub struct TargetSix;
pub fn spawn_target_6(mut commands: Commands) {
    let target = (
        TransformBundle {
            local: Transform::from_xyz(-120., -50., 120.),
            ..default()
        },
        TargetSix,
        Targetable,
    );
    commands.spawn(target);
}

pub fn move_target_6(mut target: Query<&mut Transform, With<TargetSix>>, time: Res<Time>) {
    if let Ok(mut transform) = target.get_single_mut() {
        let rotation = transform.rotation;
        transform.translation +=
            rotate_vector_by_quaternion(Vec3::NEG_Z, rotation) * time.delta_seconds() * 8.;
        if transform.translation.z > 120. || transform.translation.x < -120.{
            transform.look_at(Vec3::new(120., -50., -120.), Vec3::Y);
        }
        if transform.translation.z < -120. || transform.translation.x > 120. {
            transform.look_at(Vec3::new(-120., -50., 120.), Vec3::Y);
        }
    }
}

#[derive(Component)]
pub struct TargetSixComp;
pub fn add_component_6(
    mut commands: Commands,
    fish_query: Query<(&Enemy, Entity)>,
    target_entity: Query<Entity, With<TargetSix>>,
) {
    if let Ok(entity) = target_entity.get_single() {
        for (fish, fish_entity) in fish_query.iter() {
            if fish.target == entity {
                commands.entity(fish_entity).insert(TargetSixComp);
            }
        }
    }
}

pub fn collision_6(
    mut query: Query<(&mut Physics, &mut Transform), With<TargetSixComp>>,
    time: Res<Time>,
) {
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut physics, transform), (mut other_physics, other_transform)]) =
        combinations.fetch_next()
    {
        if transform.translation.distance(other_transform.translation)
            < (physics.collider + other_physics.collider) * 10.
        {
            let distance = transform.translation.distance(other_transform.translation);
            let direction = transform.translation - other_transform.translation;
            physics.velocity +=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
            other_physics.velocity -=
                direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
        }
        else{
            let direction = transform.translation - other_transform.translation;
            physics.velocity -= direction.normalize_or_zero() * time.delta_seconds() * 0.1;
            other_physics.velocity += direction.normalize_or_zero() * time.delta_seconds() * 0.1;
        }
    }
}
/*#endregion*/

/*#region fish_7*/
pub fn spawn_fish_7(
    commands: Commands,
    enemy_model_assets: Res<EnemyAssets>,
    target_query: Query<(Entity, &TargetSeven)>,
) {
    let size = 10.0;
    let position = Vec3::new(-600., 0., -600.);

    if let Ok((target, _)) = target_query.get_single() {
        spawn_fish(
            commands,
            enemy_model_assets.shark_model.clone(),
            size,
            position,
            target,
            1,
            12.0,
            0.8,
        );
    }
}

#[derive(Component)]
pub struct TargetSeven;
pub fn spawn_target_7(mut commands: Commands) {
    let target = (
        TransformBundle {
            local: Transform::from_xyz(-600., 0., -600.),
            ..default()
        },
        TargetSeven,
        Targetable,
    );
    commands.spawn(target);
}

pub fn move_target_7(mut target: Query<&mut Transform, With<TargetSeven>>, time: Res<Time>) {
    if let Ok(mut transform) = target.get_single_mut() {
        transform.look_at(Vec3::ZERO, Vec3::Y);
        let rotation = transform.rotation;
        transform.translation +=
            rotate_vector_by_quaternion(Vec3::X, rotation) * time.delta_seconds() * 10.;
    }
}

#[derive(Component)]
pub struct TargetSevenComp;
pub fn add_component_7(
    mut commands: Commands,
    fish_query: Query<(&Enemy, Entity)>,
    target_entity: Query<Entity, With<TargetSeven>>,
) {
    if let Ok(entity) = target_entity.get_single() {
        for (fish, fish_entity) in fish_query.iter() {
            if fish.target == entity {
                commands.entity(fish_entity).insert(TargetSevenComp);
            }
        }
    }
}
/*#endregion*/