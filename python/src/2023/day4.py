import re
from collections import defaultdict
from inputs import get_input_for_day


def parse_input(line):
    card_info, _, numbers = re.split(r":(\s+)", line)
    card_num = int(re.search(r"\d+", card_info).group())
    my_numbers, winning_numbers = re.split(r"\s+\|\s+", numbers)
    return {
        "card_num": card_num,
        "my_numbers": {int(n) for n in my_numbers.split()},
        "winning_numbers": {int(n) for n in winning_numbers.split()}
    }


def part1(inp):
    total = 0
    for line in inp:
        overlap = line["my_numbers"].intersection(line["winning_numbers"])
        if overlap:
            total += 2 ** (len(overlap) - 1)
    return total

def part2(inp):
    copies_of_cards = defaultdict(lambda: 1)
    total_cards = 0
    for line in inp:
        current_cards = copies_of_cards[line["card_num"]]
        total_cards += current_cards
        overlap = line["my_numbers"].intersection(line["winning_numbers"])
        for i in range(len(overlap)):
            copies_of_cards[line["card_num"] + i + 1] += current_cards
    return total_cards



inp = get_input_for_day(2023, 4, parse_input)
print("Input lines read:", len(inp))
print("part 1:", part1(inp))
print("part 2:", part2(inp))
