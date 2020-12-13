# day 2 of advent of code


## part 1 

# read input file
with open('day2/input', 'r') as fp:
    line = fp.readline()
    lines = [line]

    while line:
        line = fp.readline()
        if line != '': 
            lines.append(line)

# validate passwords
valid = 0
for l in lines:
    splitted = l.split()
    tuple = splitted[0].partition('-')
    minimum = int(tuple[0])
    maximum = int(tuple[2])
    char = splitted[1][0]
    password = splitted[2]

    count = 0
    for c in password:
        if c == char:
            count += 1
        if count > maximum:
            break

    if count >= minimum and count <= maximum:
        valid += 1

# print the result
print(valid)


## part 2

# validate passwords
valid = 0
for l in lines:
    splitted = l.split()
    tuple = splitted[0].partition("-")
    fst = int(tuple[0]) - 1
    snd = int(tuple[2]) - 1
    char = splitted[1][0]
    password = splitted[2]
    length = len(password)

    count = 0
    if fst < length and password[fst] == char:
        count += 1
    if snd < length and password[snd] == char:
        count += 1

    if count == 1:
        valid += 1

# print the result
print(valid)