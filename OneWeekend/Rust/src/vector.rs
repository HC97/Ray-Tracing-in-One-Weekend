use std::ops::{Neg, Add, Sub, Mul, Div, AddAssign, MulAssign, DivAssign};
use super::utils;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec3 {
    data: [f64; 3]
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn random(min: f64, max: f64) -> Self {
        let x = utils::randomf(min, max);
        let y = utils::randomf(min, max);
        let z = utils::randomf(min, max);
        Self::new(x, y, z)
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(utils::randomf(-1.0, 1.0), utils::randomf(-1.0, 1.0), 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit()
    }

    pub fn random_on_hemisphere(normal: &Self) -> Self {
        let uv = Self::random_unit_vector();
        if uv.dot(normal) > 0.0 {
            uv
        } else {
            -uv
        }
    }

    pub fn reflect(&self, normal: &Self) -> Self {
        *self - self.dot(normal) * 2.0 * *normal
    }

    pub fn refract(&self, normal: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = libm::fmin((-*self).dot(normal), 1.0);
        let r_out_prep = etai_over_etat * (*self + cos_theta * *normal);
        let r_oup_parallel = -libm::sqrt(
            libm::fabs(1.0 - r_out_prep.length_squared())
        ) * *normal;
        r_out_prep + r_oup_parallel
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        libm::fabs(self.x()) < s && libm::fabs(self.y()) < s && libm::fabs(self.z()) < s
    }

    pub fn x(&self) -> f64 {
        self.data[0]
    }

    pub fn y(&self) -> f64 {
        self.data[1]
    }

    pub fn z(&self) -> f64 {
        self.data[2]
    }

    pub fn length(&self) -> f64 {
        libm::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y() * rhs.z() - self.z() * rhs.y(),
            self.z() * rhs.x() - self.x() * rhs.z(),
            self.x() * rhs.y() - self.y() * rhs.x()
        )
    }

    pub fn unit(self) -> Self {
        self / self.length()
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x() + rhs.x(),
            self.y() + rhs.y(),
            self.z() + rhs.z()
        )
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.data[0] += rhs.data[0];
        self.data[1] += rhs.data[1];
        self.data[2] += rhs.data[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z()
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(
            self.data[0] * rhs,
            self.data[1] * rhs,
            self.data[2] * rhs,
        )
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.data[0] *= rhs;
        self.data[1] *= rhs;
        self.data[2] *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.data[0] /= rhs;
        self.data[1] /= rhs;
        self.data[2] /= rhs;
    }
}

pub type Point = Vec3;
pub type Color = Vec3;
