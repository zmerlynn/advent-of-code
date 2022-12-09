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
    head = (0, 0)
    tail = (0, 0)
    visited = { tail }
    for l in inp:
        dir, cnt = l.strip().split()
        print(dir, cnt)
        for _ in range(int(cnt)):
            head = vector_add(head, dir_vector(dir))
            tail = catchup(head, tail)
            print(f"  {head} {tail}")
            visited.add(tail)
    print(len(visited))

def catchup(head, tail):
    xdiff = head[0] - tail[0]
    ydiff = head[1] - tail[1]
    assert xdiff <= 2 and ydiff <= 2
    if abs(xdiff) <= 1 and abs(ydiff) <= 1:
        return tail
    return vector_add(tail, (sign(xdiff), sign(ydiff)))

def vector_add(v1, v2):
    return ( v1[0] + v2[0], v1[1] + v2[1] )

def dir_vector(dir):
    if dir == 'L':
        return (-1, 0)
    if dir == 'R':
        return (1, 0)
    if dir == 'D':
        return (0, -1)
    if dir == 'U':
        return (0, 1)

def sign(x):
    if x < 0:
        return -1
    if x > 0:
        return 1
    else:
        return 0

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
