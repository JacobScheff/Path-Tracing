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
        self.min.x = if triangle.min.x < self.min.x { triangle.min.x } else { self.min.x };
        self.min.y = if triangle.min.y < self.min.y { triangle.min.y } else { self.min.y };
        self.min.z = if triangle.min.z < self.min.z { triangle.min.z } else { self.min.z };

        self.max.x = if triangle.max.x > self.max.x { triangle.max.x } else { self.max.x };
        self.max.y = if triangle.max.y > self.max.y { triangle.max.y } else { self.max.y };
        self.max.z = if triangle.max.z > self.max.z { triangle.max.z } else { self.max.z };

        self.center = (self.min + self.max) / 2.0;
    }

}