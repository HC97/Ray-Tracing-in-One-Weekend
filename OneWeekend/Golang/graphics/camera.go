package graphics

import (
	"math"
)

type Camera struct {
	position   Point
	u          Vector
	v          Vector
	w          Vector
	lower_left Vector
	vertical   Vector
	horizontal Vector
	radius     float64
}

func NewCamera(pos, aim Point, vup Vector, fov, rat, aper, dist float64) Camera {
	w := aim.sub(pos).unit()
	u := w.cross(vup).unit()
	v := u.cross(w).unit()
	height := 2 * dist * math.Tan(fov/180*math.Pi/2)
	width := height * rat
	vertical := v.mutiply(height)
	horizontal := u.mutiply(width)
	lower_left := pos.add(w.mutiply(dist)).sub(vertical.mutiply(0.5)).sub(horizontal.mutiply(0.5))
	lower_left = lower_left.sub(pos)
	return Camera{
		position:   pos,
		u:          u,
		v:          v,
		w:          w,
		lower_left: lower_left,
		vertical:   vertical,
		horizontal: horizontal,
		radius:     aper / 2,
	}
}

func (cam Camera) getRay(u, v float64) Ray {
	rd := randomInUnitDisk().mutiply(cam.radius)
	offset := cam.u.mutiply(rd.x()).add(cam.v.mutiply(rd.y()))
	direction := cam.lower_left.add(cam.horizontal.mutiply(u)).add(cam.vertical.mutiply(v))
	ray := NewRay(cam.position.add(offset), direction.sub(offset))
	return ray
}

type Ray struct {
	origin    Point
	direction Vector
}

func NewRay(origin Point, direction Vector) Ray {
	return Ray{origin, direction.unit()}
}

func (r Ray) at(time float64) Point {
	return r.origin.add(r.direction.mutiply(time))
}
