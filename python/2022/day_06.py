def get_input() -> str:
    with open('day_06.txt') as f:
        return f.read().strip()

def part_1(inp: str) -> int:
    for i, chars in enumerate(zip(inp, inp[1:], inp[2:], inp[3:])):
        if len(set([*chars])) == 4:
            return i + 4
    return -1

def part_2(inp: str) -> int:
    width = 14
    for i in range(len(inp) - width):
        if len(set(inp[i: i + width])) == width:
            return i + width
    return -1
