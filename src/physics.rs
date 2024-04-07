//physics.rs
use bevy::prelude::*;
use crate::enemies::*;
use crate::world::*;
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, movement)
        .add_systems(Update, collision)
        .add_systems(Update, ground_collision)
        .add_systems(Update, drag)
        ;
    }
}

#[derive(Component)]
pub struct Physics{
    pub velocity: Vec3,
    pub collider: f32,
    pub drag: f32,
}


pub fn movement(
    mut query: Query<(&Physics, &mut Transform)>,
    time: Res<Time>,
){
    for (physics, mut transform) in query.iter_mut(){
        transform.translation += physics.velocity*time.delta_seconds();
    }
}

pub fn collision(
    mut query: Query<(&mut Physics, &mut Transform, &Enemy)>,
    time: Res<Time>,
){
    let mut combinations = query.iter_combinations_mut();
    while let Some([(mut physics, transform, enemy), (mut other_physics, other_transform, other_enemy)]) = combinations.fetch_next() {
        if transform.translation.distance(other_transform.translation) < (physics.collider + other_physics.collider)*5.{
            let distance = transform.translation.distance(other_transform.translation);
            let direction = transform.translation - other_transform.translation;
            physics.velocity += direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
            other_physics.velocity -= direction.normalize_or_zero() * time.delta_seconds() * 30. / distance.max(0.1);
            
        }
        
    }
}

pub fn ground_collision(
    ground: Query<&Transform, With<Ground>>,
    mut physics_objects: Query<(&mut Physics, &mut Transform), Without<Ground>>,
){
    if let Ok(ground_transform) = ground.get_single() {
        for (mut physics, mut transform) in physics_objects.iter_mut(){
            if transform.translation.y<ground_transform.translation.y+2.0{
                physics.velocity.y = physics.velocity.y.max(0.0);
                transform.translation.y = transform.translation.y.max(-101.0);
            }
        }
    }
}

pub fn drag(
    mut physics_objects: Query<(&mut Physics, &mut Transform), Without<Ground>>,
    time: Res<Time>,
){
    for (mut physics, transform) in physics_objects.iter_mut(){
        if transform.translation.y < 100.0{
            let drag = physics.drag * physics.velocity / 2.;
            physics.velocity -= drag * time.delta_seconds();
        }
        else{
            let drag = physics.drag * physics.velocity / 20.;
            physics.velocity -= drag * time.delta_seconds();
            physics.velocity.y -= 50.0 * time.delta_seconds();
        }
    }
}