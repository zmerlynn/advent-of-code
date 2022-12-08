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
    stacks = [ [] for _ in range(10) ]
    for l in inp:
        if l.startswith(" 1"):
            break
        l = l.rstrip()
        row = [l[c] for c in range(1, len(l), 4)]
        for c in range(len(row)):
            if row[c] != ' ':
                stacks[c+1].insert(0, row[c])
    next(inp)
    print(stacks)

    for l in inp:
        words = l.split()
        cnt, f, t = int(words[1]), int(words[3]), int(words[5])
        for _ in range(cnt):
            stacks[t].append(stacks[f].pop())
        print(stacks)

    s = ""
    for stack in stacks:
        if len(stack):
            s += stack.pop()
    print(s)

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
