#!/usr/bin/python3

import collections
import fileinput
import numpy as np
import re
import sys

def solve(inp):
    crabs = [ int(s) for s in next(inp).strip().split(',') ]
    crabs = np.array(crabs)
    bounds = np.min(crabs), np.max(crabs)
    best = None
    for pos in range(bounds[0], bounds[1]):
        fuel = fuel_at(crabs, pos)
        if not best or fuel < best[0]:
            best = (fuel, pos)
    print(best)

def fuel_at(crabs, pos):
    d = np.abs(crabs-pos)
    return sum(d*(d+1)/2)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
