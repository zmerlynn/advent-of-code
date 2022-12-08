#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import re
import sys

rps = {
    'A': 'rock',
    'B': 'paper',
    'C': 'scissors',
}

loseto = {
    'rock': 'scissors',
    'paper': 'rock',
    'scissors': 'paper',
}
defeats = {v: k for k, v in loseto.items()}

points = {
    'rock': 1,
    'paper': 2,
    'scissors': 3,
}

def solve(inp):
    sum = 0
    for l in inp:
        themIn, outcome = l.strip().split()
        them = rps[themIn]
        if outcome == 'X': # lose
            score = points[loseto[them]]
        elif outcome == 'Y': #draw
            score = 3 + points[them]
        else: # win
            score = 6 + points[defeats[them]]
        sum += score

    print(sum)

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
