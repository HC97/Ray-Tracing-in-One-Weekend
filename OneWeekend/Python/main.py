from typedef import Vec3, Point3, Color, MyImage
from utils import random_float
from camera import Camera, ray_color
from sence import random_sense

if __name__ == "__main__" :
    ratio = 3 / 2
    samples_per_pixel = 1
    max_depth = 100

    image_width = 900
    image_height = int(image_width / ratio)
    image = MyImage(image_width, image_height)

    lookfrom = Point3([13, 2, 3])
    lookat = Point3([0, 0, 0])
    vup = Vec3([0, 1, 0])
    dist_to_focus = 10
    aperture = 0
    camera = Camera(lookfrom, lookat, vup, 20, ratio, aperture, dist_to_focus);

    world = random_sense()

    for i in range(image.width):
        for j in range(image.height):
            color = Color([0, 0, 0])
            for n in range(samples_per_pixel):
                u = (i + random_float(0, 1)) / (image.width - 1)
                v = (j + random_float(0, 1)) / (image.height - 1)
                ray = camera.get_ray(u, v)
                color += ray_color(ray, world, max_depth)
            color /= samples_per_pixel
            image.set_pixel((i, image.height - j - 1), color)
    image.show()
