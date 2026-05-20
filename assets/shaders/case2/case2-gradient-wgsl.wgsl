// Case 2 WGSL shader
// Uses world-space position as color output to create a static gradient across the cube surface.

@fragment
fn fragment(
    @builtin(position) coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) uv: vec2<f32>
    ) -> @location(0) vec4<f32> {
    // Alternative debug outputs used during shader iteration:
    // return vec4<f32>(normals.x, normals.y, normals.z, 0.0); // world normal visualization
    // return vec4<f32>(uv.x, uv.y, 0.0, 0.0); // UV visualization
    
    return world_position;
}