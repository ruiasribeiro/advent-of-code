# day 6 of advent of code


def main():
    # read input file
    with open('day6/input', 'r') as fp:
        line = fp.readline()
        lines = [line]
    
        while line:
            line = fp.readline()
            if line != '': 
                lines.append(line)

    # # # # # # # # # # # # # # # # # # # # # # # # #
    
    ## part 1
    group = ''
    groups = []
    for s in lines:
        if s != '\n':
            group = group + s.strip()
        else:
            groups.append("".join(set(group)))
            group = ''

    groups.append("".join(set(group)))
    result = sum(map(lambda x : len(x), groups))
    print(result)


if __name__ == '__main__':
    main()