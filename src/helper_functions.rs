//helper_functions.rs
use bevy::prelude::*;

pub fn rotate_vector_by_quaternion(vector: Vec3, quaternion: Quat) -> Vec3 {
    let vector_quaternion = Quat::from_vec4(Vec4::new(vector.x, vector.y, vector.z, 0.0));

    // Perform quaternion rotation
    let rotated_vector_quaternion = quaternion * vector_quaternion * quaternion.inverse();

    // Extract the rotated vector
    let rotated_vector = Vec3::new(
        rotated_vector_quaternion.x,
        rotated_vector_quaternion.y,
        rotated_vector_quaternion.z,
    );

    rotated_vector
}