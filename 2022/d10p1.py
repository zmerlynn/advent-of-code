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
    report_cycle = 20
    cycle = 0
    reg = 1
    total = 0
    for l in inp:
        lastreg = reg
        l = l.strip()
        if l.startswith("noop"):
            cycle += 1
            # print(f"{cycle}: noop")
        elif l.startswith("addx"):
            addend = l.split()[1]
            reg += int(addend)
            cycle += 2
            # print(f"{cycle}: addx {addend}")

        if cycle >= report_cycle:
            total += report_cycle * lastreg
            print(f"cycle, reg = {report_cycle}, {lastreg}: {report_cycle * lastreg}")
            report_cycle += 40
    print(total)

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
