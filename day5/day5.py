# day 5 of advent of code


## part 1

def main():
    # read input file
    with open('day5/input', 'r') as fp:
        line = fp.readline()
        lines = [line]
    
        while line:
            line = fp.readline()
            if line != '': 
                lines.append(line)

    biggest = 0
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

        if (row * 8 + column) > biggest:
            biggest = row * 8 + column 

    print(biggest)


if __name__ == '__main__':
    main()