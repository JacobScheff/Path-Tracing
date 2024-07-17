use crate::vector::Vector;
use crate::triangle::Triangle;

pub struct BoundingBox {
    min: Vector,
    max: Vector,
    center: Vector,
}

impl BoundingBox {
    pub fn new(min: Vector, max: Vector) -> BoundingBox {
        BoundingBox {
            min: min,
            max: max,
            center: (min + max) / 2.0,
        }
    }

    pub fn grow_to_include_vector(&mut self, point: &Vector) {
        self.min = self.min.min(&point);
        self.max = self.max.max(&point);
        self.center = (self.min + self.max) / 2.0;
    }

    pub fn grow_to_include(&mut self, triangle: &Triangle) {
        self.grow_to_include_vector(triangle.get_a());
        self.grow_to_include_vector(triangle.get_b());
        self.grow_to_include_vector(triangle.get_c());
    }

}