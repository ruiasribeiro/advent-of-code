# day 13 (part 1) of advent of code


def main():
    
    with open('day13/input', 'r') as fp:
        timestamp = int(fp.readline().strip())
        buses = fp.readline().strip().split(',')

    buses = filter(lambda x: x != 'x', buses)
    buses = list(map(lambda x: int(x), buses))
    min_times = map(lambda x: (timestamp // x) * x + x, buses)
    val, idx = min((val, idx) for (idx, val) in enumerate(min_times))

    print(buses[idx] * (val - timestamp))


if __name__ == '__main__':
    main()