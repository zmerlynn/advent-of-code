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
MOVES = list(DIRS.values()) + [(0,0)]


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

    time = 0
    time, grid = find_goal((0,-1), (x_max-1,y_max), time, grid, x_max, y_max)
    time, grid = find_goal((x_max-1,y_max), (0,-1), time, grid, x_max, y_max)
    time, grid = find_goal((0,-1), (x_max-1,y_max), time, grid, x_max, y_max)
    print(time)


def find_goal(start, goal, time, grid, x_max, y_max):
    possible = {start}
    while True:
        time, grid = one_step(time, grid, x_max, y_max)
        next_possible = set()
        for x, y in list(possible):
            # print(x,y)
            for dir in MOVES:
                x_new, y_new = x + dir[0], y + dir[1]
                if not in_bounds(x_new, y_new, x_max, y_max):
                    continue
                if (x_new, y_new) in grid: # blizzard
                    continue
                # print(x_new, y_new)
                next_possible.add((x_new, y_new))
        assert next_possible, "Help, I'm trapped!"
        possible = next_possible
        # print_grid(grid, x_max, y_max, possible)
        # print(possible)
        if goal in possible:
            break
    return time, grid

def in_bounds(x, y, x_max, y_max):
    if (x,y) == (0,-1) or (x,y) == (x_max-1,y_max):
        return True
    return 0 <= x < x_max and 0 <= y < y_max

def one_step(old_time, old_grid, x_max, y_max):
    grid = collections.defaultdict(list)
    for (x,y), blizzards in old_grid.items():
        assert blizzards # otherwise why?
        for blizz in blizzards:
            dir = DIRS[blizz]
            x_new, y_new = (x + dir[0]) % x_max, (y + dir[1]) % y_max
            grid[(x_new, y_new)].append(blizz)
    return old_time+1, grid

def print_grid(grid, x_max, y_max, possible):
    for y in range(y_max):
        s = ""
        for x in range(x_max):
            l = grid.get((x,y), [])
            n = len(l)
            if (x,y) in possible:
                s += '@'
                assert not n
            elif n > 1:
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
