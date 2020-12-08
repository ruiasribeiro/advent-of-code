# day 7 of advent of code

import re


# for part 1
mybag = 'shiny gold'
def search_bag(dict, bags):
    for bag in bags:
        if bag[1] == mybag:
            return True
        if search_bag(dict, dict[bag[1]]):
            return True
    return False


# for part 2
def bags_inside(dict, bags):
    count = 0
    for bag in bags:
        bags_in = bags_inside(dict, dict[bag[1]])
        count += int(bag[0]) * (bags_in + 1)
    return count


def main():
    
    # read input file
    with open('day7/input', 'r') as fp:
        line = fp.readline()
        dict = {}
    
        while line:
            line = line.strip().split('.')[0]    # clean line 
            splitted = line.split(' contain ')   # split line between key and value
            bags = splitted[1].split(', ')       # split value into a list of values
            key = splitted[0].split(' bag')[0]   # remove 'bag' from key

            if bags[0].find('no other bags') >= 0:
                dict[key] = ()
            else:
                # regex for each bag
                regex = re.compile("^(\d+) (.+)$")
                # remove bag from all values
                values = list(map(lambda x: x.split(' bag')[0], bags))
                # store values as a list of tuples (quantity, type of bag)
                dict[key] = list(map(lambda x: re.search(regex, x).group(1, 2), values)) 

            line = fp.readline()

    # # # # # # # # # # # # # # # # # # # # # # # # #
    
    ## part 1
            
    count = 0
    for bag in dict:
        count += search_bag(dict, dict[bag])

    print(count)

    # # # # # # # # # # # # # # # # # # # # # # # # #
    
    ## part 2

    print(bags_inside(dict, dict[mybag]))


if __name__ == '__main__':
    main()