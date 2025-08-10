package graphics

import (
	"math"
)

type Hittable interface {
	hit(Ray, float64, float64) (HitRecord, bool)
}

type HitRecord struct {
	time     float64
	point    Point
	normal   Vector
	front    bool
	material Material
}

type HittableList struct {
	objects []Hittable
}

func (list *HittableList) add(object Hittable) {
	list.objects = append(list.objects, object)
}

func (list HittableList) hit(ray Ray, t_min, t_max float64) (HitRecord, bool) {
	time := t_max
	hit := false
	var record HitRecord
	for _, obj := range list.objects {
		rec, ok := obj.hit(ray, t_min, t_max)
		if ok && rec.time < time {
			record = rec
			time = rec.time
			hit = true
		}
	}
	return record, hit
}

type Sphere struct {
	center   Point
	radius   float64
	material Material
}

func (sphere Sphere) hit(ray Ray, t_min, t_max float64) (HitRecord, bool) {
	var rec HitRecord
	oc := ray.origin.sub(sphere.center)
	a := ray.direction.dot(ray.direction)
	b := oc.dot(ray.direction) * 2
	c := oc.dot(oc) - sphere.radius*sphere.radius
	temp := b*b - 4*a*c
	if temp <= 0 {
		return rec, false
	}
	temp = math.Sqrt(temp)
	t := (-b - temp) / (2 * a)
	if t < t_min || t > t_max {
		t = (-b + temp) / (2 * a)
		if t < t_min || t > t_max {
			return rec, false
		}
	}
	rec.time = t
	rec.point = ray.at(t)
	rec.front = true
	rec.normal = rec.point.sub(sphere.center).mutiply(1 / sphere.radius)
	if ray.direction.dot(rec.normal) > 0 {
		rec.normal = rec.normal.reverse()
		rec.front = false
	}
	rec.material = sphere.material
	return rec, true
}
