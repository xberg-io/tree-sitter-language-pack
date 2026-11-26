namespace Demo {
    public const int MAX = 8;

    public enum Color {
        RED,
        GREEN
    }

    public struct Point {
        public int x;
        public int y;
    }

    public interface Drawable : Object {
        public abstract void draw ();
    }

    public delegate int Transform (int value);

    public class Greeter : Object, Drawable {
        public string name { get; set; }
        private int count = 0;

        public signal void greeted (string who);

        public Greeter (string name) {
            this.name = name;
        }

        public void draw () {
            int local_value = count + 1;
            print ("%d\n", local_value);
            greeted (name);
        }
    }

    public static int main (string[] args) {
        var greeter = new Greeter ("world");
        greeter.draw ();
        return 0;
    }
}
