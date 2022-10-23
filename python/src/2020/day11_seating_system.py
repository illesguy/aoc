from utils import get_input_for_day


def map_seats(seat):
    if seat == 'L':
        return 1
    return 0


def map_seat_rows(seat_row):
    return [0] + [map_seats(s) for s in seat_row] + [0]


inp = get_input_for_day(2020, 11, map_seat_rows)
row_length = len(inp[0])
initial_seats = [[0] * row_length] + inp + [[0] * row_length]


def get_new_seat_state(seat, *adjacent_seats):
    if seat == 2 and not sum([s % 2 for s in adjacent_seats]):
        return 1
    if seat == 1 and sum([s % 2 for s in adjacent_seats]) >= 5:
        return 2
    return seat


def find_first_seat_seen(seats, crow, ccol, drow, dcol):
    if not crow or not ccol or crow == len(seats) - 1 or ccol == len(seats[0]) - 1:
        return seats[crow][ccol]
    if seats[crow][ccol] == 2 or seats[crow][ccol] == 1:
        return seats[crow][ccol]
    return find_first_seat_seen(seats, crow + drow, ccol + dcol, drow, dcol)


def move_seats(seats):
    new_seats = [r.copy() for r in seats]
    for row in range(1, len(seats) - 1):
        for col in range(1, len(seats[row]) - 1):
            new_seats[row][col] = get_new_seat_state(
                seats[row][col],
                find_first_seat_seen(seats, row - 1, col - 1, -1, -1),
                find_first_seat_seen(seats, row - 1, col, -1, 0),
                find_first_seat_seen(seats, row - 1, col + 1, -1, 1),
                find_first_seat_seen(seats, row, col - 1, 0, -1),
                find_first_seat_seen(seats, row, col + 1, 0, 1),
                find_first_seat_seen(seats, row + 1, col - 1, 1, -1),
                find_first_seat_seen(seats, row + 1, col, 1, 0),
                find_first_seat_seen(seats, row + 1, col + 1, 1, 1)
            )
    return new_seats


def get_seats_end_state(current_seats, previous_seats=None):
    if current_seats == previous_seats:
        return current_seats
    return get_seats_end_state(move_seats(current_seats), current_seats)


end_seats = get_seats_end_state(initial_seats)
occupied_count = sum([s % 2 for r in end_seats for s in r])
print(occupied_count)
