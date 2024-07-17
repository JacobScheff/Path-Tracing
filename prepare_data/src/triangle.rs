mod vector;
use vector::Vector;

struct triangle {
    a: Vector,
    b: Vector,
    c: Vector,
}

impl triangle {
    fn new(a: Vector, b: Vector, c: Vector) -> triangle {
        triangle {
            a: a,
            b: b,
            c: c,
        }
    }

    fn get_a(&self) -> &Vector {
        &self.a
    }

    fn get_b(&self) -> &Vector {
        &self.b
    }

    fn get_c(&self) -> &Vector {
        &self.c
    }

    fn set_a(&mut self, a: Vector) {
        self.a = a;
    }

    fn set_b(&mut self, b: Vector) {
        self.b = b;
    }

    fn set_c(&mut self, c: Vector) {
        self.c = c;
    }

    fn set(&mut self, a: Vector, b: Vector, c: Vector) {
        self.a = a;
        self.b = b;
        self.c = c;
    }
}