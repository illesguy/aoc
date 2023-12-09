from functools import reduce
from math import gcd
from inputs import get_input_for_day


def parse_input(line):
    key, values = line.split(" = ")
    left, right = values.replace("(", "").replace(")", "").split(", ")
    return (key, {'L': left, 'R': right})


def part1(instructions, mp):
    steps = 0
    current = "AAA"
    while current != "ZZZ":
        ins = instructions[steps % len(instructions)]
        steps += 1
        current = mp[current][ins]
    return steps


def part2(instructions, mp):
    starts = [k for k in mp.keys() if k.endswith("A")]
    steps_to_finish = []
    for s in starts:
        steps = 0
        current = s
        while not current.endswith("Z"):
            ins = instructions[steps % len(instructions)]
            steps += 1
            current = mp[current][ins]
        steps_to_finish.append(steps)

    return reduce(lambda a, b: a * b // gcd(a, b), steps_to_finish)


inp = get_input_for_day(2023, 8, str)

instructions = inp[0]
mp = dict([parse_input(i) for i in inp[2:]])

print("Input lines read:", len(inp))
print("part 1:", part1(instructions, mp))
print("part 2:", part2(instructions, mp))
