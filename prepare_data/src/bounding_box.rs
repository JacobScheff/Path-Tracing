use crate::vector::Vector;
use crate::triangle::Triangle;

#[derive(Clone, Copy)]
pub struct BoundingBox {
    pub min: Vector,
    pub max: Vector,
    pub center: Vector,
}

impl PartialEq for BoundingBox {
    fn eq(&self, other: &BoundingBox) -> bool {
        self.min == other.min && self.max == other.max && self.center == other.center
    }
}

impl BoundingBox {
    pub fn new() -> BoundingBox {
        let min = Vector::new(std::f32::INFINITY, std::f32::INFINITY, std::f32::INFINITY);
        let max = Vector::new(-std::f32::INFINITY, -std::f32::INFINITY, -std::f32::INFINITY);
        BoundingBox {
            min: min,
            max: max,
            center: (min + max) / 2.0,
        }
    }

    pub fn grow_to_include_vector(&mut self, point: Vector) {
        self.min = self.min.min(point);
        self.max = self.max.max(point);
        self.center = (self.min + self.max) / 2.0;
    }

    pub fn grow_to_include(&mut self, triangle: Triangle) {
        self.grow_to_include_vector(triangle.get_a());
        self.grow_to_include_vector(triangle.get_b());
        self.grow_to_include_vector(triangle.get_c());
    }

}