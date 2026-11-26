// Package shapes models simple geometry.
package shapes

import (
	"fmt"
	str "strings"
)

// Max is the largest supported side count.
const Max = 8

const (
	A = iota
	B
)

var Counter int

var (
	defaultName = "shape"
	scale, offset float64
)

type ID = string

type Celsius float64

type Point struct {
	X, Y int
	Label string
}

type Shape interface {
	fmt.Stringer
	Area() float64
}

type Pair[T any] struct {
	First, Second T
}

func (p Point) Norm() int {
	return p.X*p.X + p.Y*p.Y
}

func (p *Point) Move(dx int) {
	p.X += dx
}

func NewPoint(x, y int) Point {
	const local = 1
	var tmp int
	type inner struct{ v int }
	total := x + local + tmp
	return Point{X: total, Y: y}
}

func main() {
	p := NewPoint(1, 2)
	fmt.Println(p.Norm(), str.ToUpper(defaultName))
}
