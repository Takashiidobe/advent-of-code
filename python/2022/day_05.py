def get_input() -> list[str]:
    with open('day_05.txt') as f:
        return [line.strip() for line in f.readlines()]

def parse_crates(inp: list[str]) -> list[list[str]]:
    res = [[] for _ in range(10)]
    for line in inp:
        for i, c in enumerate(line):
            if c.isalnum():
                res[(i // 4) + 1].append(c)

    for l in res:
        l.reverse()

    return res

def parse_moves(inp: list[str]) -> list[tuple[int, int, int]]:
    res = []
    for l in inp:
        _, crate_to_move, _, from_location, _, to_location  = l.split(' ')
        res.append((int(crate_to_move), int(from_location), int(to_location)))

    return res

def parse_input(inp: list[str]) -> tuple[list[list[str]], list[tuple[int, int, int]]]:
    crates = parse_crates(inp[:8])
    moves = parse_moves(inp[10:])
    return (crates, moves)

def part_1(inp: tuple[list[list[str]], list[tuple[int, int, int]]]) -> str:
    crates, moves = inp

    for move, from_loc, to_loc in moves:
        popped_crates = []
        for _ in range(move):
            if crates[from_loc]:
                popped_crate = crates[from_loc].pop()
                popped_crates.append(popped_crate)
        crates[to_loc].extend(popped_crates)

    return ''.join(map(lambda x: x[-1], crates[1:]))

def part_2(inp: tuple[list[list[str]], list[tuple[int, int, int]]]) -> str:
    crates, moves = inp

    for move, from_loc, to_loc in moves:
        popped_crates = []
        for _ in range(move):
            if crates[from_loc]:
                popped_crate = crates[from_loc].pop()
                popped_crates.append(popped_crate)
        popped_crates.reverse()
        crates[to_loc].extend(popped_crates)

    return ''.join(map(lambda x: x[-1], crates[1:]))
