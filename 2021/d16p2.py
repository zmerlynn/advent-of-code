#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

HEX_TO_BINARY={
    "0": "0000",
    "1": "0001",
    "2": "0010",
    "3": "0011",
    "4": "0100",
    "5": "0101",
    "6": "0110",
    "7": "0111",
    "8": "1000",
    "9": "1001",
    "A": "1010",
    "B": "1011",
    "C": "1100",
    "D": "1101",
    "E": "1110",
    "F": "1111",
}
LITERAL='literal'
OPERATOR='operator'

def solve(inp):
    l = next(inp)
    bits = ""
    for c in l.strip():
        bits += HEX_TO_BINARY[c]
    packet = read_packet(iter(bits))
    print(eval(packet))

def read_packet(it):
    v = bits_to_int(it, 3)
    t = bits_to_int(it, 3)
    if t == 4:
        return (v, LITERAL, t, read_literal(it))
    return (v, OPERATOR, t, read_operator(it))

def get_bits(it, n):
    bits = ""
    for i in range(n):
        bits += next(it)
    print(bits)
    return bits
    
def bits_to_int(it, n):
    return int(get_bits(it, n), 2)

def read_literal(it):
    print("literal")
    go = 1
    n = 0
    while go:
        go = bits_to_int(it, 1)
        n = bits_to_int(it, 4) + 16*n
    return n

def read_operator(it):
    lt = get_bits(it, 1)
    print("operator", lt)
    if lt == "0":
        sublen = bits_to_int(it, 15)
        print("  ", sublen, " bits of subpackets")
        subs = get_bits(it, sublen)
        print("  subpackets:", subs)
        subit = iter(subs)
        packets = []
        while True:
            try:
                packets.append(read_packet(subit))
            except StopIteration:
                break
        return packets

    np = bits_to_int(it, 11)
    packets = []
    for i in range(np):
        print("  ", np, " subpackets")
        packets.append(read_packet(it))
    return packets

def eval(packet):
    print(packet)
    v, ts, t, c = packet
    if ts == LITERAL:
        return c

    values = []
    for p in c:
        values.append(eval(p))

    if t == 0:
        return sum(values)
    if t == 1:
        return math.prod(values)
    if t == 2:
        return min(values)
    if t == 3:
        return max(values)
    if t == 5:
        return 1 if values[0] > values[1] else 0
    if t == 6:
        return 1 if values[0] < values[1] else 0
    if t == 7:
        return 1 if values[0] == values[1] else 0
    raise Exception('moo')

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
