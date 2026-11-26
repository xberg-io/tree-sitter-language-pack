from libc.math cimport sqrt

MAX_SIZE = 10

cdef int counter = 0

cdef extern from "stdlib.h":
    int abs(int x)

ctypedef double real_t

cdef struct Vec2:
    double x
    double y

cdef enum Color:
    RED
    GREEN

cdef int square(int x):
    cdef int result = x * x
    return result

cpdef double norm(Vec2 v):
    return sqrt(v.x * v.x + v.y * v.y)

def greet(name):
    return format(name)

cdef class Point:
    cdef public double x

    def __init__(self, double x):
        self.x = x

    cpdef double scaled(self, double k):
        return self.x * k

class Plain:
    def area(self):
        return square(2)
