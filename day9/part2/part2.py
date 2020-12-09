import sys
from itertools import combinations


def load(path):
    with open(path, 'r') as file:
        return [int(line) for line in file.read().split("\n")]


def is_valid(pos, val, input):
    all_combinations = (combinations(input[pos:(pos + 25)], 2))
    return any((sum(combination) == val for combination in all_combinations))


def find_invalid_number(input):
    for pos, val in enumerate(input[25:]):
        if not is_valid(pos, val, input):
            return val


def solve(input):
    target = find_invalid_number(input)
    sum, begin, end = 0, 0, 0

    while begin < len(input):
        if sum == target and begin + 1 < end:
            return input[begin:end]
        elif sum < target and begin < len(input):
            sum += input[end]
            end += 1
        else:
            sum -= input[begin]
            begin += 1


if __name__ == "__main__":
    path = sys.argv[1]
    input = load(path)
    result = solve(input)
    min, max = min(result), max(result)

    print(min + max)
