#!/usr/bin/python3

import collections
import fileinput
import numpy as np
import re
import sys

def solve(inp):
    fish_at_age = [ 0 ] * 9
    for age in next(inp).strip().split(','):
        fish_at_age[int(age)] += 1
    print(fish_at_age)

    for t in range(256):
        zeroes, fish_at_age = fish_at_age[0], fish_at_age[1:]
        fish_at_age.append(zeroes) # babies
        fish_at_age[6] += zeroes
        print(t+1, sum(fish_at_age), fish_at_age)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
