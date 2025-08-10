use std::vec;
use std::iter;

use super::vector::{Vec3, Point, Color};
use super::sence::{Sence, Hittable};
use super::utils::{self, Interval};

#[derive(Debug, Default, Clone, Copy)]
pub struct Camera {
    depth: u32,
    origin: Point,
    viewport: Viewport,
    fdist: (Vec3, Vec3)
}

impl Camera {
    pub fn new(
        origin: Point, focal: f64, fov: f64, depth: u32,
        vup: Vec3, front:Vec3, defocus: f64, w: u32, h: u32
    ) -> Self {
        let (viewport, fdist) = {
            let right = front.cross(&vup).unit();
            let up = right.cross(&front).unit();
            let center = focal * front.unit() + origin;
            let height = focal * libm::tan(utils::degrees_to_radians(fov / 2.0)) * 2.0;
            let width = height * (w as f64 / h as f64);
            let x = width * right;
            let y = height * up;
            let o = center - x / 2.0 + y / 2.0;
            let fradius = focal * libm::tan(utils::degrees_to_radians(defocus / 2.0));
            (Viewport::new(o, x, -y, w, h), (right * fradius, up * fradius))
        };
        Self { depth, origin, viewport, fdist }
    }

    pub fn render(&self, world: &Sence, width: u32, height: u32) -> Vec<Vec3> {
        let mut sample = Vec::new();
        sample.resize((width * height) as usize, Vec3::default());
        for (color, point) in iter::zip(&mut sample, &self.viewport) {
            let origin = self.defocus_disk_sample();
            let ray = Ray::new(origin, point - origin);
            *color = self.ray_color(&ray, &world, self.depth);
        }
        sample
    }

    pub fn ray_color(&self, ray: &Ray, world: &Sence, depth: u32) -> Color {
        if depth <= 0 {
            return Color::default();
        }
        if let Some(rec) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some((scatterd, attenuation)) = rec.material().scatter(ray, &rec) {
                return attenuation * self.ray_color(&scatterd, world, depth - 1);
            }
            return Color::default();
        }
        let unit = ray.direction().unit();
        let a = 0.5 * (unit.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    fn defocus_disk_sample(&self) -> Point {
        let p = Vec3::random_in_unit_disk();
        self.origin + (p.x() * self.fdist.0) + (p.y() * self.fdist.1)
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Viewport {
    o: Point,
    x: Vec3,
    y: Vec3,
    w: u32,
    h: u32
}

impl Viewport {
    fn new(o: Point, x: Vec3, y: Vec3, w: u32, h: u32) -> Self {
        Self { o, x, y, w, h }
    }

    fn point_sample(origin: Point, i: u32, j: u32, dx: Vec3, dy: Vec3) -> Point {
        let i = i as f64 + utils::randomf(0.0, 1.0);
        let j = j as f64 + utils::randomf(0.0, 1.0);
        origin + i * dy + j * dx
    }

    fn iter(&self) -> vec::IntoIter<Vec3> {
        let dx = self.x / self.w as f64;
        let dy = self.y / self.h as f64;
        let mut points = Vec::new();
        for i in 0 .. self.h {
            for j in 0 .. self.w {
                let point = Self::point_sample(self.o, i, j, dx, dy);
                points.push(point);
            }
        }
        points.into_iter()
    }
}

impl IntoIterator for &Viewport {
    type Item = Point;
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    origin: Point,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn at(&self, time: f64) -> Point {
        self.origin + time * self.direction
    }
}
