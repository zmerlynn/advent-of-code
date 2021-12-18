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
        max_x = int(max(max_x, p1[0], p2[0]))
        max_y = int(max(max_x, p1[1], p2[1]))
        lines.append((p1, p2, np.sign(p2-p1)))

    touching = [ [ 0 for x in range(max_x+1) ] for y in range(max_y+1) ]

    for p1, p2, shift in lines:
        pos = p1
        last_touch = np.array([-1, -1])
        while not same(pos, p2):
            if not same(pos, last_touch):
                touch(touching, pos)
                last_touch = np.copy(pos)
            pos += shift
        if not same(pos, last_touch):
            touch(touching, pos)

    more_than_one = sum([ sum([ 1 if touching[x][y] > 1 else 0 for x in range(max_x+1) ]) for y in range(max_y+1) ])
    print(more_than_one)

def touch(touching, p):
    touching[p[0]][p[1]] += 1
    
def point(s):
    x, y = s.split(',')
    x, y = int(x), int(y)
    return np.array([x, y])

def same(p1, p2):
    return np.array_equal(p1, p2)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
