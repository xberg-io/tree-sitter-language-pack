module shapes;

import std::io;

const int MAX_SIDES = 8;

int counter = 0;

struct Point
{
    int x;
    int y;
}

enum Color
{
    RED,
    GREEN,
}

interface Drawable
{
    fn void draw();
}

alias Score = int;

fn int Point.sum(Point* self)
{
    return self.x + self.y;
}

macro square(x)
{
    return x * x;
}

fn int area(int w, int h)
{
    int result = w * h;
    io::printn(result);
    return result;
}
