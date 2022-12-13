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

def solve(inp):
    lines = list(inp)
    pairs = [ (eval(lines[i].strip()), eval(lines[i+1].strip())) for i in range(0, len(lines), 3) ]
    # print(pairs)

    tot = 0
    for i in range(len(pairs)):
        c = compare(*pairs[i])
        if c > 0:
            tot += i+1
    print(tot)

def compare(l,r):
    if type(l) is not list and type(r) is not list:
        return r-l
    if type(l) is list and type(r) is list:
        for i in range(max(len(l), len(r))):
            if i >= len(l):
                return 1
            if i >= len(r):
                return -1
            c = compare(l[i], r[i])
            if c:
                return c
        return 0
    # one is an int, the other not
    if type(l) is not list:
        return compare([l], r)
    assert type(r) is not list
    return(compare(l, [r]))

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
