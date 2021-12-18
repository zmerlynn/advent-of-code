#!/usr/bin/python3

import collections
import fileinput
import itertools
import math
import re
import sys

SIZE=10

def solve(inp):
    state = [ [ int(c) for c in row.strip() ] for row in inp ]

    flashes = 0
    for steps in range(100000):
        print(steps, flashes)
        for r in range(SIZE):
            print(''.join([str(c) for c in state[r]]))
        step_flashes = next_step(state)
        if step_flashes == (SIZE*SIZE):
            print(steps+1)
            return

def next_step(state):
    # First increment by 1
    for r,c in iterstate():
        state[r][c] += 1

    # Anything >9 gets reset to 0 (flashes) and increments all adjacent
    new_flashes = True
    while new_flashes:
        new_flashes = False
        for r,c in iterstate():
            if state[r][c] > 9:
                new_flashes = True
                state[r][c] = 0
                for ar, ac in adj(r, c):
                    if state[ar][ac] != 0:
                        state[ar][ac] += 1

    flashes = 0
    for r, c in iterstate():
        if state[r][c] == 0:
            flashes += 1
    return flashes

def iterstate():
    for r in range(SIZE):
        for c in range(SIZE):
            yield( (r, c) )

def adj(r, c):
    for nr, nc in [ (r-1, c-1), (r-1, c), (r-1, c+1), (r, c-1), (r, c+1), (r+1, c-1), (r+1, c), (r+1, c+1) ]:
        if (0 <= nr) and (nr < SIZE) and (0 <= nc) and (nc < SIZE):
            yield (nr, nc)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
