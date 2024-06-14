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

@group(0) @binding(0) var<storage, read> sphere_data : array<vec4<f32>>;

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

    var screen_size: vec2<f32> = vec2<f32>(1200.0, 600.0);
    var fov: f32 = 60.0 * 3.14159 / 180.0;
    var aspect_ratio: f32 = screen_size.x / screen_size.y;
    var screen_width: f32 = tan(fov * 0.5) * 2.0;
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
    // Map pixel coordinates to screen plane coordinates
    let u: f32 = (2.0 * in.pos.x / in.screen_size.x - 1.0) * in.screen_width / 2.0;
    // let u: f32 = (in.pos.x / in.screen_size.x);
    let v: f32 = (1.0 - 2.0 * (in.screen_size.y - in.pos.y) / in.screen_size.y) * in.screen_height / 2.0;

    // if u < 0 {
    //     return vec4<f32>(-u, 0.0, 0.0, 1.0);
    // }
    // else {
    //     return vec4<f32>(0.0, 0.0, u, 1.0);
    // }

    // Create ray and ray direction vector
    var ray_direction: vec3<f32> = vec3<f32>(u, v, -1.0);
    ray_direction = normalize(ray_direction);

    let max_steps: u32 = 500u;
    let max_distance: f32 = 500.0;
    var distance_traveled: f32 = 0.0;
    for (var i: u32 = 0u; i < max_steps; i = i + 1u) {
        let origin: vec3<f32> = in.camera_position;
        let ray_position: vec3<f32> = origin + ray_direction * distance_traveled;

        var closest_distance: f32 = 100000.0;
        for (var j: u32 = 0u; j < 6; j = j + 1u) {
            let sphere: vec4<f32> = sphere_data[j];
            let sphere_position: vec3<f32> = vec3<f32>(sphere.x, sphere.y, sphere.z);
            let sphere_radius: f32 = sphere.w;

            let distance: f32 = length(ray_position - sphere_position) - sphere_radius;
            if (distance < closest_distance) {
                closest_distance = distance;
            }

            if (distance < 0.1) {
                return vec4<f32>(1.0, 0.0, 0.0, 1.0);
            }
        }

        distance_traveled += closest_distance;
        if (distance_traveled > max_distance) {
            break;
        }
    }

    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}