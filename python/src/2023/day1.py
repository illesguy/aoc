from inputs import get_input_for_day


inp = get_input_for_day(2023, 1, str)


def first_and_last_digit(line):
    digits = [c for c in line if c.isdigit()]
    return int(digits[0]) * 10 + int(digits[-1])


def get_digits_from_text(line):
    return (line
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
    )


numbers = [first_and_last_digit(l) for l in inp]
print(sum(numbers))

numbers2 = [first_and_last_digit(get_digits_from_text(l)) for l in inp]
print(sum(numbers2))
