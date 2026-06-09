from typing import Any


class Ref[T: Any]:
    value: T | None = None

    def __init__(self, value: T | None = None) -> None:
        self.value = value

    def reset(self) -> None:
        self.value = None
