#!/usr/bin/python3

import collections
import fileinput
import sys

def solve(inp):
    bits = 0
    data = []
    for line in inp:
        bits = len(line.strip())
        data.append(line.strip())
    oxy = rating(data, bits, lambda zero, one: one >= zero)
    co2 = rating(data, bits, lambda zero, one: one < zero)
    print(oxy * co2)

def rating(data, bits, cmp):
    res = data[:]
    for bit in range(bits):
        counts = count_bit(res, bit)
        keep = '1' if cmp(counts['0'], counts['1']) else '0'

        keepers = []
        for l in res:
            if l[bit] == keep:
                keepers.append(l)

        res = keepers
        if len(res) == 1:
            return int(res[0], 2)
        elif len(res) == 0:
            Exception('lost them all?')
    raise Exception('should not happen')
    
def count_bit(data, bit):
    counts = collections.Counter()
    for l in data:
        counts[l[bit]] += 1
    return counts

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
