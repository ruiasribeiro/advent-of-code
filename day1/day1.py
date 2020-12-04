# day 1 of advent of code


## part 1 

# read input file
with open('day1/input', 'r') as fp:
    line = fp.readline()
    nums = [int(line)]

    while line:
        line = fp.readline()
        if line != '': 
            nums.append(int(line))

# determine the two entries
res = None
for n1 in nums:
    for n2 in nums:
        if n1 != n2 and n1 + n2 == 2020:
            res = n1 * n2
            break
    if res != None:
        break

# print the result
print(res)


## part 2

# determine the three entries
res = None
for n1 in nums:
    for n2 in nums:
        for n3 in nums:
            if n1 != n2 and n2 != n3 and n1 + n2 + n3 == 2020:
                res = n1 * n2 * n3
                break
        if res != None:
            break
    if res != None:
        break

# print the result
print(res)