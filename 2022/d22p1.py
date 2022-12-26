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

TURN_MAP = {
    'R': {
        (1,0): (0,1),
        (0,1): (-1,0),
        (-1,0): (0,-1),
        (0,-1): (1,0)
    },
    'L': {
        (1,0): (0,-1),
        (0,-1): (-1,0),
        (-1,0): (0,1),
        (0,1): (1,0)
    }
}

FACING_VALUE = {
        (1,0): 0,
        (0,1): 1,
        (-1,0): 2,
        (0,-1): 3,
}

grid = {}
x_max = 0
y_max = 0

def solve(inp):
    global grid
    global x_max
    global y_max

    y = 0
    beginning = None
    for l in inp:
        l = l.rstrip()
        if not l:
            break

        for x in range(len(l)):
            if l[x] == '.':
                grid[(x,y)] = False
                if not beginning:
                    beginning = (x,y)
            elif l[x] == '#':
                grid[(x,y)] = True
            # skip spaces

        x_max = max(x_max, len(l))
        y += 1
    y_max = y

    directions = next(inp).strip()
    assert(directions)
    directions = re.split(r'([RL])', directions)

    # print(grid)
    # print(directions)

    assert beginning

    loc = beginning
    facing = (1,0)
    print(f"loc: {loc}, facing: {facing}")

    for direction in directions:
        if direction.isnumeric():
            for _ in range(int(direction)):
                new_loc = step(loc, facing)
                if loc == new_loc:
                    break
                loc = new_loc
        else:
            facing = TURN_MAP[direction][facing]
        
        print(f"after '{direction}' loc: {loc}, facing: {facing}")

    print((loc[0]+1)*4 + (loc[1]+1)*1000 + FACING_VALUE[facing])

@functools.cache
def step(loc, facing):
    global grid

    new_loc = ((loc[0]+facing[0]) % x_max, (loc[1]+facing[1]) % y_max)
    while new_loc not in grid:
        new_loc = ((new_loc[0]+facing[0]) % x_max, (new_loc[1]+facing[1]) % y_max)
    assert new_loc in grid

    if grid[new_loc]:
        return loc # face meet rock
    return new_loc # empty spot on grid
        

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
