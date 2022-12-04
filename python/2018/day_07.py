from sortedcontainers import SortedList
from string import ascii_uppercase
from collections import defaultdict

def get_input():
    parsed = []
    with open('day_07.txt') as f:
        lines = f.readlines()
        for line in lines:
            line = line.strip()
            parsed.append((line[5], line[-12]))
    return parsed

def part_1():
    inp = get_input()
    unique_chars = set(map(lambda x: x[1], inp))
    path = ''
    queue = SortedList(set(list(ascii_uppercase)) - unique_chars)

    prereqs = defaultdict(set)
    steps = defaultdict(set)

    for prereq, step in inp:
        prereqs[prereq].add(step)
        steps[step].add(prereq)

    while queue:
        curr = queue.pop(0)
        path += curr
        for step in prereqs[curr]:
            steps[step].remove(curr)
            if not steps[step]:
                queue.add(step)

    print(path)

# if we can, we'd like to queue 5 items at the same time, and when the time passes, take another item.
# we need a queue of 5 workers that show how many more seconds they have to work.
# the queue should be sorted, and if we have no more workers, we unqueue the first worker
# and add that amount of time to our current time.
# once we're done with the order, we
def part_2():
    inp = get_input()
    unique_chars = set(map(lambda x: x[1], inp))
    path = ''
    queue = SortedList(set(list(ascii_uppercase)) - unique_chars)

    prereqs = defaultdict(set)
    steps = defaultdict(set)

    for prereq, step in inp:
        prereqs[prereq].add(step)
        steps[step].add(prereq)

    while queue:
        curr = queue.pop(0)
        path += curr
        for step in prereqs[curr]:
            steps[step].remove(curr)
            if not steps[step]:
                queue.add(step)

    print(path)

part_2()
