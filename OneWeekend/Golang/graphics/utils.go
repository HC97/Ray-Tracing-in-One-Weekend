package graphics

import "math/rand"

func randomFloat(min, max float64) float64 {
	return min + (max-min)*rand.Float64()
}

func randomVector() Vector {
	x := rand.Float64()
	y := rand.Float64()
	z := rand.Float64()
	return Vector{[3]float64{x, y, z}}
}

func randomUintVector() Vector {
	vec := randomVector()
	return vec.unit()
}

func randomVectorInHemisphere(normal Vector) Vector {
	vec := randomUintVector()
	if vec.dot(normal) < 0 {
		vec = vec.reverse()
	}
	return vec
}

func randomColor(min, max float64) Vector {
	x := randomFloat(min, max)
	y := randomFloat(min, max)
	z := randomFloat(min, max)
	return Vector{[3]float64{x, y, z}}
}

func randomInUnitDisk() Vector {
	for {
		p := NewVector(randomFloat(-1, 1), randomFloat(-1, 1), 0)
		if p.length() >= 1 {
			continue
		}
		return p
	}
}

func RandomWorld() HittableList {
	var world HittableList
	material_ground := Lambertian{NewColor(0.8, 0.8, 0.0)}
	material_center := Lambertian{NewColor(0.1, 0.2, 0.5)}
	material_left := Dielectric{1.5}
	material_right := Metal{NewColor(0.8, 0.6, 0.2), 0}

	world.add(Sphere{NewPoint(0.0, -100.5, -1.0), 100.0, material_ground})
	world.add(Sphere{NewPoint(0.0, 0.0, -1.0), 0.5, material_center})
	world.add(Sphere{NewPoint(-1.0, 0.0, -1.0), 0.5, material_left})
	world.add(Sphere{NewPoint(-1.0, 0.0, -1.0), -0.45, material_left})
	world.add(Sphere{NewPoint(1.0, 0.0, -1.0), 0.5, material_right})
	return world
}

func RandomSense() HittableList {
	var world HittableList
	groundMaterial := Lambertian{NewColor(0.5, 0.5, 0.5)}
	world.add(Sphere{NewPoint(0, -1000, 0), 1000, groundMaterial})

	for a := -11; a < 11; a++ {
		for b := -11; b < 11; b++ {
			chooseMat := randomFloat(0, 1)
			center := NewPoint(float64(a)+0.9*randomFloat(0, 1), 0.2, float64(b)+0.9*randomFloat(0, 1))
			if center.sub(NewPoint(4, 0.2, 0)).length() > 0.9 {
				if chooseMat < 0.8 {
					albedo := randomColor(0, 1).mul(randomColor(0, 1))
					sphereMaterial := Lambertian{albedo}
					world.add(Sphere{center, 0.2, sphereMaterial})
				} else if chooseMat < 0.95 {
					albedo := randomColor(0.5, 1)
					fuzz := randomFloat(0, 0.5)
					sphereMaterial := Metal{albedo, fuzz}
					world.add(Sphere{center, 0.2, sphereMaterial})
				} else {
					sphereMaterial := Dielectric{1.5}
					world.add(Sphere{center, 0.2, sphereMaterial})
				}
			}
		}
	}

	material1 := Dielectric{1.5}
	world.add(Sphere{NewPoint(0, 1, 0), 1.0, material1})

	material2 := Lambertian{NewColor(0.4, 0.2, 0.1)}
	world.add(Sphere{NewPoint(-4, 1, 0), 1.0, material2})

	material3 := Metal{NewColor(0.7, 0.6, 0.5), 0.0}
	world.add(Sphere{NewPoint(4, 1, 0), 1.0, material3})

	return world
}
