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

SIZE=0
KIDS=1

def solve(inp):
    root = (0, {})
    cwd = root
    for l in inp:
        l = l.strip()
        if l.startswith("$ cd /"):
            cwd = root
        elif l.startswith("$ cd "):
            dir = l.split(' ')[2]
            assert dir in cwd[KIDS]
            # cwd[KIDS][dir] = (0, {'..': cwd})
            cwd = cwd[KIDS][dir]
        elif l.startswith("$ ls"):
            pass
        elif l.startswith("dir"):
            cwd[KIDS][l.split(' ')[1]] = (0, {'..': cwd})
        else:
            sz, nm = l.split(' ')
            cwd[KIDS][nm] = (int(sz), )
    total, _ = sumTree(root)
    need = total + 30000000 - 70000000
    print(f"need = {need}")
    _, minatleast = sumTree(root, need)
    print(f"least = {minatleast}")

def printTree(cwd, indent=0):
    for name, meta in cwd[KIDS].items():
        if name == '..':
            continue
        if len(meta) == 1:
            thing = f"{name} (file, size={meta[SIZE]})"
        else:
            thing = f"{name} (dir, total size={meta[SIZE]})"
        print(' ' * indent, thing)
        if len(meta) == 2:
            printTree(meta, indent=indent+2)

def sumTree(cwd, atleast=0):
    total = 0
    minatleast = 70000000
    for name, meta in cwd[KIDS].items():
        if name == '..':
            continue
        total += meta[SIZE]
        if len(meta) == 2:
            (subtotal, subminatleast) = sumTree(meta, atleast)
            total += subtotal
            minatleast = min(minatleast, subminatleast)
    if total >= atleast and total < minatleast:
        # print(total)
        minatleast = total
    return (total, minatleast)

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
