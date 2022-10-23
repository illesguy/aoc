from utils import get_input_for_day


inp = get_input_for_day(2020, 14)


def parse_mask_line_p1(mask_line):
    mask = mask_line.replace('mask = ', '')
    and_mask = int(mask.replace('X', '1'), 2)
    or_mask = int(mask.replace('X', '0'), 2)
    return and_mask, or_mask


def generate_addresses_to_update(address, mask):
    perm_count = mask.count('X')
    address_to_float = '{0:036b}'.format(int(mask.replace('X', '0'), 2) | address)

    addresses = []
    for p in range(2 ** perm_count):
        p_bin = f'{p:b}'.zfill(perm_count)
        current_address = [c for c in address_to_float]
        ind = 0
        for i in range(len(mask)):
            if mask[i] == 'X':
                current_address[i] = p_bin[ind]
                ind += 1
        addresses.append(int(''.join(current_address), 2))
    return addresses


def parse_mem_line(mem_line):
    addr, val = mem_line.replace('mem[', '').split('] = ')
    return int(addr), int(val)


memory = dict()
memory2 = dict()
for line in inp:
    if line.startswith('mask = '):
        current_and, current_or = parse_mask_line_p1(line)
        current_mask = line.replace('mask = ', '')
    else:
        address, value = parse_mem_line(line)
        memory[address] = (value & current_and) | current_or
        addresses_to_update = generate_addresses_to_update(address, current_mask)
        for addr in addresses_to_update:
            memory2[addr] = value


print(sum(memory.values()))
print(sum(memory2.values()))
