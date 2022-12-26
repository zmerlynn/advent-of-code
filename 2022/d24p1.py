#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import multiprocessing
import re
import sys
import json

BIGNUM=1000000000

DIRS={
    '^': (0,-1),
    'v': (0,1),
    '<': (-1,0),
    '>': (1,0),
}

def solve(inp):
    l = next(inp)
    x_max = len(l.strip())-2 # we don't need anything from the first line except the length

    grid = collections.defaultdict(list)
    y=0
    for l in inp:
        if l.startswith('##'):
            break
        l = l.strip().strip('#')
        assert len(l) == x_max
        for x in range(len(l)):
            if l[x] == '.':
                continue
            grid[(x,y)].append(l[x])
        y += 1
    y_max = y

    # our goal is the bottom right, then we add one for move down
    goal = (x_max-1, y_max-1)

    # start from t=1 after move down
    grid = one_step(grid, x_max, y_max)
    possible = {(0,0)}
    time=1
    moves = list(DIRS.values()) + [(0,0)]
    while True:
        time += 1
        grid = one_step(grid, x_max, y_max)
        # print_grid(grid, x_max, y_max)
        next_possible = set()
        for x, y in list(possible):
            # print(x,y)
            for dir in moves:
                x_new, y_new = x + dir[0], y + dir[1]
                if x_new < 0 or x_new >= x_max or y_new < 0 or y_new >= y_max:
                    continue
                if (x_new, y_new) in grid: # blizzard
                    continue
                # print(x_new, y_new)
                next_possible.add((x_new, y_new))
        possible = next_possible
        # print(possible)
        if goal in possible:
            break
    print(time+1)

def one_step(old_grid, x_max, y_max):
    grid = collections.defaultdict(list)
    for (x,y), blizzards in old_grid.items():
        assert blizzards # otherwise why?
        for blizz in blizzards:
            dir = DIRS[blizz]
            x_new, y_new = (x + dir[0]) % x_max, (y + dir[1]) % y_max
            grid[(x_new, y_new)].append(blizz)
    return grid

def print_grid(grid, x_max, y_max):
    for y in range(y_max):
        s = ""
        for x in range(x_max):
            l = grid.get((x,y), [])
            n = len(l)
            if n > 1:
                s += str(n)
            elif n:
                s += l[0]
            else:
                s += '.'
        print(s)
    print()

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
