#!/usr/bin/python3

import collections
import fileinput
import sys

def solve(inp):
    bits = 0
    bit_counts = None
    for line in inp:
        if not bits:
            bits = len(line.strip())
            bit_counts = [ collections.Counter() for i in range(bits) ]
        for i in range(bits):
            bit_counts[i][line[i]] += 1

    gamma = ''
    epsilon = ''
    for i in range(bits):
        if bit_counts[i]['1'] > bit_counts[i]['0']:
            gamma += '1'
            epsilon += '0'
        else:
            gamma += '0'
            epsilon += '1'

    gamma = int(gamma, 2)
    epsilon = int(epsilon, 2)
    print(gamma * epsilon)

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
