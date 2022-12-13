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
    packets = [ [[2]], [[6]] ]
    for l in inp:
        if l.strip():
            packets.append(eval(l.strip()))
    packets.sort(key=functools.cmp_to_key(compare), reverse=True)

    for i in range(len(packets)):
        if packets[i] == [[2]]:
            first = i+1
        elif packets[i] == [[6]]:
            last = i+1
    print(first * last)

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
