import os
from inputs import get_input_for_day

rules = get_input_for_day(2020, 7)


def parse_child_rule(child_rule):
    child_tokens = child_rule.split(' ')
    return int(child_tokens[0]), ' '.join(child_tokens[1:])


def parse_rule(rule):
    parent, child_rules = rule[:-1].replace('bags', 'bag').split(' contain ')
    if child_rules.startswith('no'):
        return parent, []
    children = [parse_child_rule(cr) for cr in child_rules.split(', ')]
    return parent, children


relations = dict()
for r in rules:
    p, c = parse_rule(r)
    relations[p] = c


def count_children(parent):
    children = relations.get(parent, [])
    if children:
        return 1 + sum([cc * count_children(c) for cc, c in children])
    else:
        return 1


print(count_children('shiny gold bag') - 1)

