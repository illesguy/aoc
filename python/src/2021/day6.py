from utils import get_input_for_day
from collections import defaultdict


inp = get_input_for_day(2021, 6, str)


fish = defaultdict(lambda: 0)
for f in inp[0].split(","):
    fish[f] = fish[f] + 1


for _ in range(5):
    print(fish)
    zeroes = fish[0]
    for i in range(8):
        fish[i] = fish[i + 1]
    fish[6] += zeroes
    fish[8] = zeroes
    print("=======")

count = sum(v for _, v in fish.items())

print(count)
