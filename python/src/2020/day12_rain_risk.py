from utils import get_input_for_day


inp = get_input_for_day(2020, 12)
# inp = [ 'F10', 'N3', 'F7', 'R90', 'F11']
all_dirs = {0: 'E', 1: 'S', 2: 'W', 3: 'N'}


def apply_actions(actions, cd, c_moved):
    # print(c_moved)
    # print(cd)
    if not actions:
        return abs(c_moved['E'] - c_moved['W']) + abs(c_moved['N'] - c_moved['S'])

    c_act = actions[0]
    act_type = c_act[0]
    act_val = int(c_act[1:])
    # print('----------------')
    # print(c_act)

    if act_type == 'R':
        return apply_actions(actions[1:], (cd + act_val / 90) % 4, c_moved)
    if act_type == 'L':
        return apply_actions(actions[1:], (cd - act_val / 90) % 4, c_moved)
    
    new_moved = c_moved.copy()
    if act_type == 'F':
        new_moved[all_dirs[cd]] += act_val
    else:
        new_moved[act_type] += act_val
    return apply_actions(actions[1:], cd, new_moved)


manhattan = apply_actions(inp, 0, {'E': 0, 'S': 0, 'W': 0, 'N': 0})
print(manhattan)


def apply_actions_for_waypoint(actions, wp, c_moved):
    # print(c_moved)
    # print(wp)
    if not actions:
        return abs(c_moved['E'] - c_moved['W']) + abs(c_moved['N'] - c_moved['S'])

    c_act = actions[0]
    act_type = c_act[0]
    act_val = int(c_act[1:])
    # print('----------------')
    # print(c_act)

    if act_type == 'R':
        new_wp = dict()
        for i in range(4):
            new_wp[all_dirs[i]] = wp[all_dirs[(i - act_val / 90) % 4]]
        return apply_actions_for_waypoint(actions[1:], new_wp, c_moved)
    if act_type == 'L':
        new_wp = dict()
        for i in range(4):
            new_wp[all_dirs[i]] = wp[all_dirs[(i + act_val / 90) % 4]]
        return apply_actions_for_waypoint(actions[1:], new_wp, c_moved)
    if act_type == 'F':
        new_moved = {k: v + wp[k] * act_val for k, v in c_moved.items()}
        return apply_actions_for_waypoint(actions[1:], wp, new_moved)

    new_wp = wp.copy()
    new_wp[act_type] += act_val
    return apply_actions_for_waypoint(actions[1:], new_wp, c_moved)


manhattan_wp = apply_actions_for_waypoint(inp, {'E': 10, 'S': 0, 'W': 0, 'N': 1}, {'E': 0, 'S': 0, 'W': 0, 'N': 0})
print(manhattan_wp)
