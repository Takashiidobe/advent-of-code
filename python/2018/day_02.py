from typing import Iterable
from itertools import combinations

def get_input():
    with open('day_02.txt') as f:
        return f.readlines()

def main():
    s = get_input()
    stripped = sorted([x.strip() for x in s])
    print(part2(stripped))

def matching_letters(a: str, b: str) -> str:
    return ''.join(a for (a, b) in zip(a, b) if a == b)

def is_correct_pair(a: str, b: str) -> bool:
    return len(matching_letters(a, b)) == len(a) - 1

def part2(box_ids: Iterable[str]) -> str:
    matching_pair = next(t for t in combinations(box_ids, 2) if is_correct_pair(*t))
    return matching_letters(*matching_pair)

if __name__ == "__main__":
    main()
