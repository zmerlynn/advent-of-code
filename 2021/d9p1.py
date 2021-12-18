#!/usr/bin/python3

import collections
import fileinput
import itertools
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

    risk_sum = 0
    for row in range(1,rows+1):
        for col in range(1, cols+1):
            if arr[row][col] < min(arr[row-1][col], arr[row+1][col], arr[row][col-1], arr[row][col+1]):
                height = arr[row][col]
                print( (row,col), height)
                risk_sum += height + 1
    print(risk_sum)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
