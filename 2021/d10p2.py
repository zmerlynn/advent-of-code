#!/usr/bin/python3

import collections
import fileinput
import itertools
import math
import re
import sys

def solve(inp):
    scores = []
    for line in inp:
        s = score(line.strip())
        if s != 0:
            scores.append(s)
    scores.sort()
    print(scores)
    print(scores[len(scores)//2])

PAIRS={ '(': ')', '[': ']', '{': '}', '<': '>' }

SCORE_ADD= { ')': 1, ']': 2, '}': 3, '>': 4 }
SCORE_MULT=5

def score(s):
    stack = []
    for c in s:
        if c in PAIRS:
            stack.append(c)
        elif c != PAIRS[stack.pop()]:
            return 0

    s = 0
    stack.reverse()
    for c in stack:
        s = SCORE_MULT * s + SCORE_ADD[PAIRS[c]]
    print(stack, s)
    return s

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
