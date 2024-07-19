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
    split(&mut root, 0, all_nodes, all_triangles, max_depth);
}

fn split(parent: &mut Node, depth: i32, all_nodes: &mut Vec<Node>, all_triangles: &mut Vec<Triangle>, max_depth: i32) {
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

    // Create child nodes
    parent.child_index = all_nodes.len() as i32;
    let mut child_a: Node = Node::new(BoundingBox::new(), parent.triangle_index, 0);
    let mut child_b: Node = Node::new(BoundingBox::new(), parent.triangle_index, 0);
    all_nodes.push(child_a);
    all_nodes.push(child_b);

    for i in parent.triangle_index..parent.triangle_index + parent.triangle_count {
        let mut is_side_a: bool = false;
        if split_axis == 0 {
            is_side_a = all_triangles[i as usize].center.x < split_pos;
        } else if split_axis == 1 {
            is_side_a = all_triangles[i as usize].center.y < split_pos;
        } else {
            is_side_a = all_triangles[i as usize].center.z < split_pos;
        }

        let mut child: Node = if is_side_a { child_a } else { child_b };
        child.bounds.grow_to_include(all_triangles[i as usize]);
        child.triangle_count += 1;

        if is_side_a {
            // Ensure that the triangles of each child node are grouped together.
            // This allows the node to 'store' the triangles with an index and count.
            let swap: i32 = child.triangle_index + child.triangle_count - 1;
            let temp: Triangle = all_triangles[i as usize];
            all_triangles[i as usize] = all_triangles[swap as usize];
            all_triangles[swap as usize] = temp;
        }
    }

    split(&mut child_a, depth + 1, all_nodes, all_triangles, max_depth);
    split(&mut child_b, depth + 1, all_nodes, all_triangles, max_depth);
}

fn main() {
    let mut all_nodes: Vec<Node> = Vec::new();
    let mut all_triangles: Vec<Triangle> = Vec::new();

    // Load triangle data
    println!("Loading data...");
    let triangle_data = include_bytes!("../../objects/knight.bin");
    let triangle_data = triangle_data.to_vec();
    let triangle_data = triangle_data.chunks(4).collect::<Vec<_>>();
    let triangle_data = triangle_data
        .iter()
        .map(|d| f32::from_ne_bytes([d[0], d[1], d[2], d[3]]))
        .collect::<Vec<_>>();
    
    // Convert to vertices
    println!("Forming data...");
    let mut vertices: Vec<Vector> = Vec::new();
    let mut all_triangles: Vec<Triangle> = Vec::new();
    for i in 0..triangle_data.len() / 3 {
        vertices.push(Vector::new(triangle_data[i * 3], triangle_data[i * 3 + 1], triangle_data[i * 3 + 2]));
    }

    // Convert to triangles
    for i in 0..vertices.len() / 3 {
        all_triangles.push(Triangle::new(vertices[i * 3], vertices[i * 3 + 1], vertices[i * 3 + 2]));
    }

    // Build BVH
    println!("Building BVH...");
    BVH(&mut all_nodes, &mut all_triangles, 4);

    println!("Done!");
}