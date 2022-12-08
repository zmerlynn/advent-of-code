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
    bags = []
    for l in inp:
        bags.append(l.strip())
        if len(bags) == 3:
            p = prio(bags)
            print(p)
            s += p
            bags = []
    print(s)

def prio(bags):
    print(bags)
    bags = [ { i for i in bag } for bag in bags ]
    print(bags)
    i = bags[0].intersection(bags[1]).intersection(bags[2])
    i = list(i)[0]
    if ord(i) >= ord('a'):
        return ord(i)-ord('a')+1
    return ord(i)-ord('A')+27



def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
