import sys

FIELDS = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
EYE_COLORS = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]


def load(path):
    with open(path, 'r') as file:
        return file.read().split("\n\n")


def make_dict(fields):
    passport = {}
    for field in fields:
        key, value = field.split(":")
        passport[key] = value
    return passport


def validate_height(height):
    valid_cm = height[-2:] == "cm" and 150 <= int(height[:-2]) <= 193
    valid_in = height[-2:] == "in" and 59 <= int(height[:-2]) <= 76
    return valid_cm or valid_in


def is_valid(passport):
    return all(field in passport for field in FIELDS) and all(
    [
        1920 <= int(passport["byr"]) <= 2002,
        2010 <= int(passport["iyr"]) <= 2020,
        2020 <= int(passport["eyr"]) <= 2030,
        validate_height(passport["hgt"]),
        passport["hcl"][0] == "#" and len(passport["hcl"][1:]) == 6 and (
            char in "0123456789abcdef" for char in passport["hcl"][1:]),
        passport["ecl"] in EYE_COLORS,
        len(passport["pid"]) == 9 and passport["pid"].isdigit()
    ])


def count_valid(input):
    passports = (make_dict(line.replace('\n', ' ').split(' ')) for line in input)
    return len(list(filter(lambda passport: is_valid(passport), passports)))


if __name__ == "__main__":
    path = sys.argv[1]
    result = count_valid(load(path))
    print(result)
