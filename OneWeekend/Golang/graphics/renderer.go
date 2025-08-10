package graphics

import (
	"image/color"
	"image/draw"
	"math"
)

type Renderer struct {
	sample int
	depth  int
	thread int
}

func NewRenderer(sample, depth, thread int) Renderer {
	return Renderer{sample, depth, thread}
}

func (ren Renderer) Render(camera Camera, sence HittableList, image draw.Image) {
	width := image.Bounds().Dx()
	height := image.Bounds().Dy()
	buffer := make([][]Color, height)
	for i := 0; i < height; i++ {
		buffer[i] = make([]Color, width)
	}
	if ren.thread <= 1 {
		ren.serialRender(camera, sence, width, height, buffer)
	} else {
		ren.parallelRender(camera, sence, width, height, buffer)
	}
	ren.writeImage(buffer, image, width, height)
}

func (ren Renderer) serialRender(camera Camera, sence HittableList, width, height int, buffer [][]Color) {
	for i := height - 1; i >= 0; i-- {
		for j := 0; j < width; j++ {
			for n := 1; n <= ren.sample; n++ {
				u := (float64(j) + randomFloat(0, 1)) / float64(width)
				v := (float64(height-i-1) + randomFloat(0, 1)) / float64(height)
				ray := camera.getRay(u, v)
				color := rayColor(ray, sence, ren.depth)
				buffer[i][j] = buffer[i][j].add(color)
			}
		}
	}
}

func (ren Renderer) parallelRender(camera Camera, sence HittableList, width, height int, buffer [][]Color) {
	count := ren.thread
	work := make(chan [2]int, count)
	signal := make(chan bool)
	for c := 0; c < count; c++ {
		go func() {
			for point := range work {
				i, j := point[0], point[1]
				for n := 1; n <= ren.sample; n++ {
					u := (float64(j) + randomFloat(0, 1)) / float64(width)
					v := (float64(height-i-1) + randomFloat(0, 1)) / float64(height)
					ray := camera.getRay(u, v)
					color := rayColor(ray, sence, ren.depth)
					buffer[i][j] = buffer[i][j].add(color)
				}
			}
			signal <- true
		}()
	}
	for i := height - 1; i >= 0; i-- {
		for j := 0; j < width; j++ {
			work <- [2]int{i, j}
		}
	}
	close(work)
	for c := 0; c < count; c++ {
		<-signal
	}
	close(signal)
}

func (ren Renderer) writeImage(buffer [][]Color, image draw.Image, width, height int) {
	for i := 0; i < height; i++ {
		for j := 0; j < width; j++ {
			c := buffer[i][j].mutiply(1 / float64(ren.sample))
			r := uint8(255 * math.Sqrt(c.x()))
			g := uint8(255 * math.Sqrt(c.y()))
			b := uint8(255 * math.Sqrt(c.z()))
			color := color.RGBA{r, g, b, 0xff}
			image.Set(j, i, color)
		}
	}
}

func rayColor(ray Ray, sence HittableList, depth int) Color {
	if depth < 0 {
		return Color{[3]float64{0, 0, 0}}
	}
	record, ok := sence.hit(ray, 0.01, 1000)
	if ok {
		att, refRay, ok := record.material.scatter(ray, record)
		if ok {
			return rayColor(refRay, sence, depth-1).mul(att)
		} else {
			return Color{[3]float64{0, 0, 0}}
		}
	} else {
		t := (ray.direction.unit().y() + 1) / 2
		return Color{[3]float64{1, 1, 1}}.mutiply(1 - t).add(Color{[3]float64{0.5, 0.7, 1}}.mutiply(t))
	}
}
