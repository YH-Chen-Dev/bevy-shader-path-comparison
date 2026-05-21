// Case 2 Rust shader
// Uses world-space position as color output to create a static gradient across the cube surface.

pub use bevy_pbr_rust::prelude::*;

use spirv_std::{
    glam::{Vec2, Vec3, Vec4},
    spirv,
};

// Pass through world position, normal, and UV data to the fragment stage.
#[spirv(vertex)]
pub fn vertex_main_case2(
    #[spirv(uniform, descriptor_set = 0, binding = 0)] view: &View,
    in_position: Vec4,
    in_normal: Vec3,
    in_uv: Vec2,

    #[spirv(position)] out_clip_position: &mut Vec4,
    out_world_position: &mut Vec4,
    out_world_normal: &mut Vec3,
    out_uv: &mut Vec2,
) {
    *out_clip_position = view.mesh_position_world_to_clip(in_position);

    *out_world_position = in_position;
    *out_world_normal = in_normal;
    *out_uv = in_uv;
}

#[spirv(fragment)]
#[allow(unused_variables)]
pub fn fragment_main_case2(
    in_world_position: Vec4,
    in_world_normal: Vec3,
    in_uv: Vec2,
    #[spirv(uniform, descriptor_set = 0, binding = 9)] globals: &Globals,
    out_color: &mut Vec4,
) {
    // Alternative debug outputs used during shader iteration:
    // *out_color = in_world_normal.extend(1.0) // world normal visualization
    // *out_color = in_uv.extend(0.0).extend(0.0) // UV visualization

    *out_color = in_world_position;
}
