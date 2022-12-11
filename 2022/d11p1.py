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
    lines = list(inp)
    records = [ lines[i:i+6] for i in range(0, len(lines), 7) ]
    monkeys = [ Monkey(record) for record in records ]
    for monkey in monkeys:
        print(monkey)

    for round in range(20):
        for monkey in monkeys:
            monkey.round(monkeys)
        inspections = [ monkey.inspections for monkey in monkeys ]
        inspections = list(reversed(sorted(inspections)))
        monkey_business = inspections[0] * inspections[1]
        print(f"== Round {round+1}, monkey business = {monkey_business}")
        for monkey in monkeys:
            print(monkey)

class Monkey(object):
    def __init__(self, record):
        self.items = [ int(s) for s in record[1].strip().split(':')[1].split(', ') ]
        self.operation = record[2].split(':')[1].strip()
        self.test = int(record[3].split(" by ")[1].strip())
        self.throw = {
            True: int(record[4].split(" monkey ")[1].strip()),
            False: int(record[5].split(" monkey ")[1].strip())
        }
        self.inspections = 0

    def __str__(self):
        return f"Monkey(items={self.items}, operation={self.operation}, test={self.test}, throw={self.throw}, inspections={self.inspections})"

    def round(self, monkeys):
        items = self.items
        self.items = []
        for item in items:
            worry = self.do_operation(item) // 3
            to_monkey = self.throw[worry % self.test == 0]
            monkeys[to_monkey].items.append(worry)
            self.inspections += 1
    
    def do_operation(self, item):
        op = self.operation.split()[3]
        y = self.operation.split()[4]
        if y == "old":
            y = item
        else:
            y = int(y)
        if op == '+':
            return item + y
        return item * y

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
