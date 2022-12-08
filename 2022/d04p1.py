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
    count = 0
    for l in inp:
        ranges = l.strip().split(',')
        rangeset = lambda x, y: set(range(x, y+1))
        a, b = [ rangeset(*[int(x) for x in r.split('-')]) for r in ranges ]
        if a.issubset(b) or b.issubset(a):
            count += 1
    print(count)

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
