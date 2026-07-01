use crate::ray;
use crate::geometry::vectors;

pub struct Hit {
    pub t : f64,
    pub point : ray::Point3D,
    pub normal : vectors::Vector3D
}

pub trait Hittable {
    fn hit(&self, beam : ray::Ray, t_min : f64, t_max : f64) -> Option<Hit>;
}