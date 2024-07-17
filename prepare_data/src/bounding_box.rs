mod vector;
use vector::Vector;
mod triangle;
use triangle::triangle;

struct bounding_box {
    min: Vector,
    max: Vector,
    center: Vector,
}

impl bounding_box {
    fn new(min: Vector, max: Vector) -> bounding_box {
        bounding_box {
            min: min,
            max: max,
            center: (min + max) / 2.0,
        }
    }

    fn grow_to_include(&mut self, point: Vector) {
        self.min = self.min.min(&point);
        self.max = self.max.max(&point);
        self.center = (self.min + self.max) / 2.0;
    }

    fn grow_to_include(&mut self, triangle: &triangle) {
        self.grow_to_include(triangle.get_a());
        self.grow_to_include(triangle.get_b());
        self.grow_to_include(triangle.get_c());
    }

}