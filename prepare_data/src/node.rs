use crate::vector::Vector;
use crate::triangle::Triangle;
use crate::bounding_box::BoundingBox;

pub struct Node {
    bounds: BoundingBox,
    triangles: Vec<Triangle>,
    child_a: Option<Box<Node>>,
    child_b: Option<Box<Node>>,
}

impl Node {
    pub fn new(bounds: BoundingBox) -> Node {
        Node {
            bounds: bounds,
            triangles: Vec::new(),
            child_a: None,
            child_b: None,
        }
    }

    
}