from math import sqrt


struct Point:
    fn norm(self) -> Float64:
        return sqrt(self.dot())

    fn dot(self) -> Float64:
        return 0.0

    @staticmethod
    fn origin_distance() -> Int:
        return 0


fn add(a: Int, b: Int) -> Int:
    total = a + b
    return total


def main():
    print(add(1, 2))
