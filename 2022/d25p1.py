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

SNAFU_TO_INT= {
    '2': 2,
    '1': 1,
    '0': 0,
    '-': -1,
    '=': -2,
}

INT_TO_SNAFU={ c:s for s,c in SNAFU_TO_INT.items() }

def solve(inp):
    total = 0
    for l in inp:
        n = desnafu(l.strip())
        print(f"{l.strip()} => {n}")
        total += n
    print(total)
    print(snafu(total))

def desnafu(s):
    n = 0
    for c in s:
        n = n*5 + SNAFU_TO_INT[c]
    return n

def snafu(n):
    place = 1
    max_at_place = 2
    while n > max_at_place:
        place *= 5
        max_at_place = max_at_place * 5 + 2

    s = ""

    next_max = (max_at_place-2) // 5
    while True:
        print(f"n = {n}, place = {place}, s = '{s}'")
        if place == 1:
            s += INT_TO_SNAFU[n]
            break

        if n > next_max + place:
            s += INT_TO_SNAFU[2]
            n -= 2*place
        elif n > next_max:
            s += INT_TO_SNAFU[1]
            n -= place
        elif n > next_max - place:
            s += INT_TO_SNAFU[0]
        elif n > next_max - 2*place:
            s += INT_TO_SNAFU[-1]
            n += place
        else:
            s += INT_TO_SNAFU[-2]
            n += 2*place

        place //= 5
        max_at_place = next_max
        next_max = (max_at_place-2) // 5
    print(f"returning {s}")
    return s


def tests():
    tests = {
        1:              "1",
        2:              "2",
        3:             "1=",
        4:             "1-",
        5:             "10",
        6:             "11",
        7:             "12",
        8:             "2=",
        9:             "2-",
       10:             "20",
       15:            "1=0",
       20:            "1-0",
     2022:         "1=11-2",
    12345:        "1-0---0",
314159265:  "1121-1110-1=0",
    }

    for n, want_s in tests.items():
        got_s = snafu(n)
        if got_s != want_s:
            print(f"{n} => {got_s}, wanted {want_s}")

    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
