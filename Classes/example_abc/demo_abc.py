from abc import ABC, abstractmethod

class DataGroup(ABC):



class DateOfBirth:

    def __init__(self):
        self.CONFIG = {"name": "date_of_birth"}

    def extract(self) -> None:
        print("Extracting date of birth data")

    def transform(self) -> None:
        print("Transforming date of birth data")

    def format_dob(self) -> None:
        print("Formatting date of birth")


class NhsNumber:

    def __init__(self):
        self.CONFIG = {"name": "nhs_number"}

    def extract(self) -> None:
        print("Extracting nhs number data")

    def transform(self) -> None:
        print("Transforming nhs number data")

    def validate_nhs_number(self) -> None:
        print("Validating nhs number")


class Etl:

    def run_etl(self, data_group) -> None:
        print('-----------------')
        print(f'{data_group.CONFIG["name"]}')
        data_group.extract()
        data_group.transform()

def main() -> None:
    etl_runner = Etl()
    date_of_birth = DateOfBirth()
    nhs_number = NhsNumber()
    data_groups = [date_of_birth, nhs_number]
    for dg in data_groups:
        etl_runner.run_etl(dg)


if __name__ == "__main__":
    main()