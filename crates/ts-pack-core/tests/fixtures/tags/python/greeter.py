MAX_SIZE = 10


class Greeter:
    def __init__(self, name):
        self.name = name

    def greet(self):
        return format_name(self.name)


def format_name(name):
    return name.title()
