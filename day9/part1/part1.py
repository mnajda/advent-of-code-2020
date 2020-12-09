import sys
from itertools import combinations


def load(path):
    with open(path, 'r') as file:
        return [int(line) for line in file.read().split("\n")]


def is_valid(pos, val, input):
    all_combinations = (combinations(input[pos:(pos + 25)], 2))
    return any((sum(combination) == val for combination in all_combinations))


def solve(input):
    for pos, val in enumerate(input[25:]):
        if not is_valid(pos, val, input):
            return val


if __name__ == "__main__":
    path = sys.argv[1]
    input = load(path)
    result = solve(input)

    print(result)
