use crate::vector::Vector;

#[derive(Clone, Copy)]
pub struct Triangle {
    a: Vector,
    b: Vector,
    c: Vector,
    pub center: Vector,
    pub min: Vector,
    pub max: Vector,
}

impl Triangle {
    pub fn new(a: Vector, b: Vector, c: Vector) -> Triangle {
        Triangle {
            a: a,
            b: b,
            c: c,
            center: (a + b + c) / 3.0,
            min: Vector::new(
                a.x.min(b.x).min(c.x),
                a.y.min(b.y).min(c.y),
                a.z.min(b.z).min(c.z),
            ),
            max: Vector::new(
                a.x.max(b.x).max(c.x),
                a.y.max(b.y).max(c.y),
                a.z.max(b.z).max(c.z),
            ),
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
        self.min = Vector::new(
            self.a.x.min(self.b.x).min(self.c.x),
            self.a.y.min(self.b.y).min(self.c.y),
            self.a.z.min(self.b.z).min(self.c.z),
        );
        self.max = Vector::new(
            self.a.x.max(self.b.x).max(self.c.x),
            self.a.y.max(self.b.y).max(self.c.y),
            self.a.z.max(self.b.z).max(self.c.z),
        );
    }

    pub fn set_b(&mut self, b: Vector) {
        self.b = b;
        self.center = (self.a + self.b + self.c) / 3.0;
        self.min = Vector::new(
            self.a.x.min(self.b.x).min(self.c.x),
            self.a.y.min(self.b.y).min(self.c.y),
            self.a.z.min(self.b.z).min(self.c.z),
        );
        self.max = Vector::new(
            self.a.x.max(self.b.x).max(self.c.x),
            self.a.y.max(self.b.y).max(self.c.y),
            self.a.z.max(self.b.z).max(self.c.z),
        );
    }

    pub fn set_c(&mut self, c: Vector) {
        self.c = c;
        self.center = (self.a + self.b + self.c) / 3.0;
        self.min = Vector::new(
            self.a.x.min(self.b.x).min(self.c.x),
            self.a.y.min(self.b.y).min(self.c.y),
            self.a.z.min(self.b.z).min(self.c.z),
        );
        self.max = Vector::new(
            self.a.x.max(self.b.x).max(self.c.x),
            self.a.y.max(self.b.y).max(self.c.y),
            self.a.z.max(self.b.z).max(self.c.z),
        );
    }

    pub fn set(&mut self, a: Vector, b: Vector, c: Vector) {
        self.a = a;
        self.b = b;
        self.c = c;
        self.center = (self.a + self.b + self.c) / 3.0;
        self.min = Vector::new(
            self.a.x.min(self.b.x).min(self.c.x),
            self.a.y.min(self.b.y).min(self.c.y),
            self.a.z.min(self.b.z).min(self.c.z),
        );
        self.max = Vector::new(
            self.a.x.max(self.b.x).max(self.c.x),
            self.a.y.max(self.b.y).max(self.c.y),
            self.a.z.max(self.b.z).max(self.c.z),
        );
    }
}