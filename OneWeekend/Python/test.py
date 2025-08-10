import taichi as ti
ti.init(arch=ti.cpu)

width, height = 640,480
# Create a 640x480 scalar field, each of its elements representing a pixel value (f32)
gray_scale_image = ti.field(dtype=ti.f32, shape=(width, height))

@ti.kernel
def fill_image():
    # Fill the image with random gray
    for i,j in gray_scale_image:
        gray_scale_image[i,j] = ti.random()

fill_image()
# Create a GUI of same size as the gray-scale image
gui = ti.GUI('gray-scale image with random values', (width, height))
while gui.running:
    gui.set_image(gray_scale_image)
    gui.show()
