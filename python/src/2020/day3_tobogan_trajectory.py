from utils import get_input_for_day


inp = get_input_for_day(2020, 3)

height = len(inp)
width = len(inp[0])

multiplier = int((height * 7) / width) + 1

square = [l * multiplier for l in inp]

def get_tree_count_on_slope(right_step, down_step=1):
    trees = 0
    col = right_step
    for row in range(down_step, len(square), down_step):
        if square[row][col] == '#':
            trees += 1
        col += right_step
    return trees

slope1 = get_tree_count_on_slope(1)
slope2 = get_tree_count_on_slope(3)
slope3 = get_tree_count_on_slope(5)
slope4 = get_tree_count_on_slope(7)
slope5 = get_tree_count_on_slope(1, 2)


print(slope2)
print(slope1 * slope2 * slope3 * slope4 * slope5)
