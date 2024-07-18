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

fn split(parent: &mut Node, depth: i32, all_nodes: &mut Vec<Node>, all_triangles: &Vec<Triangle>) {
    if depth == max_depth {
        return;
    }

    
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