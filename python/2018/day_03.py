import re

def get_input():
    with open('day_03.txt') as f:
        return f.readlines()

def part_1():
    s = get_input()

    match_regex = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')

    matrix = [[0 for _ in range(1000)] for _ in range(1000)]

    for l in s:
        claim, left, top, width, height = re.search(match_regex, l).groups()
        claim, left, top, width, height = int(claim), int(left), int(top), int(width), int(height)
        for y in range(top, top + height):
            for x in range(left, left + width):
                matrix[y][x] += 1

    count = 0
    for y in range(len(matrix)):
        for x in range(len(matrix[0])):
            if matrix[y][x] > 1:
                count += 1

    print(count)

def part_2():
    s = get_input()
    match_regex = re.compile(r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)')

    matrix = [[0 for _ in range(1000)] for _ in range(1000)]

    for l in s:
        claim, left, top, width, height = re.search(match_regex, l).groups()
        claim, left, top, width, height = int(claim), int(left), int(top), int(width), int(height)
        for y in range(top, top + height):
            for x in range(left, left + width):
                matrix[y][x] += 1

    for l in s:
        claim, left, top, width, height = re.search(match_regex, l).groups()
        claim, left, top, width, height = int(claim), int(left), int(top), int(width), int(height)
        is_possible = True
        for y in range(top, top + height):
            for x in range(left, left + width):
                if matrix[y][x] != 1:
                    is_possible = False

        if is_possible:
            print(claim)

part_2()
