# day 4 of advent of code

import re   # regular expressions 


## part 1 

# read input file
with open('day4/input', 'r') as fp:
    line = fp.readline()
    lines = [line]

    while line:
        line = fp.readline()
        if line != '': 
            lines.append(line)

requiredfields = ['byr', 'ecl', 'eyr', 'hcl', 'hgt', 'iyr', 'pid']

all = ''.join(lines)              # join all strings in list into one
psports = all.split('\n\n')       # each passport is delimited by '\n\n'

valid = 0
for psport in psports:  
    ppfields = psport.split()     # list of key:value in a passport
    
    # split each key:value into a tuple (key, ':', value)
    partitioned = map(lambda x : x.partition(':'), ppfields) 
    
    # gather all keys into a list
    fieldnames = []
    for p in partitioned:
        fieldnames.append(p[0])
    
    # check if all the required fields are in the passport
    isValid = True
    for f in requiredfields:
        if f not in fieldnames:
            isValid = False
            break
    
    # if so, then increment valid passports
    if isValid:
        valid += 1

# print the result
print(valid)


## part 2

valid = 0
for psport in psports:  
    ppfields = psport.split()     # list of key:value in a passport
    
    # split each key:value into a tuple (key, ':', value)
    partitioned = map(lambda x : x.partition(':'), ppfields) 
    
    # 
    isValid = True
    nfields = 0
    for tuple in partitioned:
        if tuple[0] == 'byr':
            nfields += 1
            yr = int(tuple[2])
            if yr < 1920 or yr > 2002:
                isValid = False
                break
        elif tuple[0] == 'iyr':
            nfields += 1
            yr = int(tuple[2])
            if yr < 2010 or yr > 2020:
                isValid = False
                break
        elif tuple[0] == 'eyr':
            nfields += 1
            yr = int(tuple[2])
            if yr < 2020 or yr > 2030:
                isValid = False
                break
        elif tuple[0] == 'hgt':
            nfields += 1
            if re.search('cm$', tuple[2]):
                height = int(re.search('^\d+', tuple[2]).group())
                if height < 150 or height > 193:
                    isValid = False
                    break
            elif re.search('in$', tuple[2]):
                height = int(re.search('^\d+', tuple[2]).group())
                if height < 59 or height > 76:
                    isValid = False
                    break
            else: 
                isValid = False
                break
        elif tuple[0] == 'hcl':
            nfields += 1
            if re.search('^#[0-9a-f]+$', tuple[2]) == None:
                isValid = False
                break
        elif tuple[0] == 'ecl':
            nfields += 1
            if tuple[2] not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
                isValid = False
                break
        elif tuple[0] == 'pid':
            nfields += 1
            if re.search('^\d{9}$', tuple[2]) == None:
                isValid = False
                break
    
    # if so, then increment valid passports
    if isValid and nfields == 7:
        valid += 1

# print the result
print(valid)