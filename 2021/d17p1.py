#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

def solve(inp):
    # target area: x=20..30, y=-10..-5
    ranges = next(inp).strip().split(":")[1].split(",")
    range_x, range_y = [ tuple([int(s) for s in s.split('=')[1].split('..')]) for s in ranges ]
    print(range_x, range_y)

    print(hits(6, 0, range_x, range_y))
    max_y = 0
    solutions = 0
    for vx in range(400):
        for vy in range(-500,500):
            local_y = hits(vx, vy, range_x, range_y)
            if local_y is None:
                continue
            print("solution ", vx, vy, local_y)
            solutions += 1
            max_y = max(max_y, local_y)
    print("max y", max_y)
    print(solutions, " solutions")

def hits(vx, vy, range_x, range_y):
    pos_x = 0
    pos_y = 0
    max_y = 0
    while pos_x <= range_x[1] and pos_y >= range_y[0]:
        pos_x = pos_x + vx
        pos_y = pos_y + vy
        max_y = max(max_y, pos_y)

        # print(pos_x, pos_y, vx, vy)
        if (range_x[0] <= pos_x) and (pos_x <= range_x[1]) and (range_y[0] <= pos_y) and (pos_y <= range_y[1]):
            return max_y

        if vx > 0:
            vx -= 1
        elif vx < 0:
            vx += 1
        vy -= 1

        #if vx == 0 and pos_x < range_x[0]:
        #    return None
    return None

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
