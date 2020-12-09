# day 9 of advent of code


def main():

    preamble = 25
    
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



if __name__ == '__main__':
    main()