use rtl::vector::{Vec3, Point, Color};
use rtl::camera::Camera;
use rtl::sence::{Sence, Sphere};
use rtl::material::{Lambertian, Metal, Dielectric};
use rtl::utils;
use rtl::Renderer;

fn main() {
    let width = 1600;
    let height = 900;
    let samples = 50;
    let world = {
        let mut sence = Sence::new();
        let ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));
        sence.push(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground));

        for a in -11 .. 11 {
            for b in -11 .. 11 {
                let choose_mat = utils::randomf(0.0, 1.0);
                let center = Point::new(
                    a as f64 + 0.9 * utils::randomf(0.0, 1.0),
                    0.2,
                    b as f64 + 0.9 * utils::randomf(0.0, 1.0)
                );

                if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        let albedo = Color::random(0.0, 1.0) * Color::random(0.0, 1.0);
                        let mat = Lambertian::new(albedo);
                        sence.push(Sphere::new(center, 0.2, mat));
                    } else if choose_mat < 0.95 {
                        let albedo = Color::random(0.5, 1.0);
                        let fuzz = utils::randomf(0.0, 0.5);
                        let mat = Metal::new(albedo, fuzz);
                        sence.push(Sphere::new(center, 0.2, mat));
                    } else {
                        let mat = Dielectric::new(1.5);
                        sence.push(Sphere::new(center, 0.2, mat));
                    }
                }
            }
        }

        let mat1 = Dielectric::new(1.5);
        sence.push(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat1));

        let mat2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
        sence.push(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat2));

        let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
        sence.push(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat3));

        sence
    };
    let camera = {
        let depth = 50;
        let focal_length = 10.0;
        let fov = 20.0;
        let origin = Point::new(13.0, 2.0, 3.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let front = Vec3::new(-13.0, -2.0, -3.0);
        let defocus = 0.6;
        Camera::new(origin, focal_length, fov, depth, vup, front, defocus, width, height)
    };
    let mut renderer = Renderer::new(width, height, samples, camera, world);
    renderer.run();
}
