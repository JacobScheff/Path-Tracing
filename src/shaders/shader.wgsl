struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) screen_size: vec2<f32>,
    @location(1) screen_width: f32,
    @location(2) screen_height: f32,
    @location(3) camera_position: vec3<f32>,
    @location(4) camera_rotation: vec3<f32>,
};

struct HitInfo {
    did_hit: bool,
    distance: f32,
    position: vec3<f32>,
    normal: vec3<f32>,
    color: vec3<f32>,
};

@group(0) @binding(0) var<storage, read> sphere_data : array<array<f32, 7>, 6>;

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
    var camera_position: vec3<f32> = vec3<f32>(90.0, 90.0, 130.0);
    var camera_rotation: vec3<f32> = vec3<f32>(-28, 30.0, 0.0);

    var out: VertexOutput;
    out.pos = vec4<f32>(positions[i], 0.0, 1.0);
    out.screen_size = screen_size;
    out.screen_width = screen_width;
    out.screen_height = screen_height;
    out.camera_position = camera_position;
    out.camera_rotation = camera_rotation;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Map pixel coordinates to screen plane coordinates
    let u: f32 = (2.0 * in.pos.x / in.screen_size.x - 1.0) * in.screen_width / 2.0;
    let v: f32 = (1.0 - 2.0 * in.pos.y / in.screen_size.y) * in.screen_height / 2.0;

    // Create ray and ray direction vector
    var ray_direction: vec3<f32> = vec3<f32>(u, v, -1.0);
    ray_direction = normalize(ray_direction);

    // Rotate ray direction vector
    ray_direction = rotate_vector(ray_direction, in.camera_rotation);

    // Check if the ray intersects a sphere
    var hit_info: HitInfo = calculate_ray_collision(in.camera_position, ray_direction);

    // Return the color of the closest hit sphere
    if(hit_info.did_hit) {
        return vec4<f32>(hit_info.color, 1.0);
    }
    
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}

fn calculate_ray_collision(ray_origin: vec3<f32>, ray_direction: vec3<f32>) -> HitInfo {
    var closest_hit: HitInfo;
    closest_hit.did_hit = false;
    closest_hit.distance = 1000000.0;

    // Loop through each sphere in the storage buffer
    for (var i = 0u; i < 6u; i = i + 1u) {
        var sphere_center: vec3<f32> = vec3<f32>(sphere_data[i][0], sphere_data[i][1], sphere_data[i][2]);
        var sphere_radius: f32 = sphere_data[i][3];

        var hit_info: HitInfo = ray_sphere(ray_origin, ray_direction, sphere_data[i]);

        if hit_info.did_hit && hit_info.distance < closest_hit.distance {
            closest_hit = hit_info;
        }
    }

    return closest_hit;
}

fn ray_sphere(ray_origin: vec3<f32>, ray_direction: vec3<f32>, sphere: array<f32, 7>) -> HitInfo {
    var sphere_center: vec3<f32> = vec3<f32>(sphere[0], sphere[1], sphere[2]);
    var sphere_radius: f32 = sphere[3];
    var sphere_color: vec3<f32> = vec3<f32>(sphere[4], sphere[5], sphere[6]);

    var hit_info: HitInfo;
    hit_info.did_hit = false;

    var offset_ray_origin: vec3<f32> = ray_origin - sphere_center;
    let a: f32 = dot(ray_direction, ray_direction);
    let b = 2.0 * dot(offset_ray_origin, ray_direction);
    let c = dot(offset_ray_origin, offset_ray_origin) - sphere_radius * sphere_radius;
    let discriminant = b * b - 4.0 * a * c;

    // No solution when d < 0 (ray misses sphere)
    if discriminant >= 0.0 {
        // Distance to nearest interesction point
        let distance: f32 = (-b - sqrt(discriminant)) / (2.0 * a);

        // Ignore intersections that occur behind the ray
        if distance >= 0.0 {
            hit_info.did_hit = true;
            hit_info.distance = distance;
            hit_info.position = ray_origin + ray_direction * distance;
            hit_info.normal = normalize(hit_info.position - sphere_center);
            hit_info.color = sphere_color;
        }
    }

    return hit_info;
}

fn rotate_vector(ray: vec3<f32>, angles: vec3<f32>) -> vec3<f32> {
    let x = ray.x;
    let y = ray.y;
    let z = ray.z;

    let a = angles.x * 3.14159 / 180.0;
    let b = angles.y * 3.14159 / 180.0;
    let c = angles.z * 3.14159 / 180.0;

    let cos_a = cos(a);
    let sin_a = sin(a);
    let cos_b = cos(b);
    let sin_b = sin(b);
    let cos_c = cos(c);
    let sin_c = sin(c);

    let x_rot = x * cos_c * cos_b + y * (cos_c * sin_b * sin_a - sin_c * cos_a) + z * (cos_c * sin_b * cos_a + sin_c * sin_a);
    let y_rot = x * sin_c * cos_b + y * (sin_c * sin_b * sin_a + cos_c * cos_a) + z * (sin_c * sin_b * cos_a - cos_c * sin_a);
    let z_rot = -x * sin_b + y * cos_b * sin_a + z * cos_b * cos_a;

    return vec3<f32>(x_rot, y_rot, z_rot);
}