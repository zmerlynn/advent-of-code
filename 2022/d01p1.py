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
    calories = []

    current = 0
    for l in inp:
        l = l.strip()
        if not l:
            if current:
                calories.append(current)
            current = 0
            continue
        current += int(l)
    print(max(calories))
        

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
