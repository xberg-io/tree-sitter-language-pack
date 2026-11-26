package main

import "fmt"

type Point struct {
	X, Y int
}

type Shape interface {
	Area() float64
}

func (p Point) Norm() int {
	return p.X*p.X + p.Y*p.Y
}

func main() {
	fmt.Println(Point{1, 2}.Norm())
}
