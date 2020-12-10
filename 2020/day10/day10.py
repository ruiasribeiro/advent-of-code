# day 10 of advent of code


def main():
    
    with open('day10/input', 'r') as fp:
        line = fp.readline()
        nums = [0]                   # charging outlet
    
        while line:
            nums.append(int(line))
            line = fp.readline()

    nums.append(max(nums) + 3)       # built-in adapter

    nums.sort()
    pairs = zip(nums[:len(nums)-1], nums[1:])
    diff_list = list(map(lambda x: x[1] - x[0], pairs))

    print(diff_list.count(1) * diff_list.count(3))



if __name__ == '__main__':
    main()