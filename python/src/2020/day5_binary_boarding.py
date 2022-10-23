from utils import get_input_for_day


def seat_code_to_number(seat_code):
    binary = seat_code.replace('B', '1').replace('R', '1').replace('F', '0').replace('L', '0')
    return int(binary, 2)


def find_my_seat(seat_numbers, max_seat):
    seats = set(seat_numbers)
    for s in range(1, max_seat):
        if s not in seats and s + 1 in seats and s - 1 in seats:
            return s
    return -1


seat_numbers = get_input_for_day(2020, 5, seat_code_to_number)

max_seat = max(seat_numbers)
print(max_seat)
print(find_my_seat(seat_numbers, max_seat))

