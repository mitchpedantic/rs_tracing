use std::ops::{Sub, SubAssign, Add, AddAssign, Neg, Mul, MulAssign, Div, DivAssign};
use std::iter::zip;

#[derive(Copy, Clone)]
pub struct Vector3D {
    edges : [f64; 3],
}

impl Vector3D {
    pub fn x(&self) -> f64 { self.edges[0] }

    pub fn y(&self) -> f64 { self.edges[1] }
    
    pub fn z(&self) -> f64 { self.edges[2] }
    
    pub fn empty() -> Vector3D { Vector3D::new([0.0; 3]) }
    
    pub fn new(edges : [f64; 3]) -> Vector3D { Vector3D { edges } }
    
    pub fn len(&self) -> f64 { f64::sqrt(self.sqr_len()) }
    
    pub fn sqr_len(&self) -> f64 { self.edges.iter().fold(0.0, |acc, e| { acc + e * e}) }
    
    pub fn dot(&self, rhs : &Vector3D) -> f64 {
        zip(self.edges, rhs.edges).fold(0.0, |acc, (w, v) | acc + w * v)
    }
    
    pub fn cross(&self, rhs : &Vector3D) -> Vector3D {
        Vector3D::new([
                self.y() * rhs.z() - self.z() * rhs.y(),
                self.z() * rhs.x() - self.x() * rhs.z(),
                self.x() * rhs.y() - self.y() * rhs.x()
        ])
    }

    pub fn norm(&self) -> Vector3D {
        self.clone() / self.len()
    }
}

impl Sub for Vector3D {
    type Output = Vector3D;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]
        )
    }
}

impl SubAssign<&Vector3D> for Vector3D {
    fn sub_assign(&mut self, rhs: &Self) {
        *self = Vector3D::new(
            [self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]
        );
    }
}

impl Add for Vector3D {
    type Output = Vector3D;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output::new(
            [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()]
        )
    }
}

impl AddAssign<&Vector3D> for Vector3D {
    fn add_assign(&mut self, rhs: &Self) {
        *self = Self::new(
            [self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()]
        );
    }
}

impl Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(
            [-self.x(), -self.y(), -self.z()]
        )
    }
}

impl Mul for Vector3D {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(
            [self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z()]
        )
    }
}

impl Mul<f64> for Vector3D {
    type Output = Vector3D;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output::new(
            self.edges.map(|e : f64| e*rhs)
        )
    }
}

impl MulAssign<f64> for Vector3D {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self::new(
            self.edges.map(|e : f64| e*rhs)
        );
    }
}

impl Div<f64> for Vector3D {
    type Output = Vector3D;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl DivAssign<f64> for Vector3D {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs;
    }
}
