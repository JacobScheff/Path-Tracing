use crate::vector::Vector;
use crate::triangle::Triangle;
use crate::bounding_box::BoundingBox;

pub struct Node {
    pub bounds: BoundingBox,
    pub triangles: Vec<Triangle>,
    pub child_a: Option<Box<Node>>,
    pub child_b: Option<Box<Node>>,
}

impl Node {
    pub fn new(bounds: BoundingBox, triangles: Vec<Triangle>) -> Node {
        Node {
            bounds: bounds,
            triangles: triangles,
            child_a: None,
            child_b: None,
        }
    }
}