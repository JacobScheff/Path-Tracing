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
    // Create bounding box
    let mut bounds: BoundingBox = BoundingBox::new();

    for i in 0..vertices.len() {
        bounds.grow_to_include_vector(vertices[i]);
    }

    // Create triangles
    let mut triangles: Vec<Triangle> = Vec::new();
    for i in 0..triangle_indices.len() {
        let a = vertices[triangle_indices[i] as usize];
        let b = vertices[triangle_indices[i + 1] as usize];
        let c = vertices[triangle_indices[i + 2] as usize];
        let triangle = Triangle::new(a, b, c);
        triangles.push(triangle);
    }

    // Create root noode (represents entire, un-split mesh), and split it
    let mut root: Node = Node::new(bounds, triangles);
    split(&mut root, 0);
}

fn split(parent: &mut Node, depth: i32) {
    if depth == max_depth {
        return;
    }

    parent.child_a = Some(Box::new(Node::new(BoundingBox::new(), Vec::new())));
    parent.child_b = Some(Box::new(Node::new(BoundingBox::new(), Vec::new())));

    for i in 0..parent.triangles.len() {
        let tri: Triangle = parent.triangles[i];
        let in_a: bool = tri.center.x < parent.bounds.center.x;
        let child: Node = if in_a { parent.child_a } else { parent.child_b };
        child.triangles.push(tri);
        child.bounds.grow_to_include(tri);
    }

    split(parent.child_a, depth + 1);
    split(parent.child_b, depth + 1);
}

fn main() {
    println!("Hello, world!");
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