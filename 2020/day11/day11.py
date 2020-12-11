# day 11 of advent of code


n_rows = 0
n_cols = 0
old = []


def valid_pos(pos):
    x = pos[0]
    y = pos[1]
    return 0 <= x < n_rows and 0 <= y < n_cols


def get_adjs(i, j):
    adjs = [(i-1,j-1),(i-1,j),(i-1,j+1),
            (i  ,j-1),        (i  ,j+1),
            (i+1,j-1),(i+1,j),(i+1,j+1)]
    return list(filter(valid_pos, adjs))


def occupied(adjs):
    count = 0
    for (x, y) in adjs:
        if old[x][y] == '#':
            count += 1
    return count


def apply_rules():
    new = []

    for i in range(0, n_rows):
        new_line = []

        for j in range(0, n_cols):
            seat = old[i][j]
            adjs = get_adjs(i, j)
            occ = occupied(adjs)

            # first rule
            if seat == 'L' and occ == 0:
                new_line.append('#')
            elif seat == '#' and occ >= 4:
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