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
    reg = 1
    reg_record = []
    for l in inp:
        l = l.strip()
        if l.startswith("noop"):
            reg_record.append(reg)
        elif l.startswith("addx"):
            reg_record.append(reg)
            reg_record.append(reg)
            addend = l.split()[1]
            reg += int(addend)
    reg_record = [ reg_record[i:i+40] for i in range(0, len(reg_record), 40) ]
    for row in reg_record:
        s = ''
        for x in range(len(row)):
            s += '#' if abs(row[x] - x) <= 1 else '.'
        print(s)

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
