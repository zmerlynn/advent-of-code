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
    msg(l.strip())

def msg(s):
    for i in range(4, len(s)):
        if len({c for c in s[i-14:i]}) == 14:
            print(i)
            return i

def tests():
    assert msg("mjqjpqmgbljsphdztnvjfqwrcgsmlb") == 19
    assert msg("bvwbjplbgvbhsrlpgdmjqwftvncz") == 23
    assert msg("nppdvjthqldpwncqszvftbrmjlhg") == 23
    assert msg("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 29
    assert msg("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 26
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
