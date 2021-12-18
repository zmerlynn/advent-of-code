#!/usr/bin/python3

import collections
import fileinput
import numpy as np
import re
import sys

SEGMENTS = 'ABCDEFG'
DIGITS = ['ABCEFG', 'CF', 'ACDEG', 'ACDFG', 'BCDF', 'ABDFG', 'ABDEFG', 'ACF', 'ABCDEFG', 'ABCDFG']

def solve(inp):
    count = 0
    for line in inp:
        input, output = line.split('|')
        input, output = input.strip().split(), output.strip().split()
        for segments in output:
            if len(segments) not in [5, 6]:
                print(segments)
                count += 1
    print(count)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
