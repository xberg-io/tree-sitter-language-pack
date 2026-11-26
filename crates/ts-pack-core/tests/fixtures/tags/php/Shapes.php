<?php

namespace Geo\Shapes;

const DEFAULT_RADIUS = 1.0;
define('MAX_SHAPES', 100);

interface Shape
{
    public function area(): float;
}

interface Solid extends Shape
{
    public function volume(): float;
}

trait Named
{
    public string $label = "";

    public function label(): string
    {
        return strtoupper($this->label);
    }
}

enum Unit: string
{
    case Metric = 'm';
    case Imperial = 'in';

    const Fallback = self::Metric;
}

abstract class Base
{
    protected static int $count = 0;
}

final class Circle extends Base implements Shape
{
    use Named;

    public const TAU = 6.28;

    public function __construct(private float $radius, public readonly Unit $unit = Unit::Metric)
    {
        $local = $radius * 2;
    }

    public function area(): float
    {
        return pi() * $this->radius ** 2;
    }

    public static function unit(): static
    {
        return new static(DEFAULT_RADIUS);
    }
}

function total(array $shapes): float
{
    $first = new Circle(1.0);
    $first?->label();
    return array_sum(array_map(fn ($s) => $s->area(), $shapes)) + Circle::unit()->area();
}
