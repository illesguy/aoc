from utils import get_input_for_day


inp = get_input_for_day(2021, 3, str)

delta = []
epsilon = []

dc = inp.copy()
ec = dc.copy()
for i in range(len(inp[0])):
    zs = 0
    os = 0
    for j in inp:
        if int(j[i]):
            os += 1
        else:
            zs += 1
    if os >= zs:
        delta.append('1')
        epsilon.append('0')
    else:
        delta.append('0')
        epsilon.append('1')

    if len(dc) > 1:
        # print(len(dc))
        # if len(dc) == 6:
        #     print(i, dc)
        zs = 0
        os = 0
        for j in dc:
            if int(j[i]):
                os += 1
            else:
                zs += 1
        # print(os)
        # print(zs)
        # print('-------')
        mc = '1' if os >= zs else '0'
        dc = [v for v in dc if v[i] == mc]

    if len(ec) > 1:
        zs = 0
        os = 0
        for j in ec:
            if int(j[i]):
                os += 1
            else:
                zs += 1
        lc = '0' if os >= zs else '1'
        ec = [v for v in ec if v[i] == lc]

d = int(''.join(delta), 2)
e = int(''.join(epsilon), 2)
print(d, e, d*e)

og = int(dc[0], 2)
co = int(ec[0], 2)

print(og, co, og*co)
