#!/usr/bin/python3

import collections
import fileinput
import itertools
import math
import re
import sys

def solve(inp):
    sum = 0
    for line in inp:
        sum += score(line.strip())
    print(sum)

PAIRS={ '(': ')', '[': ']', '{': '}', '<': '>' }
SCORES={ ')': 3, ']': 57, '}': 1197, '>': 25137 }

def score(s):
    stack = []
    for c in s:
        if c in PAIRS:
            stack.append(c)
        elif c != PAIRS[stack.pop()]:
            return SCORES[c]
    return 0

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
