from utils import get_input_for_day


inputs = get_input_for_day(2020, 9, int)
ex_input = [35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576]


def number_is_sum_of_others(sum_to_get, others):
    to_check = set(others)
    for num in others:
        if sum_to_get - num in to_check and sum_to_get - num != num:
            return True
    return False


def find_first_non_sum(numbers, preamble=25):
    for i in range(preamble, len(numbers)):
        if not number_is_sum_of_others(numbers[i], numbers[i - preamble:i]):
            return i
    raise ValueError('No such number')


def find_contiguous_set(sum_to_get, numbers):
    for i in range(len(numbers) - 1):
        for j in range(i + 2, len(numbers) + 1):
            set_of_nums = numbers[i:j]
            if sum(set_of_nums) == sum_to_get:
                return min(set_of_nums) + max(set_of_nums)
    raise ValueError('No such set')


def example():
    first_non_sum_index = find_first_non_sum(ex_input, 5)
    print(ex_input[first_non_sum_index])
    weakness = find_contiguous_set(ex_input[first_non_sum_index], ex_input[:first_non_sum_index])
    print(weakness)


def main():
    first_non_sum_index = find_first_non_sum(inputs)
    print(inputs[first_non_sum_index])
    weakness = find_contiguous_set(inputs[first_non_sum_index], inputs[:first_non_sum_index])
    print(weakness)


# example()
main()

