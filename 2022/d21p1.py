#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import multiprocessing
import re
import sys
import json

def solve(inp):
    monkeys = {}
    for l in inp:
        name, operation = l.strip().split(": ")
        if operation.isnumeric():
            monkeys[name] = int(operation)
        else:
            monkeys[name] = (operation[5], operation[:4], operation[7:])
    print(find_value(monkeys, 'root'))

def find_value(monkeys, name):
    val = monkeys[name]
    if type(val) is int:
        return val

    op, a, b = val
    a = find_value(monkeys, a)
    b = find_value(monkeys, b)
    print(a,b)

    if op == '+':
        val = a+b
    elif op == '-':
        val = a-b
    elif op == '*':
        val = a * b
    elif op == '/':
        val = a // b
    monkeys[name] = val
    return val

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
