use crate::ray;
use crate::physics;

pub struct Sphere {
    radius : f64,
    center : ray::Point3D,
}

impl Sphere {
    fn new(radius : f64, center : ray::Point3D) -> Sphere {
        // todo!("NO NEGATIVE RADIUS. Handle with Result")
        Sphere{ radius, center }
    }
}

impl physics::Hittable for Sphere {
    fn hit(&self, beam : crate::ray::Ray, t_min : f64, t_max : f64) -> Option<physics::Hit> {
        let center_to_camera = self.center - beam.origin();
        let a = beam.direction().sqr_len();
        let h = beam.direction().dot(&center_to_camera);
        let c = center_to_camera.sqr_len() - self.radius*self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None
        }

        // nearest root that lies in the acceptable range
        let root_option = {
            match f64::sqrt(discriminant) {
                x if (x >= h - t_min * a) || (x <= h - t_max * a) => Some((h - x) / a),
                x if (x <= t_min * a - h) || (x >= t_max * a - h) => Some((h + x) / a),
                _ => None
            }
        };
        
        if let Some(root) = root_option {
            let point = beam.at(root);
            Some(physics::Hit{ t : root, point : point, normal : (point - self.center) / self.radius})
        } else {
            None
        }
    }
}