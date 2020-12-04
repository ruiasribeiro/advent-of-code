# day 3 of advent of code


## part 1 

# read input file
with open('day3/input', 'r') as fp:
    line = fp.readline()
    lines = [line]

    while line:
        line = fp.readline()
        if line != '': 
            lines.append(line)

# calculate the number of trees
length = len(lines)
width = len(lines[0]) - 1      # watch out for the \n
pos = (0, 0)
trees = 0

while pos[1] < length:
    if lines[pos[1]][pos[0]] == '#':
        trees += 1
    pos = ((pos[0] + 3) % width, pos[1] + 1)

# display the result
print(trees)


## part 2 - what a mess

# calculate all the slopes
pos = (0, 0)
trees11 = 0
while pos[1] < length:
    if lines[pos[1]][pos[0]] == '#':
        trees11 += 1
    pos = ((pos[0] + 1) % width, pos[1] + 1)
pos = (0, 0)
trees51 = 0
while pos[1] < length:
    if lines[pos[1]][pos[0]] == '#':
        trees51 += 1
    pos = ((pos[0] + 5) % width, pos[1] + 1)
pos = (0, 0)
trees71 = 0
while pos[1] < length:
    if lines[pos[1]][pos[0]] == '#':
        trees71 += 1
    pos = ((pos[0] + 7) % width, pos[1] + 1)
pos = (0, 0)
trees12 = 0
while pos[1] < length:
    if lines[pos[1]][pos[0]] == '#':
        trees12 += 1
    pos = ((pos[0] + 1) % width, pos[1] + 2)

# display the result
print(trees11 * trees * trees51 * trees71 * trees12)