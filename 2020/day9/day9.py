# day 9 of advent of code


preamble = 25


def part1():
    
    with open('day9/input', 'r') as fp:
        line = fp.readline()
        nums = []
        reached_max = False
    
        while line and not reached_max:
            nums.append(int(line))
            if len(nums) == preamble:
                reached_max = True
            line = fp.readline()

        valid = False
        res = None
        while line:
            new_num = int(line)
            valid = False
            for n in nums:
                if (new_num - n) in nums and new_num != n * 2:
                    valid = True
                    break

            if not valid:
                res = new_num
                break
            
            nums.append(new_num)
            nums.pop(0)
            line = fp.readline()
    
    print(res)



def part2():

    result = 400480901
    
    with open('day9/input', 'r') as fp:
        line = fp.readline()
        nums = []
        reached_max = False
    
        while line and not reached_max:
            nums.append(int(line))
            if len(nums) == preamble:
                reached_max = True
            line = fp.readline()

        fst = 0
        lst = 0
        found = False
        
        while line:
            fst = 0
            lst = 0
            while not found and lst <= preamble:
                total = sum(nums[fst:lst])
                if total == result:
                    found = True
                elif total > result:
                    fst += 1
                else:
                    lst += 1

            if found:
                break

            nums.append(int(line))
            nums.pop(0)
            line = fp.readline()
    
    list = nums[fst:lst]
    print(min(list) + max(list))



def main():
    part1()
    part2()


if __name__ == '__main__':
    main()