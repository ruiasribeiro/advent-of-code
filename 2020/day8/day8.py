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

    # # # # # # # # # # # # # # # # # # # # # # # # #

    ## part 2

    n_instr = len(instructions)
    instr_original = instructions.copy()
    acc = 0

    # change instruction one by one and test each time
    for idx in range(0, n_instr):
        instructions = instr_original.copy()
        change = instructions[idx]

        if change[0] == 'nop' and change[1] != 0:
            instructions[idx] = ('jmp', change[1])
        elif change[0] == 'jmp':
            instructions[idx] = ('nop', change[1])

        executed = [False] * n_instr
        acc = 0
        i = 0
        while i < n_instr:
            if executed[i]:
                break
            executed[i] = True
            instr = instructions[i]
            if instr[0] == 'acc':
                acc += instr[1]
                i += 1
            elif instr[0] == 'jmp':
                i += instr[1]
            elif instr[0] == 'nop':
                i += 1

        if i == n_instr:
            break
        
    print(acc)
    

if __name__ == '__main__':
    main()