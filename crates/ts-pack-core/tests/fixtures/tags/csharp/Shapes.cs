using System;
using System.Collections.Generic;

namespace App.Core
{
    public delegate void Handler(object sender, EventArgs args);

    public enum Color
    {
        Red,
        Green = 2,
    }

    public interface IShape : IComparable<IShape>
    {
        double Area();
        string Name { get; }
        event Handler Changed;
    }

    public struct Point
    {
        public double X;
        public double Y;

        public Point(double x, double y)
        {
            X = x;
            Y = y;
        }
    }

    public record Size(double Width, double Height);

    [Serializable]
    public abstract class Shape<T> : IShape where T : struct
    {
        public const int MaxShapes = 64;
        private static readonly List<IShape> registry = new List<IShape>();
        protected int count;

        public event Handler Changed;

        public string Name { get; protected set; }

        protected Shape()
        {
            registry.Add(this);
        }

        public abstract double Area();

        public int CompareTo(IShape other) => Area().CompareTo(other.Area());

        public void Notify() => Changed?.Invoke(this, EventArgs.Empty);

        public void Notify(string reason)
        {
            Console.WriteLine(reason);
            Notify();
        }

        public class Builder
        {
            public Shape<T> Build() => null;
        }
    }

    public sealed class Circle : Shape<int>
    {
        public double Radius { get; set; }

        public event Handler Resized
        {
            add { Changed += value; }
            remove { Changed -= value; }
        }

        public Circle(double radius)
        {
            Radius = radius;
        }

        public override double Area()
        {
            var squared = Radius * Radius;
            return Math.Round(3.14 * squared);
        }
    }
}

namespace App.Tools;

public static class Program
{
    public static void Main(string[] args)
    {
        Circle circle = new Circle(2.0);
        Console.WriteLine(circle.Area());
    }
}
