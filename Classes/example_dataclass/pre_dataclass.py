
class Person:
    def __init__(self, name: str, date_of_birth: str, nhs_number: str):
        self.name = name
        self.date_of_birth = date_of_birth
        self.nhs_number = nhs_number


    def __str__(self) -> str:
       return f"{self.name}, {self.date_of_birth}, {self.nhs_number}"

    def __repr__(self) -> str:
        return f"{self.name}, {self.date_of_birth}, {self.nhs_number}"

    def __eq__(self, other):
        if self.__class__ is other.__class__:
            return (self.name, self.date_of_birth, self.nhs_number) == (other.name, other.date_of_birth, other.nhs_number)
        else:
            return NotImplemented

    def __ne__(self, other):
        pass


    def __lt__(self, other):
        pass


def main() -> None:
    person = Person(name="Joe Blogs", date_of_birth="25081976", nhs_number="1234567890")
    print(person)
    person_b = Person(name="John Blogs", date_of_birth="25081976", nhs_number="1234567890")
    person_c = Person(name="John Blogs", date_of_birth="25081976", nhs_number="1234567890")
    print(person_b == person_c)


if __name__ == "__main__":
    main()
