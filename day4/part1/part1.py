import sys


FIELDS = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]


def load(path):
    with open(path, 'r') as file:
        return file.read().split("\n\n")


def make_dict(fields):
    return dict((field.split(":") for field in fields))


def is_valid(passport):
    return all(field in passport for field in FIELDS)


def count_valid(input):
    passports = (make_dict(line.replace('\n', ' ').split(' ')) for line in input)
    return sum(is_valid(passport) for passport in passports)


if __name__ == "__main__":
    path = sys.argv[1]
    result = count_valid(load(path))
    print(result)
