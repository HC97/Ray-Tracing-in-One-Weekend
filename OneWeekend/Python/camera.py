import numpy as np

from typedef import Vec3, Color, Hittable, Ray
from utils import random_in_uintdisk

class Camera:
    def __init__(self, lookfrom, lookat, vup, fov, ratio, aperture, focus_dist) -> None:
        theta = np.deg2rad(fov)
        h = np.tan(theta / 2)
        viewport_height = 2 * h
        viewport_width = viewport_height * ratio
        self.w = (lookfrom - lookat).unit()
        self.u = Vec3(np.cross(vup, self.w)).unit()
        self.v = Vec3(np.cross(self.w, self.u))
        self.origin = lookfrom
        self.horizontal = focus_dist * viewport_width * self.u
        self.vertical = focus_dist * viewport_height * self.v
        self.lower_left_corner = self.origin - self.horizontal / 2 - self.vertical / 2 - focus_dist * self.w
        self.lens_radius = aperture / 2

    def get_ray(self, s: float, t: float) -> Ray:
        rd = self.lens_radius * random_in_uintdisk()
        offset = self.u * rd.x() + self.v * rd.y()
        direction = self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin
        return Ray(self.origin + offset, direction - offset)

def ray_color(ray: Ray, world: Hittable, depth: int):
    record = world.hit(ray, 0.001, 1000)
    if depth < 0:
        color = Color([0, 0, 0])
    elif record is not None:
        att, ref_ray = record.material.scatter(ray, record)
        if ref_ray is not None:
            color = ray_color(ref_ray, world, depth - 1) * att
        else:
            color = Color([0, 0, 0])
    else:
        time = (ray.direction.y() + 1) / 2
        color = Color([1, 1, 1]) * (1 - time) + Color([0.5, 0.7, 1]) * time
    return color
