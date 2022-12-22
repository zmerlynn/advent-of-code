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

    def distance():
        return abs(find_value(monkeys, monkeys['root'][1])-find_value(monkeys, monkeys['root'][2]))

    d = 0
    center = 3247317268369
    for humn in range(center-100, center+100):
        monkeys['humn'] = humn
        last_d = d
        d = distance()
        print(humn, d, d-last_d)
        if d == 0:
            print(humn)
            return

def find_value(monkeys, name):
    val = monkeys[name]
    if type(val) is int:
        return val

    op, a, b = val
    a = find_value(monkeys, a)
    b = find_value(monkeys, b)
    # print(a,b)

    if op == '+':
        val = a+b
    elif op == '-':
        val = a-b
    elif op == '*':
        val = a * b
    elif op == '/':
        val = a // b
    # monkeys[name] = val
    return val

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
