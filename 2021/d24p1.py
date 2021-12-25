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
    print(find_digits(insts_by_digit))


# Returns the highest set of digits where run(insts_by_digit[-1])['z'] == target_z
@functools.cache
def find_digits(insts_by_digit, target_z=0):
    print(f"find_digits({len(insts_by_digit)}, {target_z})", flush=True)
    digits_left = len(insts_by_digit)
    digit_insts = insts_by_digit[-1]
    for try_w in range(9, 0, -1):
        for try_z in (range(0, 626) if len(insts_by_digit) > 1 else [0]):
            regs = run(digit_insts, [try_w], regs = {'z': try_z, 'w': 0, 'x': 0, 'y': 0})
            if regs['z'] != target_z:
                continue

            if digits_left == 1:
                return [try_w]

            # At this point we know that if we start with w=try_w /
            # z=try_z we should get to z=0, so let's see what starting
            # digits will get us there.
            starting_digits = find_digits(insts_by_digit[:-1], try_z)
            if not starting_digits:
                # can't find one?
                continue
            return starting_digits + [try_w]

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
