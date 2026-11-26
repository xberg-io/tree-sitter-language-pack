public with sharing class Greeter extends BaseGreeter implements Speaker, Comparable {
    public static final Integer MAX_GREETINGS = 10;
    private Integer count = 0;
    public String name { get; set; }

    public enum Mood { HAPPY, SAD }

    public interface Formatter {
        String format(String text);
    }

    public class Envelope {
        public String body;

        public Envelope(String body) {
            this.body = body;
        }
    }

    public Greeter() {
        this('Apex');
    }

    public Greeter(String name) {
        this.name = name;
    }

    @AuraEnabled
    public String greet(String other) {
        count++;
        Envelope envelope = new Envelope(format(other));
        return envelope.body;
    }

    public String greet() {
        return greet('World');
    }

    public Integer compareTo(Object other) {
        return String.valueOf(other).length() - name.length();
    }

    private static String format(String text) {
        String trimmed = text.trim();
        return 'Hello ' + trimmed;
    }
}
