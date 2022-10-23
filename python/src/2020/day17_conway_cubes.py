from utils import get_input_for_day


inp = get_input_for_day(2020, 17)


def apply_cycles(cubes, dims=2, cycles=6):
    if not cycles:
        return sum(cubes)
    return 0


print(apply_cycles(inp, dims=3))
print(apply_cycles(inp, dims=4))
