import random
import string
from dataclasses import dataclass, field

def generate_skeleton_id() -> str:
    return "".join(random.choices(string.ascii_uppercase, k=5))

@dataclass()
class Person:
    name: str
    date_of_birth: str
    nhs_number: str
    stale: bool = False
    studies: list[str] = field(default_factory=list)
    skeleton_id: str = field(default_factory=generate_skeleton_id, init=False, compare=False)
    _search_string: str = field(init=False, repr=False, compare=False)

    def __post_init__(self) -> None:
        self._search_string = f'{self.name} {self.nhs_number} {self.skeleton_id}'

def main() -> None:
    person = Person(name="Joe Blogs", date_of_birth="25081976", nhs_number="1234567890")
    print(person)
    person_b = Person(name="John Blogs", date_of_birth="25081976", nhs_number="1234567890")
    person_c = Person(name="John Blogs", date_of_birth="25081976", nhs_number="1234567890")
    print(person_b == person_c)


if __name__ == "__main__":
    main()
