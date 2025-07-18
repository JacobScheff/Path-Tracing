mod vector;
use vector::Vector;
mod triangle;
use triangle::Triangle;
mod bounding_box;
use bounding_box::BoundingBox;
mod node;
use node::Node;

const input_file_name: &str = "../objects/teapot.bin";
const output_bvh_file_name: &str = "../objects/teapot_bvh.bin";
const output_bin_file_name: &str = "../objects/teapot.bin";

fn BVH(all_nodes: &mut Vec<Node>, all_triangles: &mut Vec<Triangle>, max_depth: i32) {
    // Create bounding box
    let mut bounds: BoundingBox = BoundingBox::new();

    for i in 0..all_triangles.len() {
        bounds.grow_to_include(all_triangles[i]);
    }

    // Create root noode (represents entire, un-split mesh), and split it
    let mut root: Node = Node::new(bounds, 0, all_triangles.len() as i32);
    all_nodes.push(root);
    split(&mut root, 0, 0, all_nodes, all_triangles, max_depth);
}

fn node_cost(size: Vector, num_triangles: f32) -> f32 {
    let half_area: f32 = size.x * (size.y + size.z) + size.y * size.z;
    return half_area * num_triangles;
}

fn choose_split(node: Node, all_triangles: &mut Vec<Triangle>) -> (i32, f32, f32) {
    const num_tests_per_axis: i32 = 10;
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
    parent_index: usize,
    depth: i32,
    all_nodes: &mut Vec<Node>,
    all_triangles: &mut Vec<Triangle>,
    max_depth: i32,
) {
    if depth == max_depth {
        return;
    }

    // Choose split axis and position
    let (split_axis, split_pos, cost) = choose_split(parent.clone(), all_triangles);

    // Stop splitting if it doesn't improve the cost
    if cost >= node_cost(parent.bounds.max - parent.bounds.min, parent.triangle_count as f32) {
        return;
    }

    let mut child_a: Node = Node::new(BoundingBox::new(), parent.triangle_index, 0);
    let mut child_b: Node = Node::new(BoundingBox::new(), parent.triangle_index, 0);

    for i in parent.triangle_index..parent.triangle_index + parent.triangle_count {
        let mut is_side_a: bool = all_triangles[i as usize].center[split_axis as usize] < split_pos;

        if is_side_a {
            child_a
                .bounds
                .grow_to_include(all_triangles[i as usize].clone());
            child_a.triangle_count += 1;

            // Ensure that the triangles of each child node are grouped together.
            // This allows the node to 'store' the triangles with an index and count.
            let swap: i32 = child_a.triangle_index + child_a.triangle_count - 1;
            all_triangles.swap(i as usize, swap as usize);

            child_b.triangle_index += 1;
        } else {
            child_b.bounds.grow_to_include(all_triangles[i as usize]);
            child_b.triangle_count += 1;
        }
    }

    if child_a.triangle_count > 0 && child_b.triangle_count > 0 {
        all_nodes.push(child_a);
        all_nodes.push(child_b);

        let child_a_index: usize = all_nodes.len() - 2;
        let child_b_index: usize = all_nodes.len() - 1;
        
        all_nodes[parent_index].child_index = child_a_index as i32;

        split(&mut child_a, child_a_index, depth + 1, all_nodes, all_triangles, max_depth);
        split(&mut child_b, child_b_index, depth + 1, all_nodes, all_triangles, max_depth);
    }
}

fn main() {
    let mut all_nodes: Vec<Node> = Vec::new();
    let mut all_triangles: Vec<Triangle> = Vec::new();

    // Load triangle data
    println!("Loading data...");
    let triangle_data = std::fs::read(input_file_name).expect("Failed to read input file");
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
    let max_depth: i32 = 16;
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
    std::fs::write(output_bvh_file_name, data).unwrap();
    let triangle_data = triangle_data
        .iter()
        .map(|d| d.to_ne_bytes())
        .flatten()
        .collect::<Vec<_>>();
    std::fs::write(output_bin_file_name, triangle_data).unwrap();

    // for i in 0..all_nodes.len() {
    //     println!("Node {}\ttriangle index: {:?}\ttriangle count: {:?}\t child index: {:?} \tbounds: ({:?}, {:?}, {:?}), ({:?}, {:?}, {:?})", i, all_nodes[i].triangle_index, all_nodes[i].triangle_count, all_nodes[i].child_index, all_nodes[i].bounds.min.x, all_nodes[i].bounds.min.y, all_nodes[i].bounds.min.z, all_nodes[i].bounds.max.x, all_nodes[i].bounds.max.y, all_nodes[i].bounds.max.z);
    // }

    println!("Done!");
    println!("Number of nodes: {}", all_nodes.len());
    println!("Number of triangles: {}", all_triangles.len());
    println!("Max depth: {}", max_depth);
    println!("Time taken: {:?}", start_time.elapsed());

    // Calculate stats
    println!("\nStats:");
    let mut min_triangles: i32 = std::i32::MAX;
    let mut max_triangles: i32 = std::i32::MIN;
    let mut total_triangles: i32 = 0;
    for i in 0..all_nodes.len() {
        if all_nodes[i].child_index != 0 {
            continue;
        }
        if all_nodes[i].triangle_count < min_triangles {
            min_triangles = all_nodes[i].triangle_count;
        }
        if all_nodes[i].triangle_count > max_triangles {
            max_triangles = all_nodes[i].triangle_count;
        }
        total_triangles += all_nodes[i].triangle_count;
    }
    let average_triangles: f32 = total_triangles as f32 / all_nodes.len() as f32;
    println!("Min triangles in node: {}", min_triangles);
    println!("Max triangles in node: {}", max_triangles);
    println!("Average triangles in node: {}", average_triangles);
}
