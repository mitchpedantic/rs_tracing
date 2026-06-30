use crate::geometry::vectors::Vector3D;

pub type Point3D = Vector3D;

pub struct Ray {
    o : Point3D,
    d : Vector3D
}

impl Ray {
    pub fn origin(&self) -> Point3D { self.o }
    pub fn direction(&self) -> Vector3D { self.d }
    pub fn new(o : Point3D, d : Vector3D) -> Ray {
        Ray { o, d }
    }
    pub fn at(&self, t : f64) -> Point3D {
        self.o + self.d * t
    }
}