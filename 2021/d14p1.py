#!/usr/bin/python3

import collections
import fileinput
import itertools
import math
import re
import sys

def solve(inp):
    start = next(inp).strip()
    next(inp)
    rules = {}
    for l in inp:
        pair, ignored, middle = l.strip().split()
        rules[ (pair[0], pair[1]) ] =  middle 
    print(rules)

    edge_counts = collections.Counter()
    for a,b in zip(start, start[1:]):
        edge_counts[ (a,b) ] += 1
    print_elem_counts(0, edge_counts, start[0], start[-1])

    for steps in range(40):
        new_edge_counts = collections.Counter()
        for edge in edge_counts:
            cnt = edge_counts[edge]
            m = rules[edge]
            a,b = edge
            new_edge_counts[(a,m)] += cnt
            new_edge_counts[(m,b)] += cnt
        edge_counts = new_edge_counts
        print_elem_counts(steps+1, edge_counts, start[0], start[-1])

def print_elem_counts(steps, edge_counts, start, end):
    elems = collections.Counter()
    elems[start] = 1
    elems[end] = 1
    for edge in edge_counts:
        cnt = edge_counts[edge]
        elems[edge[0]] += cnt
        elems[edge[1]] += cnt
    for elem in elems:
        elems[elem] //= 2
    common = elems.most_common()
    print(steps, common[0][1]-common[-1][1])
        
if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
