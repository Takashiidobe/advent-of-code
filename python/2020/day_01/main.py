def get_input(path_name='./input.txt'):
    with open(path_name) as f:
        return [int(x) for x in f.read().splitlines()]


def test_input():
    get_input('./test.txt')


def part_1():
    dt = {}
    for i in get_input():
        if i in dt:
            return dt[i] * i
        else:
            dt[2020 - i] = i


def part_2():
    items = get_input()
    dt = {}
    for i in range(len(items)):
        for j in range(len(items)):
            dt[2020 - items[i] - items[j]] = (items[i], items[j])

    for item in items:
        if item in dt:
            return item * dt[item][0] * dt[item][1]


if __name__ == "__main__":
    print(f"The solution to part 1 is: {part_1()}")  # 987339
    print(f"The solution to part 2 is: {part_2()}")  # 259521570
