#!/usr/bin/python3

import collections
import fileinput
import itertools
import math
import re
import sys

CLIFF=9
def solve(inp):
    arr = []
    cols = 0
    for line in inp:
        row = [ int(c) for c in line.strip() ]
        if not cols:
            cols = len(row)
        elif len(row) != cols:
            raise Exception('uneven input?')
        arr.append([CLIFF] + row + [CLIFF])
    rows = len(arr)
    arr = [[CLIFF] * (cols+2)] + arr + [[CLIFF] * (cols+2)]

    basin_sizes = []
    for row in range(1,rows+1):
        for col in range(1, cols+1):
            if arr[row][col] < min(arr[row-1][col], arr[row+1][col], arr[row][col-1], arr[row][col+1]):
                pt = (row, col)
                sz = basin_size(arr, pt)
                print(pt, sz)
                basin_sizes.append(sz)
    print(math.prod(sorted(basin_sizes)[-3:]))

def basin_size(arr, pt):
    basin = {pt}
    while True:
        print(basin)
        new_basin = set()
        for (r, c) in basin:
            for (row, col) in [ (r,c),(r-1,c),(r+1,c),(r,c-1),(r,c+1) ]:
                if arr[row][col] < CLIFF:
                    new_basin.add( (row,col) )
        if not (new_basin - basin):
            break
        basin = new_basin
    return len(basin)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
