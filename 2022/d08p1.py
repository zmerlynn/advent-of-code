#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import re
import sys
import json

SIZE=0
KIDS=1

def solve(inp):
    trees = []
    for l in inp:
        trees.append([ int(h) for h in l.strip() ])
    cols = len(trees[0])
    rows = len(trees)

    visible = set()
    for row in range(rows):
        # from left
        h = -1
        for col in range(cols):
            if trees[row][col] > h:
                visible.add( (row, col) )
                h = trees[row][col]

        # from right
        h = -1
        for col in range(cols-1, -1, -1):
            if trees[row][col] > h:
                visible.add( (row, col) )
                h = trees[row][col]

    for col in range(cols):
        # from top
        h = -1
        for row in range(rows):
            if trees[row][col] > h:
                visible.add( (row, col) )
                h = trees[row][col]

        # from bottom
        h = -1
        for row in range(rows-1, -1, -1):
            if trees[row][col] > h:
                visible.add( (row, col) )
                h = trees[row][col]

    print(visible)
    print(len(visible))

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
