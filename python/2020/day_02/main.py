import re


def get_input(path_name="./input.txt"):
    with open(path_name) as f:
        return list(x.strip() for x in f.readlines())


def part_1():
    regex = '(-| |: )'
    parsed_input = [(int(re.split(regex, x)[0]), int(re.split(regex, x)[
                     2]), re.split(regex, x)[4], re.split(regex, x)[-1]) for x in get_input()]

    count = 0
    for minimum, maximum, letter, sequence in parsed_input:
        if sequence.count(letter) >= minimum and sequence.count(letter) <= maximum:
            count += 1
    return count


def part_2():
    regex = '(-| |: )'
    parsed_input = [(int(re.split(regex, x)[0]), int(re.split(regex, x)[
                     2]), re.split(regex, x)[4], re.split(regex, x)[-1]) for x in get_input()]

    count = 0
    for first_loc, second_loc, letter, sequence in parsed_input:
        if sequence[first_loc - 1] == letter and sequence[second_loc - 1] == letter:
            pass
        elif sequence[first_loc - 1] == letter:
            count += 1
        elif sequence[second_loc - 1] == letter:
            count += 1
        else:
            pass

    return count


if __name__ == "__main__":
    print(part_1())  # 638
    print(part_2())  # 699
