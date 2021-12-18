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
        if option.isupper() or (option not in path):
            sum += paths(graph, path[:] + [option])
    return sum

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
