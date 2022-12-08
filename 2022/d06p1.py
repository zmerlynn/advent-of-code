#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import re
import sys

def solve(inp):
    l = next(inp)
    marker(l.strip())

def marker(s):
    for i in range(4, len(s)):
        if len({c for c in s[i-4:i]}) == 4:
            print(i)
            return i

def tests():
    assert marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 7
    assert marker("bvwbjplbgvbhsrlpgdmjqwftvncz") == 5
    assert marker("nppdvjthqldpwncqszvftbrmjlhg") == 6
    assert marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 10
    assert marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 11
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
