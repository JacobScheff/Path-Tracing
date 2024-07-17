use crate::vector::Vector;

pub struct Triangle {
    a: Vector,
    b: Vector,
    c: Vector,
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
        Triangle {
            a: a,
            b: b,
            c: c,
        }
    }

    pub fn get_a(&self) -> &Vector {
        &self.a
    }

    pub fn get_b(&self) -> &Vector {
        &self.b
    }

    pub fn get_c(&self) -> &Vector {
        &self.c
    }

    pub fn set_a(&mut self, a: Vector) {
        self.a = a;
    }

    pub fn set_b(&mut self, b: Vector) {
        self.b = b;
    }

    pub fn set_c(&mut self, c: Vector) {
        self.c = c;
    }

    pub fn set(&mut self, a: Vector, b: Vector, c: Vector) {
        self.a = a;
        self.b = b;
        self.c = c;
    }
}