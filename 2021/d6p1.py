#!/usr/bin/python3

import collections
import fileinput
import numpy as np
import re
import sys

def solve(inp):
    fishes = [ int(s) for s in next(inp).strip().split(',') ]
    for t in range(80):
        fishes_next = []
        new_fishes = 0
        for fish in fishes:
            fish -= 1
            if fish >= 0:
                fishes_next.append(fish)
            else:
                fishes_next.append(6)
                new_fishes += 1
        fishes = fishes_next + [8]*new_fishes
        print(t+1, len(fishes))

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
