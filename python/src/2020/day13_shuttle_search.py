from utils import get_input_for_day
import math


inp = get_input_for_day(2020, 13)

departure = int(inp[0])
buses = [(1, int(b)) for b in inp[1].split(',') if b != 'x']


def bus_after_dep(dep, bs):
    bs_after = [(i, b * i) for i, b in bs if b * i >= dep]
    if bs_after:
        ba = min(bs_after, key=lambda t: t[1])
        return int(ba[1] / ba[0]) * (ba[1] - dep)
    return 0

bad = bus_after_dep(departure, buses)
while not bad:
    buses = [(i + 1, b) for i, b in buses]
    bad = bus_after_dep(departure, buses)

print(bad)

def parse_input(inp):
    bs = list(enumerate([[int(b), int(b)] if b != 'x' else b for b in inp[1].split(',')]))
    return [[i, b] for i, b in bs if b != 'x']


def lcm(a, b):
    return abs(a * b) // math.gcd(a, b)

for i in range(10000):
    i ** i ** i


def reduce_buses_fast(bs):
    print(bs)
    if len(bs) == 1:
        return bs[0][1][1]
    b1 = bs[0]
    b2 = bs[1]
    b2[0] %= b2[1][0]
    while b1[1][1] % b2[1][1] != b2[1][1] - b2[0]:
        b1[1][1] += b1[1][0]
    return reduce_buses_fast([[0, [lcm(bs[0][1][0], bs[1][1][0]), b1[1][1]]]] + bs[2:])

print(reduce_buses_fast(parse_input(inp)))
