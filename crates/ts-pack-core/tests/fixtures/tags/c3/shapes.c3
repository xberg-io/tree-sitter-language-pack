module shapes;

import std::io;

const int MAX_SIDES = 8;

int counter = 0;

extern fn int puts(char* str);

struct Point
{
    int x;
    int y;
}

union Number
{
    int i;
    float f;
}

bitstruct Flags : char
{
    bool visible : 0;
    bool dirty : 1;
}

enum Color
{
    RED,
    GREEN,
}

constdef Mode
{
    FAST = 1,
    SLOW = 2,
}

faultdef NOT_FOUND, IO_ERROR;

interface Drawable
{
    fn void draw();
}

alias Score = int;

typedef Id = int;

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

fn void main()
{
    Point p = { 1, 2 };
    area(p.sum(), 3);
}
