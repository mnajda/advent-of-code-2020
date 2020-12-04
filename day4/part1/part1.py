import sys


FIELDS = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]


def load(path):
    with open(path, 'r') as file:
        return file.read().split("\n\n")


def make_dict(fields):
    passport = {}
    for field in fields:
        key, value = field.split(":")
        passport[key] = value
    return passport


def count_valid(input):
    valid = 0

    for line in input:
        passport = make_dict(line.replace('\n', ' ').split(' '))
        if all (field in passport for field in FIELDS):
            valid += 1

    return valid


if __name__ == "__main__":
    path = sys.argv[1]
    result = count_valid(load(path))
    print(result)
