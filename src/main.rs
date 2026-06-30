use log::{debug, info};

use crate::geometry::vectors::Vector3D;

pub mod ray;
pub mod color;
pub mod geometry;

fn ray_color(r : ray::Ray) -> color::Color {
    let unit = r.direction().norm();
    let grad = 0.5 * (unit.y() + 1.0);
    // mix white (1.0, 1.0, 1.0) and blue (0.5, 0.7, 1.0)
    color::Color::new([1.0, 1.0, 1.0]) * (1.0 - grad) + color::Color::new([0.5, 0.7, 1.0]) * grad
}

fn main() {
    // canvas
    let aspect_ratio = 16.0 / 9.0;
    let w_image = 400;
    let h_image = {
        let h_image = w_image / aspect_ratio as i32;
        if h_image  < 1 { 1 } else { h_image }
    };

    // viewport
    let h_viewport = 2.0;
    let w_viewport = h_viewport * w_image as f64 / h_image as f64;
    
    // camera
    let focal_length= 1.0;
    let camera_center = ray::Point3D::empty();

    // horizontal (V_u) and vertical (V_v) vectors 
    let viewport_vu = Vector3D::new([w_viewport, 0.0, 0.0]);
    let viewport_vv = Vector3D::new([0.0, -h_viewport, 0.0]);

    // pixel to pixel delta
    let delta_vu = viewport_vu / w_image as f64;
    let delta_vv = viewport_vv / h_image as f64;

    // upper left pixel
    let viewport_upleft = camera_center
        - viewport_vu / 2.0 - viewport_vv / 2.0 - Vector3D::new([0.0, 0.0, focal_length]);
    let pixel_0 = viewport_upleft + (delta_vu + delta_vv) * 0.5;

    // render
    println!("P3\n{w_image} {h_image}\n255");

    for j in 0..h_image {
        debug!(target : "events.log", "Scanlines remaining: {}", (h_image - j));
        for i in 0..w_image {
            let pixel_center = pixel_0 + (delta_vu * i as f64) + (delta_vv * j as f64);
            let ray_direction = pixel_center - camera_center;
            let r = ray::Ray::new(camera_center, ray_direction);

            let pixel = ray_color(r);
            color::write_color(pixel);
        }
    }
    info!(target : "events.log", "Done");
}
