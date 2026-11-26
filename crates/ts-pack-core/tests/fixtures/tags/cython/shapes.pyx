MAX_SIZE = 10

cdef int square(int x):
    return x * x

def greet(name):
    return format(name)

cdef class Point:
    cdef public double x
