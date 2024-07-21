mod vector;
use vector::Vector;
mod triangle;
use triangle::Triangle;
mod bounding_box;
use bounding_box::BoundingBox;
mod node;
use node::Node;

fn BVH(all_nodes: &mut Vec<Node>, all_triangles: &mut Vec<Triangle>, max_depth: i32) {
    // Create bounding box
    let mut bounds: BoundingBox = BoundingBox::new();

    for i in 0..all_triangles.len() {
        bounds.grow_to_include(all_triangles[i]);
    }

    // Create root noode (represents entire, un-split mesh), and split it
    let mut root: Node = Node::new(bounds, 0, all_triangles.len() as i32);
    all_nodes.push(root);
    split(&mut root, 0, all_nodes, all_triangles, max_depth);
}

fn node_cost(size: Vector, num_triangles: f32) -> f32 {
    let half_area: f32 = size.x * (size.y + size.z) + size.y * size.z;
    return half_area * num_triangles;
}

fn choose_split(node: Node, all_triangles: &mut Vec<Triangle>) -> (i32, f32, f32) {
    const num_tests_per_axis: i32 = 5;
    let mut best_cost: f32 = std::f32::INFINITY;
    let mut best_pos: f32 = 0.0;
    let mut best_axis: i32 = 0;

    for axis in 0..3 {
        let bounds_start: f32 = node.bounds.min[axis];
        let bounds_end: f32 = node.bounds.max[axis];

        for i in 0..num_tests_per_axis {
            let split_t: f32 = (i + 1) as f32 / (num_tests_per_axis + 1) as f32;
            let pos: f32 = bounds_start + (bounds_end - bounds_start) * split_t;
            let cost: f32 = evaluate_split(node, axis, pos, all_triangles);

            if cost < best_cost {
                best_cost = cost;
                best_pos = pos;
                best_axis = axis as i32;
            }
        }
    }

    (best_axis, best_pos, best_cost)
}

fn evaluate_split(node: Node, axis: usize, pos: f32, all_triangles: &mut Vec<Triangle>) -> f32 {
    let mut bounds_a: BoundingBox = BoundingBox::new();
    let mut bounds_b: BoundingBox = BoundingBox::new();
    let mut num_in_a: i32 = 0;
    let mut num_in_b: i32 = 0;

    for i in node.triangle_index..node.triangle_index + node.triangle_count {
        let tri: Triangle = all_triangles[i as usize];
        if tri.center[axis] < pos {
            bounds_a.grow_to_include(tri);
            num_in_a += 1;
        } else {
            bounds_b.grow_to_include(tri);
            num_in_b += 1;
        }
    }

    return node_cost(bounds_a.max - bounds_a.min, num_in_a as f32)
        + node_cost(bounds_b.max - bounds_b.min, num_in_b as f32);
}

fn split(
    parent: &mut Node,
    depth: i32,
    all_nodes: &mut Vec<Node>,
    all_triangles: &mut Vec<Triangle>,
    max_depth: i32,
) {
    if depth == max_depth {
        return;
    }

    // Choose split axis and position
    let size: Vector = parent.bounds.max - parent.bounds.min;
    let split_axis = if size.x > size.y.max(size.z) {
        0
    } else if size.y > size.z {
        1
    } else {
        2
    };
    let mut split_pos: f32 = 0.0;
    if split_axis == 0 {
        split_pos = parent.bounds.center.x;
    } else if split_axis == 1 {
        split_pos = parent.bounds.center.y;
    } else {
        split_pos = parent.bounds.center.z;
    }

    // println!("Size at depth {} is {:?}", depth, size);

    let mut child_a: Node = Node::new(BoundingBox::new(), parent.triangle_index, 0);
    let mut child_b: Node = Node::new(BoundingBox::new(), parent.triangle_index, 0);

    for i in parent.triangle_index..parent.triangle_index + parent.triangle_count {
        let mut is_side_a: bool = false;
        if split_axis == 0 {
            is_side_a = all_triangles[i as usize].center.x < split_pos;
        } else if split_axis == 1 {
            is_side_a = all_triangles[i as usize].center.y < split_pos;
        } else {
            is_side_a = all_triangles[i as usize].center.z < split_pos;
        }

        if is_side_a {
            child_a
                .bounds
                .grow_to_include(all_triangles[i as usize].clone());
            child_a.triangle_count += 1;

            // Ensure that the triangles of each child node are grouped together.
            // This allows the node to 'store' the triangles with an index and count.
            let swap: i32 = child_a.triangle_index + child_a.triangle_count - 1;
            // let temp: Triangle = all_triangles[i as usize];
            // all_triangles[i as usize] = all_triangles[swap as usize];
            // all_triangles[swap as usize] = temp;
            all_triangles.swap(i as usize, swap as usize);

            child_b.triangle_index += 1;
        } else {
            child_b.bounds.grow_to_include(all_triangles[i as usize]);
            child_b.triangle_count += 1;
        }
    }

    if child_a.triangle_count > 0 && child_b.triangle_count > 0 {
        if all_nodes.len() != 0 {
            // Find the index of the parent node in the all_nodes vector
            let mut parent_index: i32 = 0;
            for i in 0..all_nodes.len() {
                if all_nodes[i].bounds == parent.bounds {
                    parent_index = i as i32;
                    break;
                }
            }

            // Set the child index of the parent node
            if all_nodes[parent_index as usize].child_index == 0 {
                all_nodes[parent_index as usize].child_index = all_nodes.len() as i32;
            }
        }

        all_nodes.push(child_a);
        all_nodes.push(child_b);

        split(&mut child_a, depth + 1, all_nodes, all_triangles, max_depth);
        split(&mut child_b, depth + 1, all_nodes, all_triangles, max_depth);
    }
}

fn main() {
    let mut all_nodes: Vec<Node> = Vec::new();
    let mut all_triangles: Vec<Triangle> = Vec::new();

    // Load triangle data
    println!("Loading data...");
    let triangle_data = include_bytes!("../../objects/dragon_8k.bin");
    let triangle_data = triangle_data.to_vec();
    let triangle_data = triangle_data.chunks(4).collect::<Vec<_>>();
    let triangle_data = triangle_data
        .iter()
        .map(|d| f32::from_ne_bytes([d[0], d[1], d[2], d[3]]))
        .collect::<Vec<_>>();

    let start_time: std::time::Instant = std::time::Instant::now();

    // Convert to vertices
    println!("Forming data for build...");
    let mut vertices: Vec<Vector> = Vec::new();
    let mut all_triangles: Vec<Triangle> = Vec::new();
    for i in 0..triangle_data.len() / 3 {
        vertices.push(Vector::new(
            triangle_data[i * 3],
            triangle_data[i * 3 + 1],
            triangle_data[i * 3 + 2],
        ));
    }

    // Convert to triangles
    for i in 0..vertices.len() / 3 {
        all_triangles.push(Triangle::new(
            vertices[i * 3],
            vertices[i * 3 + 1],
            vertices[i * 3 + 2],
        ));
    }

    // Build BVH
    println!("Building BVH...");
    let max_depth: i32 = 20;
    BVH(&mut all_nodes, &mut all_triangles, max_depth);

    // Format data for writing: min, max, triangle_index, triangle_count, child_index
    println!("Formatting data for write...");
    let mut data: Vec<f32> = Vec::new();
    for i in 0..all_nodes.len() {
        data.push(all_nodes[i].bounds.min.x);
        data.push(all_nodes[i].bounds.min.y);
        data.push(all_nodes[i].bounds.min.z);
        data.push(all_nodes[i].bounds.max.x);
        data.push(all_nodes[i].bounds.max.y);
        data.push(all_nodes[i].bounds.max.z);
        data.push(all_nodes[i].triangle_index as f32);
        data.push(all_nodes[i].triangle_count as f32);
        data.push(all_nodes[i].child_index as f32);
    }

    let mut triangle_data: Vec<f32> = Vec::new();
    for i in 0..all_triangles.len() {
        triangle_data.push(all_triangles[i].get_a().x);
        triangle_data.push(all_triangles[i].get_a().y);
        triangle_data.push(all_triangles[i].get_a().z);
        triangle_data.push(all_triangles[i].get_b().x);
        triangle_data.push(all_triangles[i].get_b().y);
        triangle_data.push(all_triangles[i].get_b().z);
        triangle_data.push(all_triangles[i].get_c().x);
        triangle_data.push(all_triangles[i].get_c().y);
        triangle_data.push(all_triangles[i].get_c().z);
    }

    // Write data
    println!("Writing data...");
    let data = data
        .iter()
        .map(|d| d.to_ne_bytes())
        .flatten()
        .collect::<Vec<_>>();
    std::fs::write("../objects/dragon_8k_bvh.bin", data).unwrap();
    let triangle_data = triangle_data
        .iter()
        .map(|d| d.to_ne_bytes())
        .flatten()
        .collect::<Vec<_>>();
    std::fs::write("../objects/dragon_8k.bin", triangle_data).unwrap();

    // for i in 0..all_nodes.len() {
    //     println!("Node {}\ttriangle index: {:?}\ttriangle count: {:?}\t child index: {:?} \tbounds: ({:?}, {:?}, {:?}), ({:?}, {:?}, {:?})", i, all_nodes[i].triangle_index, all_nodes[i].triangle_count, all_nodes[i].child_index, all_nodes[i].bounds.min.x, all_nodes[i].bounds.min.y, all_nodes[i].bounds.min.z, all_nodes[i].bounds.max.x, all_nodes[i].bounds.max.y, all_nodes[i].bounds.max.z);
    // }

    println!("Done!");
    println!("Number of nodes: {}", all_nodes.len());
    println!("Max depth: {}", max_depth);
    println!("Time taken: {:?}", start_time.elapsed());
}
