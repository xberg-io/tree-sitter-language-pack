public class Greeter implements Speaker {
    public enum Mood { HAPPY, SAD }

    public String greet(String name) {
        return format(name);
    }
}
