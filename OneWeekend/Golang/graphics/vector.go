package graphics

import "math"

type Vector struct {
	vec [3]float64
}

type Color = Vector
type Point = Vector

func NewVector(x, y, z float64) Vector {
	return Vector{[3]float64{x, y, z}}
}

var (
	NewColor = NewVector
	NewPoint = NewVector
)

func (vec Vector) x() float64 {
	return vec.vec[0]
}

func (vec Vector) y() float64 {
	return vec.vec[1]
}

func (vec Vector) z() float64 {
	return vec.vec[2]
}

func (vec Vector) length() float64 {
	return math.Sqrt(vec.dot(vec))
}

func (vec Vector) reverse() Vector {
	x, y, z := -vec.x(), -vec.y(), -vec.z()
	return Vector{[3]float64{x, y, z}}
}

func (vec Vector) add(other Vector) Vector {
	x := vec.x() + other.x()
	y := vec.y() + other.y()
	z := vec.z() + other.z()
	return Vector{[3]float64{x, y, z}}
}

func (vec Vector) sub(other Vector) Vector {
	x := vec.x() - other.x()
	y := vec.y() - other.y()
	z := vec.z() - other.z()
	return Vector{[3]float64{x, y, z}}
}

func (vec Vector) unit() Vector {
	len := vec.length()
	x := vec.x() / len
	y := vec.y() / len
	z := vec.z() / len
	return Vector{[3]float64{x, y, z}}
}

func (vec Vector) mutiply(t float64) Vector {
	x := vec.x() * t
	y := vec.y() * t
	z := vec.z() * t
	return Vector{[3]float64{x, y, z}}
}

func (vec Vector) mul(other Vector) Vector {
	x := vec.x() * other.x()
	y := vec.y() * other.y()
	z := vec.z() * other.z()
	return Vector{[3]float64{x, y, z}}
}

func (vec Vector) dot(other Vector) float64 {
	v := vec.mul(other)
	return v.x() + v.y() + v.z()
}

func (vec Vector) cross(other Vector) Vector {
	x := vec.y()*other.z() - vec.z()*other.y()
	y := vec.z()*other.x() - vec.x()*other.z()
	z := vec.x()*other.y() - vec.y()*other.x()
	return Vector{[3]float64{x, y, z}}
}
