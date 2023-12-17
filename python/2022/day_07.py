from collections import defaultdict
from itertools import accumulate

def get_input() -> defaultdict[str, int]:
    dirs = defaultdict(int)
    curr = []

    for line in open('day_07.txt'):
        match line.split():
            case '$', 'cd', '/':
                curr = ['']
            case '$', 'cd', '..':
                curr.pop()
            case '$', 'cd', x:
                curr.append(x + '/')
            case '$', 'ls':
                pass
            case 'dir', _:
                pass
            case size, _:
                for p in accumulate(curr):
                    dirs[p] += int(size)

    return dirs

def part_1(inp: defaultdict[str, int]) -> int:
    return sum(s for s in inp.values() if s <= 100_000)

def part_2(inp: defaultdict[str, int]) -> int:
    return min(s for s in inp.values() if s >= inp[''] - 40_000_000)

part_1(get_input())
