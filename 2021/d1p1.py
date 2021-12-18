#!/usr/bin/python3

import fileinput
import sys

def solve(inp):
    data = []
    for line in inp:
        data.append(int(line))

    print(increases(data))

def increases(data):
    n = 0
    for (t0, t1) in zip(data, data[1:]):
        if t0 < t1:
            n += 1
    return n
        
if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
