from utils import get_input_for_day


inputs = get_input_for_day(2020, 8)


def find_infinite_loop(instructions, accumulator=0, command_index=0, commands_executed=None):
    commands_executed = set() if commands_executed is None else commands_executed

    while True:
        if command_index == len(instructions):
            return True, accumulator
        if command_index in commands_executed:
            return False, accumulator
        else:
            commands_executed.add(command_index)
        command, param = instructions[command_index].split(' ')
        if command == 'acc':
            accumulator += int(param)
            command_index += 1
        elif command == 'jmp':
            command_index += int(param)
        else:
            command_index += 1


def fixed_output(instructions):
    accumulator = 0
    command_index = 0
    commands_executed = set()
    while True:
        commands_executed.add(command_index)
        command, param = instructions[command_index].split(' ')
        if command == 'acc':
            accumulator += int(param)
            command_index += 1
        elif command == 'jmp':
            success, acc = find_infinite_loop(instructions, accumulator, command_index + 1, commands_executed)
            if success:
                return acc
            command_index += int(param)
        else:
            success, acc = find_infinite_loop(instructions, accumulator, command_index + int(param), commands_executed)
            if success:
                return acc
            command_index += 1


print(find_infinite_loop(inputs)[1])
print(fixed_output(inputs))
