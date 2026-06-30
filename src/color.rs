use crate::geometry::vectors::Vector3D;

pub type Color = Vector3D;

pub fn write_color(pixel : Color) {
    let ir = (255.999 * pixel.x()) as i16;
    let ig = (255.999 * pixel.y()) as i16;
    let ib = (255.999 * pixel.z()) as i16;
    
    println!("{ir} {ig} {ib}");

}