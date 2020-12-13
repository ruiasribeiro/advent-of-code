# day 12 of advent of code


dir = 'E'
x = 0
y = 0


def next_dir():

    global dir

    if dir == 'N':
        return 'E'
    elif dir == 'E':
        return 'S'
    elif dir == 'S':
        return 'W'
    elif dir == 'W':
        return 'N'


def prev_dir():

    global dir

    if dir == 'N':
        return 'W'
    elif dir == 'E':
        return 'N'
    elif dir == 'S':
        return 'E'
    elif dir == 'W':
        return 'S'


def move(instr_dir, value):

    global x, y

    if instr_dir == 'N':
        x += value
    elif instr_dir == 'E':
        y += value
    elif instr_dir == 'S':
        x -= value
    elif instr_dir == 'W':
        y -= value


def rotate(turn, degrees):

    global dir

    turns = degrees // 90

    while turns > 0:
        if turn == 'R':
            dir = next_dir()
        else:
            dir = prev_dir()
        turns -= 1

    return dir


def main():
    
    global dir, x, y
    
    with open('day12/input', 'r') as fp:
        line = fp.readline().strip()
        instructions = []
    
        while line:
            instructions.append((line[0], int(line[1:])))
            line = fp.readline().strip()

    dirs = ['N', 'E', 'S', 'W']

    for instr in instructions:
        action = instr[0]
        value = instr[1]

        if action == 'F':
            move(dir, value)
        elif action in ['N', 'E', 'S', 'W']:
            move(action, value)
        elif action in ['L', 'R']:
            rotate(action, value)

    print(abs(x) + abs(y))


if __name__ == '__main__':
    main()