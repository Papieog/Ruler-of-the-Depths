//physics.rs

/* #region init */
use crate::world::*;
use bevy::prelude::*;
pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, movement)
            .add_systems(Update, ground_collision)
            .add_systems(Update, drag);
    }
}
#[derive(Component)]
pub struct Physics {
    pub velocity: Vec3,
    pub collider: f32,
    pub drag: f32,
}

/* #endregion */

/* #region physics func */
pub fn movement(mut query: Query<(&Physics, &mut Transform)>, time: Res<Time>) {
    for (physics, mut transform) in query.iter_mut() {
        transform.translation += physics.velocity * time.delta_seconds();
    }
}

pub fn ground_collision(
    ground: Query<&Transform, With<Ground>>,
    mut physics_objects: Query<(&mut Physics, &mut Transform), Without<Ground>>,
) {
    if let Ok(ground_transform) = ground.get_single() {
        for (mut physics, mut transform) in physics_objects.iter_mut() {
            if transform.translation.y < ground_transform.translation.y + 2.0 {
                physics.velocity.y = physics.velocity.y.max(0.0);
                transform.translation.y = transform.translation.y.max(-101.0);
            }
        }
    }
}

pub fn drag(
    mut physics_objects: Query<(&mut Physics, &mut Transform), Without<Ground>>,
    time: Res<Time>,
) {
    for (mut physics, transform) in physics_objects.iter_mut() {
        if transform.translation.y < 100.0 {
            let drag = physics.drag * physics.velocity / 2.;
            physics.velocity -= drag * time.delta_seconds();
        } else {
            let drag = physics.drag * physics.velocity / 20.;
            physics.velocity -= drag * time.delta_seconds();
            physics.velocity.y -= 50.0 * time.delta_seconds();
        }
    }
}

/* #endregion */