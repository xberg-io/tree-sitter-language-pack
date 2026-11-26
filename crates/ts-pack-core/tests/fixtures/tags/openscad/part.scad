include <lib.scad>
use <shapes.scad>

width = 20;

module rounded_box(size, r = 2) {
    minkowski() {
        cube(size - [2 * r, 2 * r, 0]);
        cylinder(r = r, h = 0.01);
    }
}

function half(x) = x / 2;

rounded_box([width, half(width), 5]);
