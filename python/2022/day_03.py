def get_input():
    with open('day_03.txt') as f:
        return [line.strip() for line in f.readlines()]

def get_value(char: str):
    if char.isupper():
        return ord(char) - ord('A') + 27
    else:
        return ord(char) - ord('a') + 1

def part_1():
    parsed = []
    for l in get_input():
        mid = len(l) // 2
        parsed.append((l[:mid], l[mid:]))
    shared_vals = map(lambda x: set(x[0]) & set(x[1]), parsed)
    return sum([get_value(list(x)[0]) for x in shared_vals])

def part_2(inp):
    total = 0
    for x, y, z in zip(inp[::3], inp[1::3], inp[2::3]):
        uniq = set(x) & set(y) & set(z)
        total += get_value(list(uniq)[0])
    return total

print(part_2(get_input()))
