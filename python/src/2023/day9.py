from inputs import get_input_for_day


def parse_input(line):
    return [int(i) for i in line.split()]


def get_list_of_differences(numbers):
    return [numbers[i] - numbers[i - 1] for i in range(1, len(numbers))]


def find_next_number(numbers):
    differences = get_list_of_differences(numbers)
    if all([d == 0 for d in differences]):
        return numbers[-1]
    return numbers[-1] + find_next_number(differences)


def find_prev_number(numbers):
    differences = get_list_of_differences(numbers)
    if all([d == 0 for d in differences]):
        return numbers[0]
    return numbers[0] - find_prev_number(differences)


def part1(inp):
    return sum([find_next_number(s) for s in inp])


def part2(inp):
    return sum([find_prev_number(s) for s in inp])


inp = get_input_for_day(2023, 9, parse_input)
print("Input lines read:", len(inp))
print("part 1:", part1(inp))
print("part 2:", part2(inp))
