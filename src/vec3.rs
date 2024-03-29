use rand::prelude::Rng;



#[derive(Debug, Clone)]
pub struct Vec3 {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl Vec3 {
    pub fn new(a: f32, b: f32, c: f32) -> Self {
        Vec3 { a, b, c }
    }
    
    //random vec3 with values inbetweeen 0 and 10
    pub fn random() -> Self{
        let mut rng = rand::thread_rng();
        Self::new(rng.gen::<f32>() * 10.0, rng.gen::<f32>() * 10.0, rng.gen::<f32>() * 10.0)
    }

    pub fn zeroes() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    pub fn ones() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }

    pub fn scale(&mut self, a: f32) {
        self.map(|b| b * a);
    }
    pub fn div(&mut self, a: f32){
        self.map(|b| b / a)
    }

    pub fn add(&mut self, other: &Self) {
        self.combine(other, |a, b| a + b);
    }

    pub fn mag_squared(&self) -> f32{
        self.a.powf(2.0) + self.b.powf(2.0) + self.c.powf(2.0)
    }

    pub fn magnitude(&self) -> f32{
        self.mag_squared().sqrt()
    }

    pub fn add_scaled(&mut self, other: &Self, s: f32) {
        self.a += other.a * s;
        self.b += other.b * s;
        self.c += other.c * s;
    }

    pub fn sub(&mut self, other: &Self) {
        self.combine(other, |a, b| a - b);
    }

    pub fn sum(&self) -> f32 {
        self.a + self.b + self.c
    }

    pub fn elwise_mul(&mut self, other: &Self) {
        self.combine(other, |a, b| a * b);
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.a * other.a + self.b * other.b + self.c * other.c
    }

    pub fn map<F: Fn(f32) -> f32>(&mut self, f: F) {
        self.a = f(self.a);
        self.b = f(self.b);
        self.c = f(self.c);
    }

    pub fn combine<F: Fn(f32, f32) -> f32>(&mut self, other: &Self, f: F) {
        self.a = f(self.a, other.a);
        self.b = f(self.b, other.b);
        self.c = f(self.c, other.c)
    }

    pub fn extreme(&self) -> f32{
        
        let abs_a = self.a.abs();
        let abs_b = self.b.abs();
        let abs_c = self.c.abs();

        if abs_a >= abs_b && abs_a >= abs_c {
            self.a
        }
        else if abs_b >= abs_a && abs_b >= abs_c {
            self.b
        }
        else {
            self.c
        }
    }

    pub fn distance_squared_from(&self, other: &Self) -> f32 {
        (other.a - self.a).powf(2.0) +
        (other.b - self.b).powf(2.0) +
        (other.c - self.c).powf(2.0)
    }

    pub fn within_distance(&self, other: &Self, r: f32) -> bool {
        self.distance_squared_from(other) <= r
    }

    pub fn get_projection(&mut self, other: &Self) {

        self.scale(self.dot(other) / self.mag_squared());

    }

    pub fn normalize(&mut self){
        self.div(self.magnitude())
    }
}
