mod vector;
use vector::Vector;
mod triangle;
use triangle::Triangle;
mod bounding_box;
use bounding_box::BoundingBox;
mod node;
use node::Node;

fn BVH(vertices: Vec<Vector>, triangle_indices: Vec<i32>) {
    // Create bounding box
    let mut bounds: BoundingBox = BoundingBox::new();

    for i in 0..vertices.len() {
        bounds.grow_to_include_vector(vertices[i]);
    }
}

fn main() {
    println!("Hello, world!");
}