#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

def solve(inp):
    arr = [ [ int(c) for c in l.strip() ] for l in inp ]
    newarr = []
    for j in range(5):
        for row in arr:
            print(row)
            newrow = []
            for i in range(5):
                newrow += [ (x - 1 + i + j) % 9 + 1 for x in row ]
            newarr.append(newrow)
    arr = newarr

    rows = len(arr)
    cols = len(arr[0])
    print(rows, cols)

    costs = [ [ 0 for c in range(cols) ] for r in range(rows) ]

    for row in range(rows):
        for col in range(cols):
            above_or_left = []
            if row > 0:
                above_or_left.append(costs[row-1][col]+arr[row][col])
            if col > 0:
                above_or_left.append(costs[row][col-1]+arr[row][col])
            costs[row][col] = min(above_or_left or [0])
    best = costs[rows-1][cols-1]
    print("best", best)

    paths = [ (0, [(0,0)]) ]
    while paths:
        newpaths = []
        for (cost, path) in paths:
            row, col = path[-1]
            if row == rows - 1 and col == cols - 1:
                if cost < best:
                    best = cost
                    print("best", best, path)
                continue
            for nr, nc in [ (row+1, col), (row, col+1) ]: # [ (row-1, col), (row+1, col), (row, col-1), (row, col+1) ]:#
                if (0 <= nr) and (nr < rows) and (0 <= nc) and (nc < cols):
                    newcost = cost + arr[nr][nc]
                    if newcost < best:
                        newpaths.append( (newcost, path + [(nr,nc)]) )
        paths = newpaths
        print(len(paths), "paths", paths)

def best(row, col, rows, cols, arr):
    if row == rows-1 and col == cols-1:
        return 0
    costs = []
    for nr, nc in [ (row-1, col), (row+1, col), (row, col-1), (row, col+1) ]:
        if (0 <= nr) and (nr < rows) and (0 <= nc) and (nc < cols):
            costs.append(arr[nr][nc] + best(nr, nc, rows, cols, arr))
    print(costs)
    return min(costs)
        
if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
