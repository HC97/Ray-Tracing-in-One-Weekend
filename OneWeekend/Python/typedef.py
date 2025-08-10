import abc
from dataclasses import dataclass

import numpy as np

from matplotlib import pyplot as plt
from PIL import Image

class MyImage:
    def __init__(self, width, height) -> None:
        self.width = width
        self.height = height
        self.image = Image.new("RGB", (width, height))
    
    def set_pixel(self, point, color):
        color = np.sqrt(color)
        color *= 256
        self.image.putpixel(point, (color[0], color[1], color[2]))

    def show(self):
        plt.imshow(self.image)
        plt.show()

class Vector(np.ndarray):
    def __new__(cls, data):
        return super().__new__(cls, (len(data),), np.float32, np.array(data, np.float32))

class Vec3(Vector):
    def __new__(cls, data: list[float]):
        return super().__new__(cls, data)

    def x(self):
        return self[0]

    def y(self):
        return self[1]

    def z(self):
        return self[2]

    def unit(self):
        length = self.length()
        return Vec3([self[0] / length, self[1] / length, self[2] / length])

    def length(self):
        return np.sqrt(self.length_squared())

    def length_squared(self):
        return self.dot(self)

Color = Vec3
Point3 = Vec3

class Ray:
    def __init__(self, origin: Point3, direction: Vec3) -> None:
        self.origin = origin
        self.direction = direction.unit()

    def at(self, time):
        return self.origin + self.direction * time

class Material(abc.ABC):
    @abc.abstractmethod
    def scatter(self, ray, record) -> tuple[Color, Ray]:
        pass

@dataclass
class HitRecord:
    point: Point3
    normal: Vec3
    time: float
    front_face: bool
    material: Material

class Hittable(abc.ABC):
    @abc.abstractmethod
    def hit(self, ray, t_min, t_max) -> HitRecord:
        pass
