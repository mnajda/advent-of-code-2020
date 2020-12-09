import sys
from itertools import combinations


def load(path):
    with open(path, 'r') as file:
        return [int(line) for line in file.read().split("\n")]


def is_valid(range, val):
    return any((sum(combination) == val for combination in (combinations(range, 2))))


def solve(input):
    return next((val for pos, val in enumerate(input[25:]) if not is_valid(input[pos:(pos + 25)], val)), None)


if __name__ == "__main__":
    path = sys.argv[1]
    result = solve(load(path))

    print(result)
