def get_input() -> list[list[int]]:
    res = []
    with open ('day_08.txt') as f:
        for line in f.readlines():
            res.append([int(x) for x in line.strip()])
    return res

def part_1(inp: list[list[int]]) -> int:
    m = len(inp)
    n = len(inp[0])
    visibility = [[False for _ in range(m)] for _ in range(n)]
    count = 0

    def inbounds(y, x):
        return 0 <= y < m and 0 <= x < n
    is_visible = False
    def dfs(y, x, starting_height, is_start, visited) -> bool:
        nonlocal is_visible
        if is_visible:
            return True
        dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        if (y, x) in visited:
            return False
        else:
            visited.add((y, x))

        if not inbounds(y, x):
            return False
        elif not is_start and inp[y][x] >= starting_height:
            return False
        elif inp[y][x] < starting_height and visibility[y][x]:
            is_visible = True
            return True
        else:
            return any([dfs(dy + y, dx + x, starting_height, False, visited) for dy, dx in dirs])
    for y in range(m):
        for x in range(n):
            if (y == 0 or y == m - 1) or (x == 0 or x == n - 1):
                visibility[y][x] = True
                count += 1
            else:
                is_visible = False
                res = dfs(y, x, inp[y][x], True, set())
                if res:
                    count += 1
                visibility[y][x] = res

    return count

def part_2(_: list[list[int]]) -> int:
    return 0

part_1(get_input())
