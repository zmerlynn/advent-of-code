#!/usr/bin/python3

import collections
import fileinput
import functools
import heapq
import itertools
import math
import re
import sys

REGISTERS=["w", "x", "y", "z"]
DIGITS=14

def solve(inp):
    insts = read_program(inp)

    input_indexes = []
    for inst_idx in range(len(insts)):
        if insts[inst_idx][0] == "inp":
            input_indexes.append(inst_idx)

    digit_insts_ranges = zip(input_indexes, input_indexes[1:] + [len(insts)])
    insts_by_digit = tuple([ tuple(insts[start:fin]) for start, fin in digit_insts_ranges ])
    assert(all([len(digit_insts) == 18 for digit_insts in insts_by_digit]))

    fixed = [] # for testing
    start_z = run(insts[:input_indexes[len(fixed)]], fixed)['z']
    print(find_digits(insts_by_digit, set(), pos=len(fixed), start_z=start_z))


def find_digits(insts_by_digit, visited, pos, start_z):
    token = (pos, start_z)
    if token in visited:
        # We don't need to record anything but failures, success is immediate.
        return None
    visited.add(token)

    if pos < 3:
        print(f"find_digits({pos}, {start_z})", flush=True)
    digit_insts = insts_by_digit[pos]
    last_digit = pos == len(insts_by_digit)-1
    # for try_w in reversed(range(1, 10)):
    for try_w in range(1, 10):
        end_z = run(digit_insts, [try_w], regs = {'z': start_z, 'w': 0, 'x': 0, 'y': 0})['z']
        # print(pos, start_z, "+ digit", try_w, " => ", end_z)

        if last_digit:
            if end_z == 0:
                return (try_w,)
            else:
                continue

        found = find_digits(insts_by_digit, visited, pos+1, end_z)
        if found:
            digits = tuple([try_w] + list(found))
            print(f"find_digits({pos}, {start_z}) => {digits}", flush=True)
            return digits

    return None

def run(insts, inputs, regs=None):
    inputs = inputs[:]
    regs = regs or { reg: 0 for reg in REGISTERS }
    register_or_scalar = lambda s: regs[s] if s in REGISTERS else int(s)
    for inst in insts:
        store_reg = inst[1]
        if inst[0] == "inp":
            regs[store_reg] = int(inputs.pop(0))
        elif inst[0] == "add":
            regs[store_reg] = regs[store_reg] + register_or_scalar(inst[2])
        elif inst[0] == "mul":
            regs[store_reg] = regs[store_reg] * register_or_scalar(inst[2])
        elif inst[0] == "div":
            regs[store_reg] = regs[store_reg] // register_or_scalar(inst[2])
        elif inst[0] == "mod":
            regs[store_reg] = regs[store_reg] % register_or_scalar(inst[2])
        elif inst[0] == "eql":
            regs[store_reg] = 1 if regs[store_reg] == register_or_scalar(inst[2]) else 0
    return regs

def read_program(lines):
    return tuple([ tuple(l.strip().split()) for l in lines if not l.startswith('#') ])

def tests():
    insts = read_program(
        '''
inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
        '''.strip().split('\n'))
    print( run(insts, [11]) )
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
