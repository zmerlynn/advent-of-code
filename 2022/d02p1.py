#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import re
import sys

defeats = {
    'X': 'C',
    'Z': 'B',
    'Y': 'A',
}

points = {
    'A': 1,
    'X': 1,
    'B': 2,
    'Y': 2,
    'C': 3,
    'Z': 3,
}

def solve(inp):
    sum = 0
    for l in inp:
        them, us = l.strip().split()
        score = points[us]
        if points[us] == points[them]:
            score += 3
        elif defeats[us] == them:
            score += 6
        sum += score
    print(sum)

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
