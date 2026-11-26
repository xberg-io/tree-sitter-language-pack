package demo

class Greeter {
    String name

    String greet(String other) {
        return "Hello ${other}, I am ${name}"
    }
}

def shout(String text) {
    println text.toUpperCase()
}

def greeter = new Greeter(name: 'Groovy')
shout(greeter.greet('World'))
