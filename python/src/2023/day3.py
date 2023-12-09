from inputs import get_input_for_day
from collections import defaultdict


def parse_input(line):
    symbol_indices = defaultdict(set)
    numbers = []

    current_number = []
    number_start = 0
    for i, c in enumerate(line):
        if c.isdigit():
            if not current_number:
                number_start = i
            current_number.append(c)
        else:    
            if c != '.':
                symbol_indices[c].add(i)
            if current_number:
                numbers.append({"num": int(''.join(current_number)), "start": number_start, "end": i})
                current_number.clear()
    if current_number:
        numbers.append({"num": int(''.join(current_number)), "start": number_start, "end": len(line)})

    return {"symbols": symbol_indices, "numbers": numbers}


inp = get_input_for_day(2023, 3, parse_input)


def get_all_symbol_indices(line):
    return {c for s in line["symbols"].values() for c in s}


def get_total_of_part_numbers(parsed_lines):
    total_code = 0
    total_non_code = 0
    total = 0
    total_lines = len(parsed_lines)
    for i, line in enumerate(parsed_lines):
        for n in line["numbers"]:
            indexes_to_check = set(range(n["start"] - 1, n["end"] + 1))
            if (
                get_all_symbol_indices(line).intersection(indexes_to_check) or
                (i and get_all_symbol_indices(parsed_lines[i - 1]).intersection(indexes_to_check)) or
                (i < total_lines - 1 and get_all_symbol_indices(parsed_lines[i + 1]).intersection(indexes_to_check))
            ):
                total_code += n["num"]
            else:
                total_non_code += n["num"]
            
            total += n["num"]

    return total_code


def get_total_gear_ratios(parsed_lines):
    total = 0
    for i, line in enumerate(parsed_lines):
        for gear_index in line["symbols"]["*"]:
            numbers_for_gear = []
            numbers_for_gear.extend([n["num"] for n in line["numbers"] if gear_index in range(n["start"] - 1, n["end"] + 1)])
            if i:
                numbers_for_gear.extend([n["num"] for n in parsed_lines[i - 1]["numbers"] if gear_index in range(n["start"] - 1, n["end"] + 1)])
            if i < len(parsed_lines) - 1:
                numbers_for_gear.extend([n["num"] for n in parsed_lines[i + 1]["numbers"] if gear_index in range(n["start"] - 1, n["end"] + 1)])
            
            if len(numbers_for_gear) == 2:
                total += numbers_for_gear[0] * numbers_for_gear[1]

    return total


print(get_total_of_part_numbers(inp))
print(get_total_gear_ratios(inp))
