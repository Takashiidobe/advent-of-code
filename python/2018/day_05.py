from sortedcontainers import SortedList
import string

def get_input():
    with open('day_05.txt') as f:
        return f.read().strip()

def flipped_case(left_c, right_c):
    return abs(ord(left_c) - ord(right_c)) == 32

def part_1():
    lowercase_chars = list(string.ascii_lowercase)
    uppercase_chars = list(string.ascii_uppercase)
    inp = get_input()

    min_chars_so_far = 50000

    for lowercase, uppercase in zip(lowercase_chars, uppercase_chars):
        sd = SortedList()
        for i, c in enumerate(inp):
            if c != lowercase and c != uppercase:
                sd.add((i, c))

        i = 0
        j = 1
        while j < len(sd):
            _, left_c = sd[i]
            _, right_c = sd[j]

            if flipped_case(left_c, right_c):
                del sd[i], sd[i]
                if i > 0:
                    i -= 1
                j = i + 1
            else:
                i += 1
                j += 1

        min_chars_so_far = min(min_chars_so_far, len(sd))

    print(min_chars_so_far)

part_1()
