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
    s = 0
    for l in inp:
        p = prio(l.strip())
        s += p
    print(s)

def prio(bags):
    l = len(bags) // 2
    bags = bags[:l], bags[l:]
    assert(len(bags[0]) == len(bags[1]))
    bags = [ { i for i in bag } for bag in bags ]
    i = bags[0].intersection(bags[1])
    i = list(i)[0]
    if ord(i) >= ord('a'):
        return ord(i)-ord('a')+1
    return ord(i)-ord('A')+27



def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
