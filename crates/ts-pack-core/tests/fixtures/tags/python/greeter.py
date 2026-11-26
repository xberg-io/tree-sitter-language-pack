import os.path
from dataclasses import dataclass
from typing import TypeAlias

MAX_SIZE = 10
default_greeting: str = "Hello"
first, second = "a", "b"
Names: TypeAlias = list[str]
type Pair[T] = tuple[T, T]


class Base:
    pass


@dataclass
class Greeter(Base):
    count: int = 0

    class Options:
        loud = False

    def __init__(self, name):
        self.name = name

    @property
    def greeting(self):
        return format_name(self.name)

    async def fetch(self, path):
        return os.path.join(path, self.name)


def format_name(name):
    local_value = name.strip()
    return local_value.title()


async def main():
    for item in [Greeter("a")]:
        print(item.greeting)
