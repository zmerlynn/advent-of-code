#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

def solve(inp):
    strings = []
    for l in inp:
        strings.append(l.strip())

    max_mag = 0
    for i in range(len(strings)):
        for j in range(len(strings)):
            if i == j:
                continue
            sn = [eval(strings[i]), eval(strings[j])]
            reduce(sn)
            max_mag = max(max_mag, magnitude(sn))
    print(max_mag)

def read_snailfish(s):
    return eval(s)

def reduce(sn):
    while True:
        path = needs_explode(sn, [])
        if path:
            explode(sn, path)
            continue

        path = needs_split(sn, [])
        if path:
            spl(sn, path)
            continue

        break
    return sn

def needs_explode(sn, path):
    if len(path) == 4:
        return path
    left, right = sn
    if type(left) is list:
        p = needs_explode(left, path[:] + [0])
        if p:
            return p
    if type(right) is list:
        p = needs_explode(right, path[:] + [1])
        if p:
            return p
    return None

def needs_split(sn, path):
    left, right = sn
    if type(left) is list:
        p = needs_split(left, path[:] + [0])
        if p:
            return p
    if type(left) is int and left >= 10:
        return path[:] + [0]

    if type(right) is list:
        p = needs_split(right, path[:] + [1])
        if p:
            return p
    if type(right) is int and right >= 10:
        return path[:] + [1]

    return None

def explode(sn, path):
    tree = sn_at(sn, path[:-1])
    exl, exr = tree[path[-1]]
    tree[path[-1]] = 0
    # print("after zero", sn, exl, exr, path)
    add(sn, path, 0, exl)
    # print("after left add", sn)
    add(sn, path, 1, exr)
    # print("AFTER EXPLODE", sn)

def sn_at(sn, path):
    for lr in path:
        sn = sn[lr]
    return sn

def add(sn, path, lr, n):
    # print("add at", path, "lr", lr, "n", n)
    rl = 0 if lr == 1 else 1

    while path and path[-1] == lr:
        path = path[:-1]
    if not path:
        return
    
    path = path[:-1]+[lr]
    tree = sn_at(sn, path)
    # print("redescent", tree, path)
    went_down = False
    while type(tree) is list:
        went_down = True
        tree = tree[rl]
        path = path[:] + [rl]
    # print("redescent end", tree, path)

    tree = sn_at(sn, path[:-1])
    # print("tree ", tree, rl if went_down else lr)
    tree[rl if went_down else lr] += n

def spl(sn, path):
    tree = sn_at(sn, path[:-1])
    # print("tree", tree)
    n = tree[path[-1]]
    tree[path[-1]] = [ n // 2, n - (n // 2) ]
    # print("AFTER SPLIT  ", sn)

def magnitude(sn):
    if type(sn) is int:
        return sn
    left, right = sn
    return 3*magnitude(left) + 2*magnitude(right)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
