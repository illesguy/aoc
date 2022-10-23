import re
from utils import get_input_for_day

inp = get_input_for_day(2020, 4)
passports = ' '.join(inp).split('  ')


def passport_has_valid_fields(passport):
    passport_fields_to_check = ['byr:', 'iyr:', 'eyr:', 'hgt:', 'hcl:', 'ecl:', 'pid:']
    for passport_field in passport_fields_to_check:
        if passport_field not in passport:
            return False
    return True


def passport_has_valid_fields_and_values(passport):
    passport_fields_and_values_to_check = [
        '.*byr:((19[2-9][0-9])|(200[0-2])).*',
        '.*iyr:((201[0-9])|(2020)).*',
        '.*eyr:((202[0-9])|(2030)).*',
        '.*hgt:((((1[5-8][0-9])|(19[0-3]))cm)|(((59)|(6[0-9])|(7[0-6]))in)).*',
        '.*hcl:#[a-f0-9]{6}.*',
        '.*ecl:((amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)).*',
        '.*pid:\d{9}((\D.*)|$)',
    ]

    for passport_value in passport_fields_and_values_to_check:
        if not re.search(passport_value, passport):
            return False
    return True


passports_with_valid_fields = 0
passports_with_valid_fields_and_values = 0

for p in passports:
    if passport_has_valid_fields(p):
        passports_with_valid_fields += 1
        if passport_has_valid_fields_and_values(p):
            passports_with_valid_fields_and_values += 1

print(passports_with_valid_fields)
print(passports_with_valid_fields_and_values)
