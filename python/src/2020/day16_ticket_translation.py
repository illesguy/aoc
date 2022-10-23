from utils import get_input_for_day


inp = get_input_for_day(2020, 16)
inp_text = '\n'.join(inp)
# inp_text =\
# '''class: 0-1 or 4-19
# row: 0-5 or 8-19
# seat: 0-13 or 16-19

# your ticket:
# 11,12,13

# nearby tickets:
# 3,9,18
# 15,1,5
# 5,14,9'''

rule_lines, my_ticket_lines, nearby_ticket_lines = inp_text.split('\n\n')


def parse_rule_line(rule_line):
    line_split = rule_line.split(' ')
    r1 = line_split[-3]
    r2 = line_split[-1]

    def parse_range(range_text):
        s, e = range_text.split('-')
        return set(range(int(s), int(e) + 1))

    return parse_range(r1).union(parse_range(r2))


rules = [parse_rule_line(r) for r in rule_lines.split('\n')]
tickets = [[int(n) for n in t.split(',')] for t in nearby_ticket_lines.split('\n')[1:]]


def field_is_valid(field):
    for r in rules:
        if field in r:
            return True
    return False


ticket_error = 0
valid_tickets = []
for t in tickets:
    valid = True
    for f in t:
        if not field_is_valid(f):
            ticket_error += f
            valid = False
            break
    if valid:
        valid_tickets.append(t)

print(ticket_error)

positions = list(range(len(rules)))
rule_positions = {p: set(positions) for p in positions}

for t in valid_tickets:
    for i in range(len(t)):
        possible_positions = {p for p in range(len(rules)) if t[i] in rules[p]}
        rule_positions[i] = rule_positions[i].intersection(possible_positions)

print(rule_positions)
ordered = sorted(list(rule_positions.items()), key=lambda e: len(e[1]), reverse=True)

final_positions = dict()
for i in range(len(ordered) - 1):
    pos = ordered[i][1].difference(ordered[i + 1][1])
    print(pos)
    assert len(pos) == 1
    final_positions[ordered[i][0]] = pos.pop()
final_positions[ordered[-1][0]] = ordered[-1][1].pop()

my_ticket_values = [int(v) for v in my_ticket_lines.split('\n')[1].split(',')]
print(my_ticket_values)

print(final_positions)
fp_flipped = {v: k for k, v in final_positions.items()}
mult = 1
for i in range(6):
    field_to_get = fp_flipped[i]
    mult *= my_ticket_values[field_to_get]

print(mult)
