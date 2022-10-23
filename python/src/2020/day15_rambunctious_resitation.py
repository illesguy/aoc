from utils import get_input_for_day
from collections import defaultdict


inp = get_input_for_day(2020, 15)[0].split(',')
# inp = ['0', '3', '6']
numbers = defaultdict(list)

prev = 0
for i, num in enumerate(inp):
    nm = int(num)
    numbers[nm].append(i)
    prev = nm

i = len(inp)
while i < 30_000_000:
    if len(numbers[prev]) == 1:
        prev = 0
        numbers[prev].append(i)
    else:
        prev = numbers[prev][-1] - numbers[prev][-2]
        numbers[prev].append(i)
    i += 1

print(prev)
