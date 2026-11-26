<?php

namespace Geo;

interface Shape
{
    public function area(): float;
}

class Circle implements Shape
{
    public function __construct(private float $r) {}

    public function area(): float
    {
        return pi() * $this->r ** 2;
    }
}

function total(array $shapes): float
{
    return array_sum(array_map(fn ($s) => $s->area(), $shapes));
}
