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

SIGNATURE_SIZE=100

pieces = [
    [
        [1, 1, 1, 1],
    ],
    [
        [0, 1, 0],
        [1, 1, 1],
        [0, 1, 0],
    ],
    [
        [1, 1, 1],
        [0, 0, 1],
        [0, 0, 1],
    ],
    [
        [1],
        [1],
        [1],
        [1],
    ],
    [
        [1, 1],
        [1, 1],
    ],
]

def solve(inp):
    jets = next(inp).strip()
    raster = collections.defaultdict(lambda: [0]*7)
    raster[0] = [1]*7 # start with floor; raster is non-empty rows in ascending order
    first_empty = 1

    piece_iter = itertools.cycle(pieces)
    jet_iter = itertools.cycle(jets)

    rocks = 0
    want_rocks = 1000000000000
    seen = {}
    cycle_height = 0
    cycle_rocks = 0
    finishing = False
    while True:
        piece = next(piece_iter)
        y = first_empty+3
        x = 2

        while True:
            jet = next(jet_iter)

            # jet push
            if jet == '<' and x != 0 and not intersects(raster, piece, x-1, y):
                # print("left")
                x -= 1
            elif jet == '>' and x + len(piece[0]) < 7 and not intersects(raster, piece, x+1, y):
                # print("right")
                x += 1
            
            # down
            if not intersects(raster, piece, x, y-1):
                # print("down")
                y -= 1
            else:
                break

        place(raster, piece, x, y)
        first_empty = max(first_empty, y+len(piece))

        rocks += 1
        if rocks + cycle_rocks >= want_rocks:
            print(first_empty + cycle_height -1)
            return

        if first_empty <= SIGNATURE_SIZE:
            continue
        if not cycle_rocks:
            sig = tuple([tuple(raster[y]) for y in range(first_empty-SIGNATURE_SIZE, first_empty)])
            if sig in seen:
                last_rocks, last_first_empty = seen[sig]
                rocks_per_cycle = rocks - last_rocks
                height_per_cycle = first_empty-last_first_empty
                print(f"cycle found at: {rocks}, height {first_empty}, rocks/cycle: {rocks_per_cycle}, height/cycle: {height_per_cycle}")

                finishing = True
                cycles = (want_rocks - rocks) // rocks_per_cycle
                cycle_rocks = cycles * rocks_per_cycle
                cycle_height = cycles * height_per_cycle

            seen[sig] = (rocks, first_empty)

    # print_raster(raster, first_empty)

def intersects(raster, piece, x, y):
    for y_in_piece in reversed(range(len(piece))):
        for x_in_piece in range(len(piece[0])):
            if piece[y_in_piece][x_in_piece] and raster[y+y_in_piece][x+x_in_piece]:
                return True
    return False

def place(raster, piece, x, y):
    for y_in_piece in reversed(range(len(piece))):
        for x_in_piece in range(len(piece[0])):
            if piece[y_in_piece][x_in_piece]:
                assert not raster[y+y_in_piece][x+x_in_piece]
                raster[y+y_in_piece][x+x_in_piece] = 1

def signature(rows):
    return tuple(sum(rows, []))

def print_raster(raster, first_empty):
    for y in reversed(range(1,first_empty)):
        s = '|'
        for p in raster[y]:
            s += '#' if p else '.'
        s += '|'
        print(s)
    print("+-------+")

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
