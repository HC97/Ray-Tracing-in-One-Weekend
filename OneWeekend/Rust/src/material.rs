use super::vector::{Vec3, Color};
use super::camera::Ray;
use super::sence::HitRecord;
use super::utils;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let mut dir = record.normal() + Vec3::random_unit_vector();
        if dir.near_zero() {
            dir = record.normal();
        }
        let scatterd = Ray::new(record.point(), dir);
        let attenuation = self.albedo;
        Some((scatterd, attenuation))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected = ray.direction().unit().reflect(&record.normal());
        let scattered = Ray::new(
            record.point(), reflected + self.fuzz * Vec3::random_unit_vector()
        );
        if scattered.direction().dot(&record.normal()) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    ir: f64
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> bool {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        (r0 + (1.0 - r0) * libm::pow(1.0 - cosine, 5.0)) > utils::randomf(0.0, 1.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if record.front() { 1.0 / self.ir } else { self.ir };
        let unit_direction = ray.direction().unit();
        let cos_theta = libm::fmin((-unit_direction).dot(&record.normal()), 1.0);
        let sin_theta = libm::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) {
            unit_direction.reflect(&record.normal())
        } else {
            unit_direction.refract(&record.normal(), refraction_ratio)
        };
        
        let scatterd = Ray::new(record.point(), direction);
        Some((scatterd, attenuation))
    }
}
