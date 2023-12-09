from inputs import get_input_for_day


def parse_game(line):
    game, showed = line.split(": ")
    game_id = int(''.join(game[5:]))

    sets = {"blue": [], "red": [], "green": []}
    for s in showed.split("; "):
        for c in s.split(", "):
            num, col = c.split(" ")
            sets[col].append(int(num))

    return {"game_id": game_id, "sets": sets}


def is_game_possible(sets, max_red, max_green, max_blue):
    return max(sets["red"]) <= max_red and max(sets["green"]) <= max_green and max(sets["blue"]) <= max_blue


def power_of_game(sets):
    return max(sets["red"]) * max(sets["green"]) * max(sets["blue"])


games = get_input_for_day(2023, 2, parse_game)


result1 = sum([g["game_id"] for g in games if is_game_possible(g["sets"], 12, 13, 14)])
print(result1)

result2 = sum([power_of_game(g["sets"]) for g in games])
print(result2)
