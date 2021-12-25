#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import re
import sys

def solve(inp):
    grid = [ [c for c in l.strip()] for l in inp ]
    rows = len(grid)
    cols = len(grid[0])

    step = 0
    while True:
        print("Step", step)
        [ print(''.join(row)) for row in grid ]

        moves_right = []
        for row in range(rows):
            for col in range(cols):
                if grid[row][col] == '>' and grid[row][(col+1) % cols] == '.':
                    moves_right.append( (row, col) )
        for (row, col) in moves_right:
            grid[row][col] = '.'
            grid[row][(col+1) % cols] = '>'

        moves_down = []
        for row in range(rows):
            for col in range(cols):
                if grid[row][col] == 'v' and grid[(row+1) % rows][col] == '.':
                    moves_down.append( (row, col) )
        for (row, col) in moves_down:
            grid[row][col] = '.'
            grid[(row+1) % rows][col] = 'v'

        step += 1
        if not moves_down and not moves_right:
            break

    print("Step", step)

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
