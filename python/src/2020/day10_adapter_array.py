from utils import get_input_for_day

inp = get_input_for_day(2020, 10, int)
# inp = [ 16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4]
# inp = [28,33,18,42,31,14,46,20,48,47,24,23,49,45,19,38,39,11,1,32,25,35,8,17,7,9,4,2,34,10,3]
sort_inp = [0] + sorted(inp)
# print(len(sort_inp))
# print(sort_inp)

j1 = 0
j2 = 0
j3 = 1

for i in range(len(sort_inp) - 1):
    d = sort_inp[i + 1] - sort_inp[i]
    if d == 1:
        j1 += 1
    elif d == 2:
        j2 += 1
    elif d == 3:
        j3 += 1

print(j1)
print(j2)
print(j3)
print(j1 * j3)


paths = {0: 1}
for jolt in sort_inp[1:]:
    paths[jolt] = paths.get(jolt - 1, 0) + paths.get(jolt - 2, 0) + paths.get(jolt - 3, 0)

print(paths[sort_inp[-1]])
