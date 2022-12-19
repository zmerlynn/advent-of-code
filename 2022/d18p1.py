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
    coords = set([ tuple([int (s) for s in l.strip().split(',')]) for l in inp ])

    surface = 0
    for coord in list(coords):
        for dir in [(1,0,0),(-1,0,0),(0,1,0),(0,-1,0),(0,0,1),(0,0,-1)]:
            if vector_add(coord, dir) not in coords:
                surface += 1
    print(surface)

def vector_add(v1, v2):
    return ( v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2] )

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
