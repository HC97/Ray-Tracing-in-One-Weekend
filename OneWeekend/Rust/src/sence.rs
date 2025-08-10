use std::rc::Rc;

use super::vector::{Vec3, Point};
use super::camera::Ray;
use super::material::Material;
use super::utils::Interval;

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}

pub struct HitRecord {
    point: Point,
    normal: Vec3,
    front: bool,
    material: Rc<dyn Material>,
    time: f64
}

impl HitRecord {
    fn new(
        point: Point, normal: Vec3, front: bool, material: Rc<dyn Material>, time: f64
    ) -> Self {
        Self { point, normal, front, material, time }
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn front(&self) -> bool {
        self.front
    }

    pub fn material(&self) -> Rc<dyn Material> {
        self.material.clone()
    }
}

pub struct Sence {
    objects: Vec<Box<dyn Hittable>>
}

impl Sence {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn push(&mut self, hittable: impl Hittable + 'static) {
        self.objects.push(Box::new(hittable));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for Sence {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut record = None;
        let mut max_time = interval.max();
        for object in &self.objects {
            if let Some(rec) = object.hit(
                ray, Interval::new(interval.min(), max_time)
            ) {
                if rec.time < max_time {
                    max_time = rec.time;
                    record = Some(rec);
                }
            }
        }
        record
    }
}

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: impl Material + 'static) -> Self {
        Self { center, radius, material: Rc::new(material) }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let dis = h * h - a * c;
        if dis < 0.0 {
            return None;
        }

        let mut root = (h - libm::sqrt(dis)) / a;
        if !interval.surrounds(root) {
            root = (h + libm::sqrt(dis)) / a;
            if !interval.surrounds(root) {
                return None;
            }
        }
        let time = root;
        let point = ray.at(time);
        let (front, normal) = {
            let n = (point - self.center) / self.radius;
            if n.dot(&ray.direction()) < 0.0 {
                (true, n)
            } else {
                (false, -n)
            }
        };
        Some(HitRecord::new(point, normal, front, self.material.clone(), time))
    }
}
