import numpy as np

from typedef import Vec3

def random_float(min, max) -> float:
    return min + (max - min) * np.random.rand()

def random_uint_vector() -> Vec3:
    a = random_float(0, 2 * np.pi)
    z = random_float(-1, 1)
    r = np.sqrt(1 - z ** 2)
    return Vec3([r * np.cos(a), r * np.sin(a), z])

def random_vector(min, max) -> Vec3:
    v = [random_float(min, max), random_float(min, max), random_float(min, max)]
    return Vec3(v)

def random_in_hemisphere(normal: Vec3) -> Vec3:
    v = [random_float(0, 1), random_float(0, 1), random_float(0, 1)]
    v = Vec3(v)
    if v.dot(normal) < 0:
        v = -v
    return v

def random_in_uintdisk() -> Vec3:
    while True:
        p = [random_float(0, 1), random_float(0, 1)]
        v = Vec3([*p, 0])
        if v.length_squared() < 1:
            return v
