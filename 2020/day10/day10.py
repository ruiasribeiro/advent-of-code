# day 10 of advent of code


import functools


def arrange(cur, nums):
    count = 0
    to_test = nums[:3]
    n = len(to_test)

    if n == 0:
        return 1

    if n > 0 and nums[0] - cur <= 3:
        count += arrange(nums[0], nums[1:])
    if n > 1 and nums[1] - cur <= 3:
        count += arrange(nums[1], nums[2:])
    if n > 2 and nums[2] - cur <= 3:
        count += arrange(nums[2], nums[3:])

    return count


def main():
    
    with open('day10/input', 'r') as fp:
        line = fp.readline()
        nums = [0]                   # charging outlet
    
        while line:
            nums.append(int(line))
            line = fp.readline()

    nums.sort()
    nums.append(nums[-1] + 3)        # built-in adapter


    ## part 1

    pairs = zip(nums[:len(nums) - 1], nums[1:])
    diff_list = list(map(lambda x: x[1] - x[0], pairs))

    print(diff_list.count(1) * diff_list.count(3))


    ## part 2

    group = []
    groups = []
    for n in nums:
        if not group:
            group.append(n)
        else:
            diff = n - group[-1]
            if diff < 3:
                group.append(n)
            else:
                groups.append(group)
                group = [n]
    groups.append(group)

    # ignore groups with no combinations possible
    groups = list(filter(lambda x: len(x) > 2, groups))

    # calculate combinations in the remaining groups
    combinations = list(map(lambda x: arrange(x[0], x[1:]), groups))

    # multiply all combinations
    count = functools.reduce((lambda x, y: x * y), combinations)

    print(count)



if __name__ == '__main__':
    main()