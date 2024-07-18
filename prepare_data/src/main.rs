mod vector;
use vector::Vector;
mod triangle;
use triangle::Triangle;
mod bounding_box;
use bounding_box::BoundingBox;
mod node;
use node::Node;

const max_depth: i32 = 4;

fn BVH(vertices: Vec<Vector>, triangle_indices: Vec<i32>) {
    // // Create bounding box
    // let mut bounds: BoundingBox = BoundingBox::new();

    // for i in 0..vertices.len() {
    //     bounds.grow_to_include_vector(vertices[i]);
    // }

    // // Create triangles
    // let mut triangles: Vec<Triangle> = Vec::new();
    // for i in 0..triangle_indices.len() {
    //     let a = vertices[triangle_indices[i] as usize];
    //     let b = vertices[triangle_indices[i + 1] as usize];
    //     let c = vertices[triangle_indices[i + 2] as usize];
    //     let triangle = Triangle::new(a, b, c);
    //     triangles.push(triangle);
    // }

    // // Create root noode (represents entire, un-split mesh), and split it
    // let mut root: Node = Node::new(bounds, triangles);
    // split(&mut root, 0);
}

// https://stackoverflow.com/questions/42264041/how-do-i-get-an-owned-value-out-of-a-box
fn unbox<T>(value: Box<T>) -> T {
    *value
}

fn split(parent: &mut Node, depth: i32, all_nodes: &mut Vec<Node>, all_triangles: &mut Vec<Triangle>) {
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
    let mut child_a: Node = Node::new(BoundingBox::new(), parent.triangle_index);
    let mut child_b: Node = Node::new(BoundingBox::new(), parent.triangle_index);
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

    split(&mut child_a, depth + 1, all_nodes, all_triangles);
    split(&mut child_b, depth + 1, all_nodes, all_triangles);
}

fn main() {
    println!("Hello, world!");
    let mut all_nodes: Vec<Node> = Vec::new();
    let mut all_triangles: Vec<Triangle> = Vec::new();

    // // Load triangle data
    // println!("Loading data...");
    // let triangle_data = include_bytes!("../../objects/knight.bin");
    // let triangle_data = triangle_data.to_vec();
    // let triangle_data = triangle_data.chunks(4).collect::<Vec<_>>();
    // let triangle_data = triangle_data
    //     .iter()
    //     .map(|d| f32::from_ne_bytes([d[0], d[1], d[2], d[3]]))
    //     .collect::<Vec<_>>();
    
    // // Convert to vertices and triangle indices
    // let mut vertices: Vec<Vector> = Vec::new();
    // let mut triangle_indices: Vec<i32> = Vec::new();
    // for i in 0..triangle_data.len() / 3 {
    //     vertices.push(Vector::new(triangle_data[i * 3], triangle_data[i * 3 + 1], triangle_data[i * 3 + 2]));
    // }
    // for i in 0..triangle_data.len() / 3 {
    //     triangle_indices.push(i as i32);
    // }

    // // Build BVH
    // BVH(vertices, triangle_indices);
}