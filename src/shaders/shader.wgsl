struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) screen_size: vec2<f32>,
    @location(1) fov: f32,
    @location(2) screen_width: f32,
    @location(3) screen_height: f32,
    @location(4) aspect_ratio: f32,
    @location(5) camera_position: vec3<f32>,
    @location(6) camera_rotation: vec3<f32>,
};

struct HitInfo {
    did_hit: bool,
    distance: f32,
    position: vec3<f32>,
    normal: vec3<f32>,
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
    var aspect_ratio: f32 = screen_size.x / screen_size.y;
    var screen_width: f32 = tan(fov / 2.0) * 2.0;
    var screen_height: f32 = screen_width / aspect_ratio;
    var camera_position: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var camera_rotation: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    var out: VertexOutput;
    out.pos = vec4<f32>(positions[i], 0.0, 1.0);
    out.screen_size = screen_size;
    out.fov = fov;
    out.screen_width = screen_width;
    out.screen_height = screen_height;
    out.aspect_ratio = screen_size.x / screen_size.y;
    out.camera_position = camera_position;
    out.camera_rotation = camera_rotation;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Store the spheres
    var sphere_centers = array<vec3<f32>, 6>(
        vec3<f32>(40.0, 0.0, 0.0),
        vec3<f32>(-40.0, 0.0, 0.0),
        vec3<f32>(0.0, 40.0, 0.0),
        vec3<f32>(0.0, -40.0, 0.0),
        vec3<f32>(0.0, 0.0, 40.0),
        vec3<f32>(0.0, 0.0, -40.0)
    );
    var sphere_radii = array<f32, 6>(
        10.0,
        10.0,
        10.0,
        10.0,
        10.0,
        10.0
    );

    // Map pixel coordinates to screen plane coordinates
    let u: f32 = (2.0 * in.pos.x / in.screen_size.x - 1.0) * in.screen_width / 2.0;
    let v: f32 = (1.0 - 2.0 * in.pos.y / in.screen_size.y) * in.screen_height / 2.0;

    // Create ray and ray direction vector
    var ray: vec3<f32> = vec3<f32>(u, v, -1.0);
    ray = normalize(ray);
    var ray_direction = ray;

    // Check if the ray intersects a sphere

    return vec4<f32>(0.0, 0.4, 0.4, 1.0);
}

fn ray_sphere(ray_origin: vec3<f32>, ray_direction: vec3<f32>, sphere_center: vec3<f32>, sphere_radius: f32) -> bool {
    return true;
}