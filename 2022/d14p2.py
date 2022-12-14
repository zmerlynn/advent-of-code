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
    grid = {}
    maxy = 0
    minx = 100000000
    maxx = 0
    for l in inp:
        points = l.strip().split(' -> ')
        points = [ eval("(" + p + ")") for p in points ]
        for a,b in zip(points, points[1:]):
            print(a,b)
            if a[0] == b[0]:
                lo, hi = min(a[1], b[1]), max(a[1], b[1])
                for y in range(lo, hi+1):
                    grid[ (a[0], y) ] = '#'
                    maxy = max(maxy, y)
            else:
                assert a[1] == b[1]
                lo, hi = min(a[0], b[0]), max(a[0], b[0])
                for x in range(lo, hi+1):
                    grid[ (x, b[1]) ] = '#'
                    maxx = max(maxx, x)
                    minx = min(minx, x)

    sands = 0
    while simulate(grid, maxy):
        sands += 1
        # print_grid(grid, minx, maxx, maxy)
    print(sands)

def simulate(grid, maxy):
    sand_pos = (500, 0)
    if sand_pos in grid:
        return False

    while True:
        if sand_pos[1] == maxy+1:
            grid[sand_pos] = 'o'
            return True

        liberty = False
        for dir in [ (0,1), (-1,1), (1,1) ]:
            maybe = vector_add(sand_pos, dir)
            if maybe not in grid:
                liberty = True
                sand_pos = maybe
                break
        if not liberty:
            grid[sand_pos] = 'o'
            return True

def print_grid(grid, minx, maxx, maxy):
    for y in range(0, maxy+1):
        s = ''
        for x in range(minx, maxx+1):
            s += grid.get((x,y), ',')
        print(s)

def vector_add(v1, v2):
    return ( v1[0] + v2[0], v1[1] + v2[1] )

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
