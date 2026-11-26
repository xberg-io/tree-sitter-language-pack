package demo

import groovy.transform.CompileStatic

interface Speaker {
    String speak(String text)
}

trait Polite {
    String please(String text) {
        return "${text}, please"
    }
}

enum Mood {
    HAPPY, SAD
}

@CompileStatic
class Greeter implements Speaker, Polite {
    static final String DEFAULT = 'Hello'
    String name
    int count = 0

    Greeter(String name) {
        this.name = name
    }

    String speak(String text) {
        count++
        return please("${DEFAULT} ${text}, I am ${name}")
    }

    def reset() {
        count = 0
    }
}

def shout(String text) {
    println text.toUpperCase()
}

String whisper(String text) {
    return text.toLowerCase()
}

def greeter = new Greeter('Groovy')
shout(greeter.speak('World'))
println whisper('Bye')
