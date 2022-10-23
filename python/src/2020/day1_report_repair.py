from utils import get_input_for_day


inp = get_input_for_day(2020, 1, int)


def get_report_2parts(exps):
    numbers = set(exps)
    for num in exps:
        if 2020 - num in numbers:
            return num * (2020 - num)
    raise ValueError('not found')


def get_report_3parts(exps):
    numbers = set(exps)
    for i in range(len(exps) - 3):
        for j in range(i + 1, len(exps) - 2):
            if 2020 - (exps[i] + exps[j]) in numbers:
                return exps[i] * exps[j] * (2020 - (exps[i] + exps[j]))
    raise ValueError('not found')


print(get_report_2parts(inp))
print(get_report_3parts(inp))
