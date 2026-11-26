namespace Geometry
{
    public interface IShape
    {
        double Area();
    }

    public class Circle : IShape
    {
        public double Radius { get; set; }

        public double Area()
        {
            return Math.Round(3.14 * Radius * Radius);
        }
    }
}
