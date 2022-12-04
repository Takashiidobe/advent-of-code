def get_input():
    with open('day_01.txt') as f:
        return [line.strip() for line in f.readlines()]

def parse_input():
    calories = []
    curr_calories = 0
    for line in get_input():
        if line:
            curr_calories += int(line)
        else:
            calories.append(curr_calories)
            curr_calories = 0
    return calories

def part_1():
    return max(parse_input())

def part_2():
    return sum(sorted(parse_input())[-3:])

print(part_2())
