# day 4 of advent of code

import re   # regular expressions 

def validate(tuple):
    isValid = True
    matched = False

    if tuple[0] == 'byr':
        matched = True
        yr = int(tuple[2])
        if yr < 1920 or yr > 2002:
            isValid = False

    elif tuple[0] == 'iyr':
        matched = True
        yr = int(tuple[2])
        if yr < 2010 or yr > 2020:
            isValid = False

    elif tuple[0] == 'eyr':
        matched = True
        yr = int(tuple[2])
        if yr < 2020 or yr > 2030:
            isValid = False

    elif tuple[0] == 'hgt':
        matched = True
        if re.search('cm$', tuple[2]):
            height = int(re.search('^\d+', tuple[2]).group())
            if height < 150 or height > 193:
                isValid = False
        elif re.search('in$', tuple[2]):
            height = int(re.search('^\d+', tuple[2]).group())
            if height < 59 or height > 76:
                isValid = False
        else: 
            isValid = False

    elif tuple[0] == 'hcl':
        matched = True
        if re.search('^#[0-9a-f]+$', tuple[2]) == None:
            isValid = False

    elif tuple[0] == 'ecl':
        matched = True
        if tuple[2] not in ['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth']:
            isValid = False

    elif tuple[0] == 'pid':
        matched = True
        if re.search('^\d{9}$', tuple[2]) == None:
            isValid = False

    return (isValid, matched)

    

def main(): 
    
    # read input file
    with open('day4/input', 'r') as fp:
        line = fp.readline()
        lines = [line]
    
        while line:
            line = fp.readline()
            if line != '': 
                lines.append(line)
    
    all = ''.join(lines)              # join all strings in list into one
    psports = all.split('\n\n')       # each passport is delimited by '\n\n'

    # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # 

    ## part 1

    requiredfields = ['byr', 'ecl', 'eyr', 'hcl', 'hgt', 'iyr', 'pid']
    
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
    
    # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # # #
    
    ## part 2
    
    valid = 0
    for psport in psports:  
        ppfields = psport.split()     # list of key:value in a passport
        
        # split each key:value into a tuple (key, ':', value)
        partitioned = map(lambda x : x.partition(':'), ppfields) 
        
        # verify each tuple (key, ':', value)
        isValid = True
        nfields = 0
        for tuple in partitioned:
            (validated, matched) = validate(tuple)
            if not validated:
                isValid = False
                break
            if matched:
                nfields += 1
                
        # if the passport is valid and the required fields are present, then increment valid passports
        if isValid and nfields == 7:
            valid += 1
    
    # print the result
    print(valid)


if __name__ == '__main__':
    main()