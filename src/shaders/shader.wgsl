struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) screen_size: vec2<f32>,
    @location(1) fov: f32,
    @location(2) aspect_ratio: f32,
    @location(3) camera_position: vec3<f32>,
    @location(4) camera_rotation: vec3<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) i: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0), // Bottom Left
        vec2<f32>(1.0, -1.0),  // Bottom Right
        vec2<f32>(-1.0, 1.0),   // Top Left

        vec2<f32>(1.0, 1.0), // Top Right
        vec2<f32>(-1.0, 1.0), // Top Left
        vec2<f32>(1.0, -1.0) // Bottom Right
    );

    var screen_size: vec2<f32> = vec2<f32>(12000.0, 600.0);
    var fov: f32 = 60.0 * 3.14159 / 180.0;
    var camera_position: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var camera_rotation: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    var out: VertexOutput;
    out.clip_position = vec4<f32>(positions[i], 0.0, 1.0);
    out.screen_size = screen_size;
    out.fov = fov;
    out.aspect_ratio = screen_size.x / screen_size.y;
    out.camera_position = camera_position;
    out.camera_rotation = camera_rotation;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    return vec4<f32>(0.0, 0.4, 0.4, 1.0);
}