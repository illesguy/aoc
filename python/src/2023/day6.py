import math
from inputs import get_input_for_day


def parse_input(line):
    parts = line.split()
    return [int(p) for p in parts[1:]]


def count_wins(time, distance):
    half_time = float(time) / 2
    x = math.floor(half_time)
    y = math.ceil(half_time)
    if x * y < distance:
        return 0
    if x == y:
        wins = 1
        x += 1
        y -= 1
    else:
        wins = 2
        x += 2
        y -= 2
    while x * y >= distance:
        wins += 2
        x += 1
        y -= 1
    return wins


def part1(inp):
    total = 1
    for time, distance in inp:
        total *= count_wins(time, distance)
    return total


def part2(inp):
    pass


inp = get_input_for_day(2023, 6, parse_input)
inp1 = list(zip(*inp))
time = int(''.join([str(i) for i in inp[0]]))
distance = int(''.join([str(i) for i in inp[1]]))

print("part 1:", part1(inp1))
print("part 2:", count_wins(time, distance))
