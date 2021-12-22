#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

def solve(inp):
    insts = []
    x_planes = set()
    for l in inp:
        if not l.strip():
            continue
        # on x=-20..26,y=-36..17,z=-47..7
        onoff, bounds = l.strip().split(' ')
        bounds = bounds.split(',')
        bounds = [ s.split('=')[1] for s in bounds ]
        bounds = [ tuple(s.split('..')) for s in bounds ]
        bounds = [ (int(min), int(max)+1) for (min,max) in bounds ]
        # bounds = [ (clip(min, -50, 51), clip(max, -50, 51))  for (min, max) in bounds ]
        # if any([ min >= max for (min, max) in bounds ]):
        #     continue
        prism = tuple(bounds)
        insts.append( (prism, onoff == "on") )
        x_planes.add(prism[0][0])
        x_planes.add(prism[0][1])

    insts.reverse()

    total_on = 0
    x_planes = sorted(x_planes)
    for bounds_x in zip(x_planes, x_planes[1:]):
        insts_in_bounds_x = []
        y_planes = set()
        for (inst_prism, onoff) in insts:
            if bounds_intersect(bounds_x, inst_prism[0]):
                insts_in_bounds_x.append( (inst_prism, onoff) )
                y_planes.add(inst_prism[1][0])
                y_planes.add(inst_prism[1][1])

        if len(insts_in_bounds_x) == 0:
            continue

        y_planes = sorted(y_planes)
        print("x in", bounds_x, ",", len(insts_in_bounds_x), "matching instructions, ", len(y_planes), "y-planes")

        for bounds_y in zip(y_planes, y_planes[1:]):
            insts_in_bounds_y = []
            z_planes = set()
            for (inst_prism, onoff) in insts_in_bounds_x:
                if bounds_intersect(bounds_y, inst_prism[1]):
                    insts_in_bounds_y.append( (inst_prism, onoff) )
                    z_planes.add(inst_prism[2][0])
                    z_planes.add(inst_prism[2][1])

            if len(insts_in_bounds_y) == 0:
                continue

            z_planes = sorted(z_planes)
            # print("y in", bounds_y, ",", len(insts_in_bounds_y), "matching instructions, ", len(z_planes), "z-planes")

            for bounds_z in zip(z_planes, z_planes[1:]):
                for (inst_prism, onoff) in insts_in_bounds_y:
                    if bounds_intersect(bounds_z, inst_prism[2]):
                        if onoff:
                            total_on += (bounds_x[1]-bounds_x[0])*(bounds_y[1]-bounds_y[0])*(bounds_z[1]-bounds_z[0])
                        break
    print(total_on)


def clip(x, clip_min, clip_max):
    return max(min(x, clip_max), clip_min)

def intersects(prism, test_prism):
    return all([bounds_intersect(bounds, test_bounds) for (bounds, test_bounds) in zip(prism, test_prism)])

def bounds_intersect(bounds_a, bounds_b):
    bounds_low, bounds_high = (bounds_a, bounds_b) if bounds_a[0] < bounds_b[0] else (bounds_b, bounds_a)
    return bounds_high[0] < bounds_low[1]

def tests():
    assert bounds_intersect( (-5,6), (5, 10) )
    assert bounds_intersect( (5, 10), (-5,6) )
    assert not bounds_intersect( (5, 10), (-5,5) )

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
