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


    scores = [ [1] * cols for _ in range(rows) ]
    print(scores)
    def viewing_distance(trees, h):
        for i in range(len(trees)):
            if trees[i] >= h:
                return i+1
        return len(trees)

    for row in range(rows):
        # from left
        in_direction = []
        for col in range(cols):
            scores[row][col] *= viewing_distance(in_direction, trees[row][col])
            in_direction.insert(0, trees[row][col])

        # from right
        in_direction = []
        for col in range(cols-1, -1, -1):
            scores[row][col] *= viewing_distance(in_direction, trees[row][col])
            in_direction.insert(0, trees[row][col])

    for col in range(cols):
        # from top
        in_direction = []
        for row in range(rows):
            scores[row][col] *= viewing_distance(in_direction, trees[row][col])
            in_direction.insert(0, trees[row][col])

        # from bottom
        in_direction = []
        for row in range(rows-1, -1, -1):
            scores[row][col] *= viewing_distance(in_direction, trees[row][col])
            in_direction.insert(0, trees[row][col])

    print(max([max(row) for row in scores]))

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
