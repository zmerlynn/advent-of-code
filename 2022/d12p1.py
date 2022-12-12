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

def solve(inp):
    grid = [ [ c for c in l.strip() ] for l in inp]
    rows = len(grid)
    cols = len(grid[0])

    for y in range(rows):
        for x in range(cols):
            if grid[y][x] == 'S':
                start = (x,y)
                grid[y][x] = 'a'
            elif grid[y][x] == 'E':
                end = (x,y)
                grid[y][x] = 'z'
            
            grid[y][x] = ord(grid[y][x]) - ord('a')

    assert start
    assert end

    graph = collections.defaultdict(set)
    for y in range(rows):
        for x in range(cols):
            h = grid[y][x]
            for dy, dx in [(0,1),(0,-1),(1,0),(-1,0)]:
                if y+dy >= rows or x+dx >= cols or y+dy < 0 or x+dx < 0:
                    continue
                if grid[y+dy][x+dx] - h <= 1:
                    graph[(x,y)].add( (x+dx, y+dy) )
    print(shortest_path(graph, start, end))

def shortest_path(graph, start, end):
    visited = {start: 0}
    edge = {start}
    edgedist = 0
    while end not in visited and edge:
        lastedge, edge = edge, set()
        edgedist += 1

        for v in lastedge:
            for next in graph[v]:
                if next not in visited:
                    # print(f"visited {next} len {edgelen}")
                    edge.add(next)
                    visited[next] = edgedist
        print(f"distance {edgedist}: {len(edge)} edge elements, {len(visited)} visited elements")
    assert end in visited
    return visited[end]

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
