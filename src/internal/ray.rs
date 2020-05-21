use crate::internal::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn project(&self, temp: f64) -> Vec3 {
        self.orig + self.dir * temp
    }
}