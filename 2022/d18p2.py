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

DIRS=[(1,0,0),(-1,0,0),(0,1,0),(0,-1,0),(0,0,1),(0,0,-1)]

def solve(inp):
    coords = [ tuple([int (s) for s in l.strip().split(',')]) for l in inp ]
    min_x = min([coord[0] for coord in coords])-2
    max_x = max([coord[0] for coord in coords])+2
    min_y = min([coord[1] for coord in coords])-2
    max_y = max([coord[1] for coord in coords])+2
    min_z = min([coord[2] for coord in coords])-2
    max_z = max([coord[2] for coord in coords])+2
    print(min_x, max_x, min_y, max_y, min_z, max_z)
    coords = set(coords)
    assert (0,0,0) not in coords

    edge = {(min_x, min_y, min_z)}
    outside = {(0,0,0)}
    while edge:
        new_edge = set()
        for coord in list(edge):
            for dir in DIRS:
                v = vector_add(coord, dir)
                if v[0] < min_x or v[0] > max_x or v[1] < min_x or v[1] > max_y or v[2] < min_x or v[2] > max_z:
                    continue
                if v in outside or v in edge or v in coords:
                    continue

                new_edge.add(v)
                outside.add(v)
        # print(new_edge)
        edge = new_edge

    surface = 0
    for coord in list(coords):
        for dir in DIRS:
            v = vector_add(coord, dir) 
            if v in outside:
                surface += 1

    print(surface)

def vector_add(v1, v2):
    return ( v1[0] + v2[0], v1[1] + v2[1], v1[2] + v2[2] )

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
