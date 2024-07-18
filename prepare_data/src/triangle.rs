use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Triangle {
    a: Vector,
    b: Vector,
    c: Vector,
    pub center: Vector,
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
        Triangle {
            a: a,
            b: b,
            c: c,
            center: (a + b + c) / 3.0,
        }
    }

    pub fn get_a(&self) -> Vector {
        self.a
    }

    pub fn get_b(&self) -> Vector {
        self.b
    }

    pub fn get_c(&self) -> Vector {
        self.c
    }

    pub fn set_a(&mut self, a: Vector) {
        self.a = a;
        self.center = (self.a + self.b + self.c) / 3.0;
    }

    pub fn set_b(&mut self, b: Vector) {
        self.b = b;
        self.center = (self.a + self.b + self.c) / 3.0;
    }

    pub fn set_c(&mut self, c: Vector) {
        self.c = c;
        self.center = (self.a + self.b + self.c) / 3.0;
    }

    pub fn set(&mut self, a: Vector, b: Vector, c: Vector) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.center = (self.a + self.b + self.c) / 3.0;
    }
}