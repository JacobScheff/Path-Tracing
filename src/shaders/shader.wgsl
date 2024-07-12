struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) screen_width: f32,
    @location(1) screen_height: f32,
};

struct HitInfo {
    did_hit: bool,
    distance: f32,
    position: vec3<f32>,
    normal: vec3<f32>,
    color: vec3<f32>,
    emission_color: vec3<f32>,
    emission_strength: f32,
    smoothness: f32,
};

struct Ray {
    origin: vec3<f32>,
    dir: vec3<f32>,
};

const sphere_count: u32 = 1; // Number of spheres in the scene
const nums_per_sphere: u32 = 12; // Number of values stored for every sphere
const triangle_count: u32 = 456; // Number of triangles in the scene
const max_bounce_count: u32 = 20; // Max bounces per ray
const rays_per_pixel: u32 = 40; // Number of rays per pixel
const screen_size: vec2<f32> = vec2<f32>(1200.0, 600.0); // Size of the screen
const fov: f32 = 60.0 * 3.14159 / 180.0; // Field of view in radians
const aspect_ratio: f32 = screen_size.x / screen_size.y; // Aspect ratio of the screen

@group(0) @binding(0) var<storage, read> sphere_data : array<array<f32, nums_per_sphere>, sphere_count>;
@group(0) @binding(1) var<storage, read> frame_count: u32;
@group(0) @binding(2) var<storage, read_write> frame_data: array<array<vec3<f32>, u32(screen_size.x)>, u32(screen_size.y * 1.5)>;
@group(0) @binding(3) var<storage, read> camera_position: vec3<f32>;
@group(0) @binding(4) var<storage, read> camera_rotation: vec3<f32>;
@group(0) @binding(5) var<storage, read> triangle_data: array<array<vec3<f32>, 4>, triangle_count>;
@group(0) @binding(6) var<storage, read> bounding_box: array<vec3<f32>, 2>;

// Environment lighting
const sky_color_horizon: vec3<f32> = vec3<f32>(0.5, 0.7, 1.0);
const sky_color_zenith: vec3<f32> = vec3<f32>(0.1, 0.25, 1.0);
const ground_color: vec3<f32> = vec3<f32>(0.2, 0.2, 0.2);
const sun_light_direction: vec3<f32> = vec3<f32>(0, -0.4, 0.5); // Not normalized
const sun_intensity: f32 = 3;
const sun_focus: f32 = 200;
const use_environment_lighting: bool = false;

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

    var screen_width: f32 = tan(fov * 0.5) * 2.0;
    var screen_height: f32 = screen_width / aspect_ratio;

    var out: VertexOutput;
    out.pos = vec4<f32>(positions[i], 0.0, 1.0);
    out.screen_width = screen_width;
    out.screen_height = screen_height;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // Map pixel coordinates to screen plane coordinates
    let u: f32 = (2.0 * in.pos.x / screen_size.x - 1.0) * in.screen_width / 2.0;
    let v: f32 = (1.0 - 2.0 * in.pos.y / screen_size.y) * in.screen_height / 2.0;
    let pixel_index: u32 = u32(in.pos.x + in.pos.y * screen_size.x);

    // Create ray and ray direction vector
    var ray_direction: vec3<f32> = vec3<f32>(u, v, -1.0);
    ray_direction = normalize(ray_direction);

    // Rotate ray direction vector
    ray_direction = rotate_vector(ray_direction, camera_rotation);

    // Create ray
    var ray: Ray;
    ray.origin = camera_position;
    ray.dir = ray_direction;

    // Calculate pixel color
    var pixel_color: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    for (var i: u32 = 0u; i < rays_per_pixel; i = i + 1u) {
        pixel_color += trace(ray, pixel_index + i * 248135);
    }
    pixel_color /= f32(rays_per_pixel);

    let weight: f32 = 1.0 / f32(frame_count + 1); // Might need to be + 2 since frame_count starts at 0
    let weighted_average: vec3<f32> = frame_data[i32(in.pos.y)][i32(in.pos.x)] * (1.0 - weight) + pixel_color * weight;
    frame_data[i32(in.pos.y)][i32(in.pos.x)] = weighted_average;
    
    return vec4<f32>(weighted_average, 1.0);
}

fn trace(ray_in: Ray, seed: u32) -> vec3<f32> {
    var ray: Ray = ray_in;

    var incoming_light: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var ray_color: vec3<f32> = vec3<f32>(1.0, 1.0, 1.0);

    for(var i: u32 = 0; i <= max_bounce_count; i++){
        var hit_info: HitInfo = calculate_ray_collision(ray);
        if(hit_info.did_hit) {
            ray.origin = hit_info.position;
            let diffuse_dir: vec3<f32> = normalize(hit_info.normal + random_direction(seed + i * 12345 + frame_count * 393939));
            let specular_dir: vec3<f32> = reflect(ray.dir, hit_info.normal);
            ray.dir = lerp(diffuse_dir, specular_dir, hit_info.smoothness);

            var emitted_light: vec3<f32> = hit_info.emission_color * hit_info.emission_strength;
            incoming_light += emitted_light * ray_color;
            ray_color *= hit_info.color;
        } else {
            if(use_environment_lighting){
                incoming_light += get_environment_light(ray) * ray_color;
            }
            break;
        }
    }

    return incoming_light;
}

fn calculate_ray_collision(ray: Ray) -> HitInfo {
    var closest_hit: HitInfo;
    closest_hit.did_hit = false;
    closest_hit.distance = 1000000.0;

    // Check for sphere intersections
    for (var i = 0u; i < sphere_count; i = i + 1u) {
        var sphere_center: vec3<f32> = vec3<f32>(sphere_data[i][0], sphere_data[i][1], sphere_data[i][2]);
        var sphere_radius: f32 = sphere_data[i][3];

        var hit_info: HitInfo = ray_sphere(ray, sphere_data[i]);

        if hit_info.did_hit && hit_info.distance < closest_hit.distance {
            closest_hit = hit_info;
        }
    }

    // Check if ray intersects bounding box
    if (ray_box(ray, bounding_box)) {
        // Check for triangle intersections
        for (var i = 0u; i < triangle_count; i = i + 1u) {
            var hit_info: HitInfo = ray_triangle(ray, triangle_data[i]);

            if hit_info.did_hit && hit_info.distance < closest_hit.distance {
                closest_hit = hit_info;
            }
        }
    }

    return closest_hit;
}

fn ray_box(ray: Ray, bounding_box: array<vec3<f32>, 2>) -> bool {
    let min_bound = bounding_box[0];
    let max_bound = bounding_box[1];

    let t_min = (min_bound - ray.origin) / ray.dir;
    let t_max = (max_bound - ray.origin) / ray.dir;

    let t1 = min(t_min, t_max);
    let t2 = max(t_min, t_max);

    let t_near = max(max(t1.x, t1.y), t1.z);
    let t_far = min(min(t2.x, t2.y), t2.z);

    return t_near <= t_far && t_far >= 0.0;
}

fn ray_triangle(ray: Ray, triangle: array<vec3<f32>, 4>) -> HitInfo {
    var hit_info: HitInfo;
    hit_info.did_hit = false;

    var edge_ab: vec3<f32> = triangle[1] - triangle[0];
    var edge_ac: vec3<f32> = triangle[2] - triangle[0];
    var normal_vector: vec3<f32> = cross(edge_ab, edge_ac);
    var ao: vec3<f32> = ray.origin - triangle[0];
    var dao: vec3<f32> = cross(ao, ray.dir);

    var determinant: f32 = -dot(ray.dir, normal_vector);
    var inv_det: f32 = 1.0 / determinant;

    // Calculate distance to triangle and barycentric coordinates of intersection point
    var dst: f32 = dot(ao, normal_vector) * inv_det;
    var u: f32 = dot(edge_ac, dao) * inv_det;
    var v: f32 = -dot(edge_ab, dao) * inv_det;
    var w: f32 = 1.0 - u - v;

    if(determinant >= 0.000001 && dst >= 0 && u >= 0 && v >= 0 && w >= 0){
        hit_info.did_hit = true;
        hit_info.distance = dst;
        hit_info.position = ray.origin + ray.dir * dst;
        hit_info.normal = triangle[3];
        hit_info.color = vec3<f32>(1.0, 1.0, 1.0);
        hit_info.emission_color = vec3<f32>(0.0, 0.0, 0.0);
        hit_info.emission_strength = 0.0;
        hit_info.smoothness = 0.0;
    }

    return hit_info;
}

fn ray_sphere(ray: Ray, sphere: array<f32, nums_per_sphere>) -> HitInfo {
    var sphere_center: vec3<f32> = vec3<f32>(sphere[0], sphere[1], sphere[2]);
    var sphere_radius: f32 = sphere[3];
    var sphere_color: vec3<f32> = vec3<f32>(sphere[4], sphere[5], sphere[6]);

    var hit_info: HitInfo;
    hit_info.did_hit = false;

    var offset_ray_origin: vec3<f32> = ray.origin - sphere_center;
    let a: f32 = dot(ray.dir, ray.dir);
    let b = 2.0 * dot(offset_ray_origin, ray.dir);
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
            hit_info.position = ray.origin + ray.dir * distance;
            hit_info.normal = normalize(hit_info.position - sphere_center);
            hit_info.color = sphere_color;
            hit_info.emission_color = vec3<f32>(sphere[7], sphere[8], sphere[9]);
            hit_info.emission_strength = sphere[10];
            hit_info.smoothness = sphere[11];
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

// Function to generate a random number between 0 and 1
fn random_value(seed: u32) -> f32 
{
    var state = seed;
    state = state ^ (state << 13);
    state = state ^ (state >> 17);
    state = state ^ (state << 5);
    return f32(state) / 4294967296.0;  // Normalize to [0, 1)
}

// Random value in normal distribution (mean = 0, std_dev = 1)
fn random_normal(seed: u32) -> f32
{
    var u1 = random_value(seed);
    var u2 = random_value(seed * 7462);
    return sqrt(-2.0 * log(u1)) * cos(2.0 * 3.14159 * u2);
}

// Random direction vector
fn random_direction(seed: u32) -> vec3<f32>
{
    var x = random_normal(seed);
    var y = random_normal(seed * 379);
    var z = random_normal(seed * 123);
    return normalize(vec3<f32>(x, y, z));
}

// Random direction in the hemisphere oriented around the given normal vector
fn random_hemisphere_direction(normal: vec3<f32>, seed: u32) -> vec3<f32>
{
    var dir = random_direction(seed);
    return dir * sign(dot(normal, dir));
}

// Lerp
fn lerp(a: vec3<f32>, b: vec3<f32>, t: f32) -> vec3<f32>
{
    return a * (1.0 - t) + b * t;
}

// Background environment lighting
fn get_environment_light(ray: Ray) -> vec3<f32>
{
    let sky_gradient_t = pow(smoothstep(0.0, 0.4, ray.dir.y), 0.35);
    let sky_gradient: vec3<f32> = lerp(sky_color_horizon, sky_color_zenith, sky_gradient_t);
    let sun: f32 = pow(max(0.0, dot(ray.dir, -normalize(sun_light_direction))), sun_focus) * sun_intensity;

    // Combine ground, sky, and sun
    let ground_to_sky_t = smoothstep(-0.01, 0.0, ray.dir.y);
    var sun_mask: f32 = 0.0;
    if (ground_to_sky_t >= 1) {
        sun_mask = 1.0;
    }
    return lerp(ground_color, sky_gradient, ground_to_sky_t) + sun * sun_mask;
}