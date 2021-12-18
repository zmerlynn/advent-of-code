#!/usr/bin/python3

import collections
import fileinput
import itertools
import re
import sys

SEGMENTS_OUT = 'ABCDEFG'
DIGITS = {'ABCEFG':'0', 'CF':'1', 'ACDEG':'2', 'ACDFG':'3', 'BCDF':'4', 'ABDFG':'5', 'ABDEFG':'6', 'ACF':'7', 'ABCDEFG':'8', 'ABCDFG':'9' }

def solve(inp):
    count = 0
    for line in inp:
        input, output = line.split('|')
        input, output = input.strip().split(), output.strip().split()
        o = solve_entry(input, output)
        print(o)
        count += o
    print(count)

def solve_entry(i, o):
    for segments_in in itertools.permutations(SEGMENTS_OUT.lower()):
        map = dict( (segments_in[i], SEGMENTS_OUT[i] ) for i in range(len(SEGMENTS_OUT)) )
        if prove(i+o, map):
            out_digits = ""
            for segment_set in o:
                dig, ignored = digit(segment_set, map)
                out_digits += dig
            return int(out_digits)
    raise Exception('not found')

def prove(segment_sets, map):
    for segment_set in segment_sets:
        ignored, valid = digit(segment_set, map)
        if not valid:
            return False
    return True

def digit(segment_set, map):
    decode = ""
    for c in segment_set:
        decode += map[c]
    decode = sortstring(decode)
    if decode in DIGITS:
        return DIGITS[decode], True
    else:
        return -1, False

def sortstring(s):
    return ''.join(sorted(s))

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
