#!/usr/bin/python3

import collections
import fileinput
import itertools
import math
import re
import sys

def solve(inp):
    points = set()
    for l in inp:
        l = l.strip()
        if not l:
            break
        points.add(tuple([int(s) for s in l.split(',')]))
    # print(points)

    insts = [ tuple(l.strip().split()[2].split('=')) for l in inp ]
    insts = [ (axis, int(s)) for (axis, s) in insts ]

    for inst in insts:
        points = do_fold(points, inst)
        # print(points)
    pretty_print(points)

def do_fold(points, insts):
    if insts[0] == 'x':
        return fold_along_x(points, insts[1])
    return fold_along_y(points, insts[1])

def fold_along_x(points, x):
    newp = set()
    for (px, py) in points:
        if px > x:
            newp.add( (x-(px-x), py) )
        else:
            newp.add( (px, py) )
    return newp

def fold_along_y(points, y):
    newp = set()
    for (px, py) in points:
        if py > y:
            newp.add( (px, y-(py-y)) )
        else:
            newp.add( (px, py) )
    return newp

def pretty_print(points):
    size_x = max([ x for (x,y) in points])+1
    size_y = max([ y for (x,y) in points])+1

    for y in range(size_y):
        print(''.join(['#' if (x,y) in points else '.' for x in range(size_x)]))
    
if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
