from utils import get_input_for_day


def parse_line(line):
    params, pw = line.split(': ')
    bounds, ch = params.split(' ')
    mn, mx = bounds.split('-')
    return int(mn), int(mx), ch, pw.strip()


def pw_is_valid(mn, mx, ch, pw):
    c_count = 0
    for c in pw:
        if c == ch:
            c_count += 1
        if c_count > mx:
            break
    return mn <= c_count <= mx


def pw_is_valid_pt2(mn, mx, ch, pw):
    return (pw[mx - 1] == ch) ^ (pw[mn - 1] == ch)


inp = get_input_for_day(2020, 2)

valid_pws_part1 = 0
valid_pws_part2 = 0
for line in inp:
    mn, mx, ch, pw = parse_line(line)
    if pw_is_valid(mn, mx, ch, pw):
        valid_pws_part1 += 1
    if pw_is_valid_pt2(mn, mx, ch, pw):
        valid_pws_part2 += 1

print(valid_pws_part1)
print(valid_pws_part2)