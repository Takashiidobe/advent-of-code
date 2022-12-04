from collections import defaultdict
from dateutil import parser

def get_input():
    with open('day_04.txt') as f:
        return [l.strip() for l in f.readlines()]

def part_1():
    inp = get_input()
    guards = defaultdict(list)
    times = defaultdict(int)
    start = None
    end = None
    guard = None

    for line in sorted(inp):
        time, action = line.split('] ')

        time = parser.parse(time[1:])

        if action.startswith('Guard'):
            guard = int(action.split()[1][1:])
        elif action == 'falls asleep':
            start = time
        elif action == 'wakes up':
            end = time
            guards[guard].append((start.minute, end.minute))
            times[guard] += (end - start).seconds

    (guard, time) = max(times.items(), key=lambda i: i[1])
    (minute, count) = max([
        (minute, sum(1 for start, end in guards[guard] if start <= minute < end))
    for minute in range(60)], key=lambda i: i[1])

    print('part 1:', guard * minute)

    (guard, minute, _) = max([
    (guard, minute, sum(1 for start, end in guards[guard] if start <= minute < end))
    for minute in range(60) for guard in guards], key=lambda i: i[2])

    print('part 2:', guard * minute)

part_1()
