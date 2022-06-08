import random
import string
from dataclasses import dataclass, field


def generate_skeleton_id() -> str:
    return "".join(random.choices(string.ascii_uppercase, k=5))


@dataclass(frozen=False)
class Person:
    name: str
    date_of_birth: str
    nhs_number: str

    # Default value example for primitive types
    stale: bool = False

    # Default value example for complex types
    studies: list[str] = field(default_factory=list)
    skeleton_id: str = field(init=False, default_factory=generate_skeleton_id)

    # Post Init -> Demo how to generate a val from another value
    _search_string: str = field(init=False, repr=False)

    def __post_init__(self) -> None:
        self._search_string = f"{self.name} {self.nhs_number} {self.skeleton_id}"


def main() -> None:
    person_a = Person(name="Joe Blogs", date_of_birth="25081976", nhs_number="1234567890")
    person_b = Person(name="Bat Man", date_of_birth="19062002", nhs_number="2234567890", stale=True)
    print(person_a,'\n', person_b)
    person_a.name = "Shiv"




if __name__ == "__main__":
    main()

