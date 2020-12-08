# day 8 of advent of code


def main():

    # read input file
    with open('day8/input', 'r') as fp:
        line = fp.readline()
        instructions = []
    
        while line: 
            list = line.strip().split()
            instructions.append((list[0], int(list[1])))
            line = fp.readline()

    # # # # # # # # # # # # # # # # # # # # # # # # #
    
    ## part 1

    i = 0
    acc = 0
    executed = [False] * len(instructions)
    while not executed[i]:
        executed[i] = True
        instr = instructions[i]

        if instr[0] == 'acc':
            acc += instr[1]
            i += 1
        elif instr[0] == 'jmp':
            i += instr[1]
        elif instr[0] == 'nop':
            i += 1

    print(acc)


if __name__ == '__main__':
    main()