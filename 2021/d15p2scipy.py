#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

from scipy.sparse import csr_matrix,lil_matrix
from scipy.sparse.csgraph import dijkstra

def solve(inp):
    arr = [ [ int(c) for c in l.strip() ] for l in inp ]
    arr = expand(arr, 5)

    rows = len(arr)
    cols = len(arr[0])
    nodes = rows*cols
    print(nodes, "nodes")
    graph = lil_matrix((nodes, nodes))

    for row in range(rows):
        for col in range(cols):
            from_idx = node_index(row, col, cols)
            for nr, nc in [ (row-1, col), (row+1, col), (row, col-1), (row, col+1) ]:
                if (0 <= nr) and (nr < rows) and (0 <= nc) and (nc < cols):
                    to_idx = node_index(nr, nc, cols)
                    graph[(from_idx,to_idx)] = arr[nr][nc]
    graph = graph.tocsr()

    dists = dijkstra(csgraph=graph, indices=0)
    print(dists[nodes-1])

def node_index(row, col, cols):
    return row*cols + col

def expand(arr, tiles):
    newarr = []
    for j in range(tiles):
        for row in arr:
            newrow = []
            for i in range(5):
                newrow += [ (x - 1 + i + j) % 9 + 1 for x in row ]
            newarr.append(newrow)
    return newarr

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
