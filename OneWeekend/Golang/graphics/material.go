package graphics

import (
	"math"
)

type Material interface {
	scatter(Ray, HitRecord) (Color, Ray, bool)
}

type Lambertian struct {
	albedo Color
}

func (lam Lambertian) scatter(ray Ray, record HitRecord) (Color, Ray, bool) {
	refDir := randomVectorInHemisphere(record.normal)
	refRay := NewRay(record.point, refDir)
	return lam.albedo, refRay, true
}

type Metal struct {
	albedo Color
	fuzz   float64
}

func (metal Metal) scatter(ray Ray, record HitRecord) (Color, Ray, bool) {
	refDir := reflect(ray.direction, record.normal)
	if metal.fuzz > 1 {
		metal.fuzz = 1
	}
	refDir = refDir.add(randomUintVector().mutiply(metal.fuzz))
	if refDir.dot(record.normal) < 0 {
		return metal.albedo, Ray{}, false
	} else {
		refRay := NewRay(record.point, refDir)
		return metal.albedo, refRay, true
	}
}

type Dielectric struct {
	ir float64
}

func (die Dielectric) scatter(ray Ray, record HitRecord) (Color, Ray, bool) {
	attenuation := Color{[3]float64{1, 1, 1}}
	refractionRatio := 0.
	if record.front {
		refractionRatio = 1 / die.ir
	} else {
		refractionRatio = die.ir
	}
	var dir Vector
	cos := math.Min(record.normal.dot(ray.direction.reverse()), 1)
	sin := math.Sqrt(1 - cos*cos)
	if refractionRatio*sin > 1 || reflectance(cos, refractionRatio) > randomFloat(0, 1) {
		dir = reflect(ray.direction, record.normal)
	} else {
		dir = refract(ray.direction, record.normal, refractionRatio)
	}
	return attenuation, NewRay(record.point, dir), true
}

func reflect(dir Vector, normal Vector) Vector {
	return dir.sub(normal.mutiply(dir.dot(normal) * 2))
}

func refract(dir Vector, normal Vector, etaiOverEtat float64) Vector {
	cos := math.Min(normal.dot(dir.reverse()), 1)
	rayOutPrep := dir.add(normal.mutiply(cos)).mutiply(etaiOverEtat)
	rayOutParallel := normal.mutiply(-math.Sqrt(math.Abs(1 - rayOutPrep.dot(rayOutPrep))))
	return rayOutPrep.add(rayOutParallel)
}

func reflectance(cosine float64, refIdx float64) float64 {
	r := (1 - refIdx) / (1 + refIdx)
	r = r * r
	return r + (1-r)*math.Pow((1-cosine), 5)
}
