#!/usr/bin/python3

import collections
import fileinput
import itertools
import math
import re
import sys

SIZE=10

def solve(inp):
    edges = [ l.strip().split('-') for l in inp ]
    graph = collections.defaultdict(list)
    for v1, v2 in edges:
        graph[v1].append(v2)
        graph[v2].append(v1)
    print(graph)
    print(paths(graph, ['start']))

def paths(graph, path):
    at = path[-1]
    if at == 'end':
        print(path)
        return 1

    sum = 0
    for option in graph[at]:
        print(path, option)
        if viable(path, option):
            sum += paths(graph, path[:] + [option])
    return sum

def viable(path, option):
    if option.isupper():
        return True
    if option not in path:
        return True
    if option == 'start':
        return False

    cnt = collections.Counter(path)
    retraversed = False
    for v, c in cnt.items():
        if not v.isupper() and c > 1:
            retraversed = True
    return not retraversed

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
