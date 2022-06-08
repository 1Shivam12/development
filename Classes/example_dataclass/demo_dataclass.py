import random
import string
from dataclasses import dataclass

def generate_skeleton_id() -> str:
    return "".join(random.choices(string.ascii_uppercase, k=5))

class Person:
    def __init__(self, name: str, date_of_birth: str, nhs_number: str):
        self.name = name
        self.date_of_birth = date_of_birth
        self.nhs_number = nhs_number

    # def __str__(self) -> str:
    #    return f"{self.name}, {self.date_of_birth}, {self.nhs_number}"

    def __repr__(self) -> str:
        return f"{self.name}, {self.date_of_birth}, {self.nhs_number}"


def main() -> None:
    person = Person(name="Joe Blogs", date_of_birth="25081976", nhs_number="1234567890")
    print(person)


if __name__ == "__main__":
    main()



