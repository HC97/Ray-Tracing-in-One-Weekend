import numpy as np

from typedef import Point3, Color, Hittable, HitRecord, Material
from material import Lambertian, Metal, Dielectric
from utils import random_float, random_vector

class HittableList(Hittable):
    def __init__(self) -> None:
        super().__init__()
        self.list = []

    def clear(self):
        self.list.clear()

    def add(self, object: Hittable):
        self.list.append(object)

    def hit(self, ray, t_min, t_max) -> HitRecord:
        time = t_max
        record = None
        for object in self.list:
            rec = object.hit(ray, t_min, t_max)
            if rec is not None and rec.time < time:
                record = rec
                time = rec.time
        return record

class Sphere(Hittable):
    def __init__(self, center, radius, material):
        super().__init__()
        self.center = center
        self.radius = radius
        self.material = material

    def hit(self, ray, t_min, t_max) -> HitRecord:
        oc = ray.origin - self.center
        dir = ray.direction
        a = dir.length_squared()
        b = 2 * dir.dot(oc)
        c = oc.length_squared() - self.radius ** 2
        dis = b ** 2 - 4 * a * c
        if dis < 0:
            return None
        sqrtd = np.sqrt(dis)
        time = (-b - sqrtd) / (2 * a)
        if time < t_min or time > t_max:
            time = (-b + sqrtd) / (2 * a)
            if time < t_min or time > t_max:
                return None
        point = ray.at(time)
        front_face = True
        normal = (point - self.center) / self.radius
        if ray.direction.dot(normal) > 0:
            normal = -normal
            front_face = False
        return HitRecord(point, normal, time, front_face, self.material)

def random_sense() -> HittableList:
    world  = HittableList()
    ground_material = Lambertian(Color([0.5, 0.5, 0.5]))
    world.add(Sphere(Point3([0,-1000,0]), 1000, ground_material))

    for a in range(-11, 11):
        for b in range(-11, 11):
            choose_mat = random_float(0, 1)
            center = Point3([a + 0.9 * random_float(0, 1), 0.2, b + 0.9 * random_float(0, 1)])
            if ((center - Point3([4, 0.2, 0])).length() > 0.9):
                if (choose_mat < 0.8):
                    albedo = random_vector(0, 1) * random_vector(0, 1)
                    sphere_material = Lambertian(albedo)
                    world.add(Sphere(center, 0.2, sphere_material))
                elif choose_mat < 0.95:
                    albedo = random_vector(0.5, 1)
                    fuzz = random_float(0, 0.5)
                    sphere_material = Metal(albedo, fuzz)
                    world.add(Sphere(center, 0.2, sphere_material))
                else:
                    sphere_material = Dielectric(1.5)
                    world.add(Sphere(center, 0.2, sphere_material))

    material1 = Dielectric(1.5)
    world.add(Sphere(Point3([0, 1, 0]), 1.0, material1))

    material2 = Lambertian(Color([0.4, 0.2, 0.1]))
    world.add(Sphere(Point3([-4, 1, 0]), 1.0, material2))

    material3 = Metal(Color([0.7, 0.6, 0.5]), 0.0)
    world.add(Sphere(Point3([4, 1, 0]), 1.0, material3))

    return world
