use crate::vector::Vector;
use crate::triangle::Triangle;
use crate::bounding_box::BoundingBox;

#[derive(Clone, Copy)]
pub struct Node {
    pub bounds: BoundingBox,
    pub triangle_index: i32,
    pub triangle_count: i32,
    pub child_index: i32,
}

impl Node {
    pub fn new(bounds: BoundingBox, triangle_index: i32, triangle_count: i32) -> Node {
        Node {
            bounds: bounds,
            triangle_index,
            triangle_count,
            child_index: 0,
        }
    }
}