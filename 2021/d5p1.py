#!/usr/bin/python3

import collections
import fileinput
import numpy as np
import re
import sys

def solve(inp):
    lines = []
    max_x, max_y = 0, 0
    for line in inp:
        p1, p2 = re.split('\s+->\s', line.strip())
        p1, p2 = point(p1), point(p2)
        if p1[0] != p2[0] and p1[1] != p2[1]:
            continue
        max_x = int(max(max_x, p1[0], p2[0]))
        max_y = int(max(max_x, p1[1], p2[1]))
        lines.append((p1, p2, (p2-p1)/np.linalg.norm(p2-p1)))

    touching = [ [ 0 for x in range(max_x+1) ] for y in range(max_y+1) ]

    for p1, p2, shift in lines:
        pos = p1
        last_touch = np.array([-1, -1])
        stop = False
        while True:
            if not same(pos, last_touch):
                touching[int(pos[0])][int(pos[1])] += 1
                last_touch = np.copy(pos)

            if stop:
                break
            pos += shift
            if same(pos, p2):
                stop = True

    more_than_one = sum([ sum([ 1 if touching[x][y] > 1 else 0 for x in range(max_x+1) ]) for y in range(max_y+1) ])
    print(more_than_one)

def point(s):
    x, y = s.split(',')
    x, y = float(x), float(y)
    return np.array([x, y])

def round(p):
    return np.array(p, dtype=int)

def same(p1, p2):
    return np.array_equal(round(p1), round(p2))

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
