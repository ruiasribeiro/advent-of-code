# day 6 of advent of code

import string

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

    # # # # # # # # # # # # # # # # # # # # # # # # #

    ## part 2

    group = ''
    total = 0
    persons = 0
    for s in lines:
        if s != '\n':
            persons += 1
            group = group + s.strip()
        else:
            for c in string.ascii_lowercase:
                if group.count(c) == persons:
                    total += 1
            group = ''
            persons = 0

    for c in string.ascii_lowercase:
        if group.count(c) == persons:
            total += 1

    print(total)


if __name__ == '__main__':
    main()