from itertools import chain
from copy import deepcopy

mat = [list(x)
       for x in open('input.txt').read().replace('L', '#').splitlines()]


def adj(adjrows, i):
    return list(chain(*[[x[i] for i in range(i-1, i+2)] for x in adjrows]))


def simulate(mat):  # Part A.
    idom = range(len(mat[0]))
    jdom = range(len(mat))
    adjrows = [None, ['.' for x in idom]+['.'], mat[0]+['.']]
    mat_ = deepcopy(mat)
    d = 0
    for j in jdom:
        adjrows.pop(0)
        adjrows.append(mat[j+1]+['.']) if j + \
            1 in jdom else adjrows.append(['.' for x in idom]+['.'])
        for i in idom:
            cell = mat[j][i]
            if cell == "#":
                adjm = adj(adjrows, i).count('#')-1
                if adjm > 3:
                    mat_[j][i] = 'L'
                    d += 1
            elif cell == "L":
                adjm = adj(adjrows, i).count('#')
                if adjm == 0:
                    mat_[j][i] = '#'
                    d += 1
    if d > 0:
        mat = simulate(mat_)
        return mat
    return mat


# Holy [] this is slow!
[print(''.join(x)) for x in simulate(mat)]
count = []
[count.append(x.count('#')) for x in simulate(mat)]
print(sum(count))
