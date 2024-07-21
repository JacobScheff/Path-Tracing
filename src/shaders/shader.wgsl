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
const triangle_count: u32 = 8712; // Number of triangles in the scene
const bvh_node_count: u32 = 17147; // Number of nodes in the BVH
const bvh_max_depth: u32 = 27; // Max depth of the BVH
const max_bounce_count: u32 = 10; // Max bounces per ray
const rays_per_pixel: u32 = 10; // Number of rays per pixel
const screen_size: vec2<f32> = vec2<f32>(1200.0, 600.0); // Size of the screen
const fov: f32 = 60.0 * 3.14159 / 180.0; // Field of view in radians
const aspect_ratio: f32 = screen_size.x / screen_size.y; // Aspect ratio of the screen

@group(0) @binding(0) var<storage, read> sphere_data : array<array<f32, nums_per_sphere>, sphere_count>;
@group(0) @binding(1) var<storage, read> frame_count: u32;
@group(0) @binding(2) var<storage, read_write> frame_data: array<array<vec3<f32>, u32(screen_size.x)>, u32(screen_size.y * 1.5)>;
@group(0) @binding(3) var<storage, read> camera_position: vec3<f32>;
@group(0) @binding(4) var<storage, read> camera_rotation: vec3<f32>;
@group(0) @binding(5) var<storage, read> triangle_data: array<f32, u32(i32(triangle_count) * 3 * 3)>;
@group(0) @binding(6) var<storage, read> bvh_data: array<f32, u32(9 * bvh_node_count)>;

// Environment lighting
const sky_color_horizon: vec3<f32> = vec3<f32>(0.5, 0.7, 1.0);
const sky_color_zenith: vec3<f32> = vec3<f32>(0.1, 0.25, 1.0);
const ground_color: vec3<f32> = vec3<f32>(0.2, 0.2, 0.2);
const sun_light_direction: vec3<f32> = vec3<f32>(0, -0.4, 0.5); // Not normalized
const sun_intensity: f32 = 3;
const sun_focus: f32 = 200;
const use_environment_lighting: bool = true;

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

        // Check for triangle intersections using BVH
        var hit_info: HitInfo = ray_triangle_bvh(ray);
        if hit_info.did_hit && hit_info.distance < closest_hit.distance {
            closest_hit = hit_info;
        }

    return closest_hit;
}

fn get_node(index: i32) -> array<f32, 9> {
    return array<f32, 9>(bvh_data[u32(index * 9)], bvh_data[u32(index * 9 + 1)], bvh_data[u32(index * 9 + 2)], bvh_data[u32(index * 9 + 3)], bvh_data[u32(index * 9 + 4)], bvh_data[u32(index * 9 + 5)], bvh_data[u32(index * 9 + 6)], bvh_data[u32(index * 9 + 7)], bvh_data[u32(index * 9 + 8)]);
}

fn ray_triangle_bvh(ray: Ray) -> HitInfo {
    var node_stack: array<i32, u32(bvh_max_depth + 1)> = array<i32, u32(bvh_max_depth + 1)>();
    var stack_index: i32 = 0;
    node_stack[stack_index] = 0;
    stack_index++;

    var result: HitInfo;
    result.did_hit = false;
    result.distance = 1000000.0;

    while(stack_index > 0){
        stack_index--;
        let node_index: i32 = node_stack[stack_index];
        let node: array<f32, 9> = get_node(node_index);

            if(node[8] == 0) {
                // Leaf node (no children, so test triangles)
                for(var i: u32 = u32(node[6]); i < u32(node[6] + node[7]); i++){
                    let triangle_hit_info: HitInfo = ray_triangle(ray, array<vec3<f32>, 3>(
                        vec3<f32>(triangle_data[i * 9], triangle_data[i * 9 + 1], triangle_data[i * 9 + 2]),
                        vec3<f32>(triangle_data[i * 9 + 3], triangle_data[i * 9 + 4], triangle_data[i * 9 + 5]),
                        vec3<f32>(triangle_data[i * 9 + 6], triangle_data[i * 9 + 7], triangle_data[i * 9 + 8])
                    ));
                    if (triangle_hit_info.did_hit && triangle_hit_info.distance < result.distance) {
                        result = triangle_hit_info;
                    }
                }
            } else {
                let child_index_a: i32 = i32(node[8]);
                let child_index_b: i32 = i32(node[8] + 1);
                let child_a: array<f32, 9> = get_node(child_index_a);
                let child_b: array<f32, 9> = get_node(child_index_b);

                let dst_a: f32 = ray_box(ray, array<f32, 6>(child_a[0], child_a[1], child_a[2], child_a[3], child_a[4], child_a[5]));
                let dst_b: f32 = ray_box(ray, array<f32, 6>(child_b[0], child_b[1], child_b[2], child_b[3], child_b[4], child_b[5]));

                var dst_near: f32;
                var dst_far: f32;
                var child_index_near: i32;
                var child_index_far: i32;

                if (dst_a < dst_b){
                    dst_near = dst_a;
                    dst_far = dst_b;
                    child_index_near = child_index_a;
                    child_index_far = child_index_b;
                }
                else {
                    dst_near = dst_b;
                    dst_far = dst_a;
                    child_index_near = child_index_b;
                    child_index_far = child_index_a;
                }

                if(dst_far < result.distance){
                        node_stack[stack_index] = child_index_far;
                        stack_index++;
                    }
                    if(dst_near < result.distance){
                        node_stack[stack_index] = child_index_near;
                        stack_index++;
                    }
                
                // let child_a = get_node(i32(node[8]));
                // let child_b = get_node(i32(node[8] + 1));
                
                // let dst_a: f32 = ray_box(ray, array<f32, 6>(child_a[0], child_a[1], child_a[2], child_a[3], child_a[4], child_a[5]));
                // let dst_b: f32 = ray_box(ray, array<f32, 6>(child_b[0], child_b[1], child_b[2], child_b[3], child_b[4], child_b[5]));

                // if (dst_a > dst_b) {
                //     if dst_a < result.distance {
                //         node_stack[stack_index] = i32(child_a[8]);
                //         stack_index++;
                //     }
                //     if dst_b < result.distance {
                //         node_stack[stack_index] = i32(child_b[8]);
                //         stack_index++;
                //     }
                // } else {
                //     if dst_b < result.distance {
                //         node_stack[stack_index] = i32(child_b[8]);
                //         stack_index++;
                //     }
                //     if dst_a < result.distance {
                //         node_stack[stack_index] = i32(child_a[8]);
                //         stack_index++;
                //     }
                // }
            }
    }

    return result;
}

fn ray_box(ray: Ray, bounding_box: array<f32, 6>) -> f32 {
    let min_bound: vec3<f32> = vec3<f32>(bounding_box[0], bounding_box[1], bounding_box[2]);  
    let max_bound: vec3<f32> = vec3<f32>(bounding_box[3], bounding_box[4], bounding_box[5]);

    let t_min = (min_bound - ray.origin) / ray.dir;
    let t_max = (max_bound - ray.origin) / ray.dir;

    let t1 = min(t_min, t_max);
    let t2 = max(t_min, t_max);

    let t_near = max(max(t1.x, t1.y), t1.z);
    let t_far = min(min(t2.x, t2.y), t2.z);

    let did_hit: bool = t_near <= t_far && t_far >= 0.0;
    if did_hit {
        return t_near;
    } else {
        return 999999999.0;
    }
}

fn ray_triangle(ray: Ray, triangle: array<vec3<f32>, 3>) -> HitInfo {
    var hit_info: HitInfo;
    hit_info.did_hit = false;

    var edge_ab: vec3<f32> = triangle[1] - triangle[0];
    var edge_ac: vec3<f32> = triangle[2] - triangle[0];
    var normal_vector: vec3<f32> = cross(edge_ab, edge_ac);
    var ao: vec3<f32> = ray.origin - triangle[0];
    var dao: vec3<f32> = cross(ao, ray.dir);

    var determinant: f32 = -dot(ray.dir, normal_vector);
    var inv_det: f32 = 1.0 / determinant;

    // Calculate distance to triangle and barycentirc coordinates of intersection point
    var dst: f32 = dot(ao, normal_vector) * inv_det;
    var u: f32 = dot(edge_ac, dao) * inv_det;
    var v: f32 = -dot(edge_ab, dao) * inv_det;
    var w: f32 = 1.0 - u - v;

    // Check if the intersection is within the triangle's bounds
    if (dst >= 0.0 && u >= 0.0 && v >= 0.0 && w >= 0.0) {
        hit_info.did_hit = determinant >= 0.0001 && dst >= 0 && u >= 0 && v >= 0 && w >= 0;
        hit_info.distance = dst;
        hit_info.position = ray.origin + ray.dir * dst;
        hit_info.normal = normalize(triangle[0] * w + triangle[1] * u + triangle[2] * v);
        hit_info.color = vec3<f32>(0.25, 1.0, 0.25);
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