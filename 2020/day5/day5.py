# day 5 of advent of code


def main():
    # read input file
    with open('day5/input', 'r') as fp:
        line = fp.readline()
        lines = [line]
    
        while line:
            line = fp.readline()
            if line != '': 
                lines.append(line)

    # # # # # # # # # # # # # # # # # # # # # # # # #
    
    ## part 1

    biggest = 0
    ids = []
    for line in lines:

        lower = 0
        higher = 127
        for letter in line[0:7]:
            if letter == 'F':
                higher = (lower + higher) // 2
            elif letter == 'B':
                lower = ((lower + higher) // 2) + 1

        row = lower

        lower = 0
        higher = 7
        for letter in line[7:10]:
            if letter == 'L':
                higher = (lower + higher) // 2
            elif letter == 'R':
                lower = ((lower + higher) // 2) + 1
        
        column = lower

        id = row * 8 + column
        ids.append(id)

        if id > biggest:
            biggest = id

    print(biggest)

    # # # # # # # # # # # # # # # # # # # # # # # # #

    ## part 2

    ids.sort()
    result = 0
    for i in range(127 * 8 + 7):
        if ids[i + 1] - ids[i] > 1:
            result = ids[i] + 1
            break

    print(result)


if __name__ == '__main__':
    main()