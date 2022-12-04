def get_input() -> list[str]:
    with open('day_04.txt') as f:
        return [line.strip() for line in f.readlines()]

def split_range_to_int(l: str) -> tuple[int, int]:
    start, end = l.split('-')
    start, end = int(start), int(end)
    return (start, end)

def parse_input(inp: list[str]) -> list[tuple[tuple[int, int], tuple[int, int]]]:
    res = []
    for l in inp:
        left, right = l.split(',')
        res.append((split_range_to_int(left), split_range_to_int(right)))
    return res

def part_1(inp: list[tuple[tuple[int, int], tuple[int, int]]]) -> int:
    overlapped_shifts = 0
    for (left_start, left_end), (right_start, right_end) in inp:
        if left_start <= right_start and left_end >= right_end:
            overlapped_shifts += 1
        elif right_start <= left_start and right_end >= left_end:
            overlapped_shifts += 1
    return overlapped_shifts

def part_2(inp: list[tuple[tuple[int, int], tuple[int, int]]]) -> int:
    overlapped_shifts = 0
    for (left_start, left_end), (right_start, right_end) in inp:
        if (left_end >= right_start and left_start <= right_end) or (right_end >= left_start and right_start <= left_end):
            overlapped_shifts += 1

    return overlapped_shifts
