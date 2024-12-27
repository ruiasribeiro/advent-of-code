# day 12 (part 2) of advent of code


# waypoint
x_way = 10
y_way = 1

# ship
x = 0
y = 0


def rot_right():

    global x_way, y_way

    temp  = y_way
    y_way = - x_way
    x_way = temp 


def rot_left():

    global x_way, y_way

    temp  = x_way 
    x_way = - y_way
    y_way = temp 


def move_way(instr_dir, value):

    global x_way, y_way

    if instr_dir == 'N':
        y_way += value
    elif instr_dir == 'E':
        x_way += value
    elif instr_dir == 'S':
        y_way -= value
    elif instr_dir == 'W':
        x_way -= value


def move_ship(value):

    global x, y

    x += x_way * value
    y += y_way * value


def rotate(turn, degrees):

    turns = degrees // 90

    while turns > 0:
        if turn == 'R':
            rot_right()
        else:
            rot_left()
        turns -= 1


def main():
    
    with open('day12/input', 'r') as fp:
        line = fp.readline().strip()
        instructions = []
    
        while line:
            instructions.append((line[0], int(line[1:])))
            line = fp.readline().strip()

    for instr in instructions:
        action = instr[0]
        value = instr[1]

        if action == 'F':
            move_ship(value)
        elif action in ['N', 'E', 'S', 'W']:
            move_way(action, value)
        elif action in ['L', 'R']:
            rotate(action, value)

    print(abs(x) + abs(y))


if __name__ == '__main__':
    main()