package main

import (
	"image"
	"image/png"
	"os"
	"raytracing/graphics"
)

func main() {
	ratio := 3. / 2
	samples := 100
	depth := 100
	imageWidth := 600
	imageHeight := int(float64(imageWidth) / ratio)

	pos := graphics.NewPoint(13, 2, 3)
	aim := graphics.NewPoint(0, 0, 0)
	vup := graphics.NewVector(0, 1, 0)
	dist := 10.
	aper := 0.
	img := image.NewRGBA(image.Rect(0, 0, imageWidth, imageHeight))
	camera := graphics.NewCamera(pos, aim, vup, 20, ratio, aper, dist)
	world := graphics.RandomSense()
	renderer := graphics.NewRenderer(samples, depth, 8)
	renderer.Render(camera, world, img)

	file, err := os.Create("./test.png")
	if err != nil {
		println(err.Error())
		return
	}
	err = png.Encode(file, img)
	if err != nil {
		println(err.Error())
		return
	}
}
