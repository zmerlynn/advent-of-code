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

MAX_COORD=4000000

def solve(inp):
    sensors = {}
    for l in inp:
        m = re.match("Sensor at x=([^,]+), y=([^:]+): closest beacon is at x=([^,]+), y=(.*)$", l)
        coords = [ int(s) for s in m.groups() ]
        sensors[(coords[0], coords[1])] = (coords[2], coords[3])

    for row in range(MAX_COORD, 0, -1):
        found = check_row(sensors, row)
        if found:
            print(found[0] * 4000000 + found[1])
            return



def check_row(sensors, row):
    intervals = []
    for sensor, beacon in sensors.items():
        d = man_dist(sensor, beacon)
        d_to_row = abs(row - sensor[1])
        if d_to_row >= d: # >= because there are no ties
            continue
        dx = d-d_to_row # width of interval around sensor[0] (x)
        intervals.append( (sensor[0]-dx, sensor[0]+dx) ) # closed interval of banned spots

    intervals.sort()
    # print(intervals)
    merged = []
    merged.append(intervals[0])
    for interval in intervals[1:]:
        if merged[-1][0] <= interval[0] and interval[0] <= merged[-1][1]:
            merged[-1] = (merged[-1][0], max(merged[-1][1], interval[1]))
        else:
            merged.append(interval)
    for interval in merged:
        if interval[1] < MAX_COORD:
            return (interval[1]+1, row)

def man_dist(a,b):
    return abs(a[0]-b[0]) + abs(a[1]-b[1])

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
