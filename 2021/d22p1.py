#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

def solve(inp):
    cubes_on = set()
    for l in inp:
        # on x=-20..26,y=-36..17,z=-47..7
        onoff, ranges = l.strip().split(' ')
        ranges = ranges.split(',')
        ranges = [ s.split('=')[1] for s in ranges ]
        ranges = [ tuple(s.split('..')) for s in ranges ]
        ranges = [ (int(min), int(max)+1) for (min,max) in ranges ]
        ranges = [ (clip(min, -50, 51), clip(max, -50, 51))  for (min, max) in ranges ]

        for x in range(*ranges[0]):
            for y in range(*ranges[1]):
                for z in range(*ranges[2]):
                    if onoff == "on":
                        cubes_on.add( (x,y,z) )
                    elif onoff == "off":
                        cubes_on.discard( (x,y,z) )

    print(len(cubes_on))

def clip(x, clip_min, clip_max):
    return max(min(x, clip_max), clip_min)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
