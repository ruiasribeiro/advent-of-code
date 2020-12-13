# day 11 (part 2) of advent of code


n_rows = 0
n_cols = 0
old = []


def valid_pos(pos):
    x = pos[0]
    y = pos[1]
    return 0 <= x < n_rows and 0 <= y < n_cols


def is_first_seat_occupied(i, j, dir_i, dir_j):
    i += dir_i
    j += dir_j

    is_occupied = False
    while not is_occupied and valid_pos((i, j)):
        if old[i][j] == '#':
            is_occupied = True
        elif old[i][j] == 'L':
            break
        else:
            i += dir_i
            j += dir_j

    return is_occupied


def occupied(x, y):
    adjs = [(x,y,-1,-1),(x,y,-1, 0),(x,y,-1,+1),
            (x,y, 0,-1),            (x,y, 0,+1),
            (x,y,+1,-1),(x,y,+1, 0),(x,y,+1,+1)]
    
    count = 0
    for i, j, dir_i, dir_j in adjs:
        count += is_first_seat_occupied(i, j, dir_i, dir_j)

    return count


def apply_rules():
    new = []

    for i in range(0, n_rows):
        new_line = []

        for j in range(0, n_cols):
            seat = old[i][j]
            occ = occupied(i, j)

            if seat == 'L' and occ == 0:
                new_line.append('#')
            elif seat == '#' and occ >= 5:
                new_line.append('L')
            else:
                new_line.append(seat)

        new.append(new_line)

    return new


def main():
    global n_rows
    global n_cols 
    global old
    
    with open('day11/input', 'r') as fp:
        line = fp.readline()
        lines = []
    
        while line:
            lines.append(line.strip())
            line = fp.readline()

    n_rows = len(lines)
    n_cols = len(lines[0])
    old = lines.copy()
    new = apply_rules()

    while new != old:
        old = new
        new = apply_rules()

    occupied_list = [[y == '#' for y in x] for x in new]
    print(sum([sum(x) for x in occupied_list]))


if __name__ == '__main__':
    main()