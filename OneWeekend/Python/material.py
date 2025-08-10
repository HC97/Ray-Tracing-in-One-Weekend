import math

import numpy as np

from typedef import Vec3, Material, Ray, Color, HitRecord
from utils import random_float, random_in_hemisphere

class Lambertian(Material):
    def __init__(self, albedo: Vec3) -> None:
        super().__init__()
        self.albedo = albedo

    def scatter(self, ray: Ray, record: HitRecord) -> tuple[Color, Ray]:
        ref_ray = Ray(record.point, random_in_hemisphere(record.normal))
        return self.albedo, ref_ray

class Metal(Material):
    def __init__(self, albedo: Vec3, fuzz: float) -> None:
        super().__init__()
        self.albedo = albedo
        if fuzz > 1:
            self.fuzz = 1
        else:
            self.fuzz = fuzz
    
    def scatter(self, ray: Ray, record: HitRecord) -> tuple[Color, Ray]:
        ref_dir = reflect(ray.direction, record.normal)
        if ref_dir.dot(record.normal) < 0:
            ref_ray = None
        else:
            ref_ray = Ray(record.point, ref_dir + self.fuzz * random_in_hemisphere(record.normal))
        return self.albedo, ref_ray

class Dielectric(Material):
    def __init__(self, ir) -> None:
        super().__init__()
        self.ir = ir

    def scatter(self, ray: Ray, record: HitRecord) -> tuple[Color, Ray]:
        attenuation = Color([1, 1, 1])
        if record.front_face:
            refraction_ratio = 1 / self.ir
        else:
            refraction_ratio = self.ir
        cos_theta = np.fmin(record.normal.dot(-ray.direction), 1)
        sin_theta = np.sqrt(1 - cos_theta ** 2)
        if refraction_ratio * sin_theta > 1 or reflectance(cos_theta, refraction_ratio) > random_float(0, 1):
            dir = reflect(ray.direction, record.normal)
        else:
            dir = refract(ray.direction, record.normal, refraction_ratio)
        return attenuation, Ray(record.point, dir)

def reflect(dir: Vec3, normal: Vec3) -> Vec3:
    return dir - 2 * dir.dot(normal) * normal

def refract(dir: Vec3, normal: Vec3, etai_over_etat: float) -> Vec3:
    cos_theta = np.fmin(normal.dot(-dir), 1)
    ray_out_prep = etai_over_etat * (dir + cos_theta * normal)
    ray_out_parallel = -np.sqrt(np.fabs(1.0 - ray_out_prep.length_squared())) * normal
    return ray_out_parallel + ray_out_prep

def reflectance(cosine, ref_idx):
    r0 = (1 - ref_idx) / (1 + ref_idx)
    r0 = r0 ** 2
    return r0 + (1 - r0)*math.pow((1 - cosine), 5)
