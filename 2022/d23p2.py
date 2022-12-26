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

BIGNUM=1000000000

def solve(inp):
    elves = set()

    y = 0
    for l in inp:
        l = l.strip()
        for x in range(len(l)):
            if l[x] == '#':
                elves.add((x,y))
        y += 1

    proposal_order = [
        ((0,-1),{(0,-1),(1,-1),(-1,-1)}),  # N: N, NE, NW
        ((0,1),{(0,1),(1,1),(-1,1)}),      # S: S, SE, SW
        ((-1,0),{(-1,0),(-1,-1),(-1,1)}),  # W: W, NW, SW
        ((1,0),{(1,0),(1,-1),(1,1)})       # E: E, NE, SE
    ]
    all_dirs = set()
    for proposal in proposal_order:
        all_dirs.update(proposal[1])
    all_dirs = tuple(all_dirs)

    # print_grid(elves)

    for i in itertools.count():
        proposed_dest = collections.defaultdict(list) # new (x,y) => list of current positions proposing
        for x_elf, y_elf in list(elves):
            found_elves_in_dirs = set()
            for x_d, y_d in all_dirs:
                maybe_elf = (x_elf+x_d, y_elf+y_d)
                if maybe_elf in elves:
                    found_elves_in_dirs.add((x_d,y_d))
            if not found_elves_in_dirs:
                # if there are no surrounding elves, stay put
                proposed_dest[(x_elf, y_elf)].append((x_elf,y_elf))
                continue

            made_proposal = False
            for proposal in proposal_order:
                if found_elves_in_dirs.intersection(proposal[1]):
                    continue

                x_d, y_d = proposal[0]
                proposed_dest[(x_elf+x_d, y_elf+y_d)].append((x_elf,y_elf))
                made_proposal = True
                break

            if not made_proposal:
                # nothing to do, stay put?
                proposed_dest[(x_elf, y_elf)].append((x_elf,y_elf))

        new_elves = set()
        for dest, elves_proposing in proposed_dest.items():
            assert(elves_proposing) # ???
            if len(elves_proposing) == 1:
                new_elves.add(dest)
                continue

            # otherwise, reject
            for elf in elves_proposing:
                new_elves.add(elf)

        if elves == new_elves:
            break
        assert len(elves) == len(new_elves)
        elves = new_elves
        proposal_order = proposal_order[1:] + proposal_order[:1]

        # print_grid(elves)

    print(i+1)
    

def print_grid(elves):
    x_bounds, y_bounds = bounds(elves)
    for y in range(y_bounds[0], y_bounds[1]):
        s = ""
        for x in range(x_bounds[0], x_bounds[1]):
            s += '#' if (x,y) in elves else '.'
        print(s)
    print()

def bounds(elves):
    x_min, x_max = BIGNUM, -BIGNUM
    y_min, y_max = BIGNUM, -BIGNUM

    for x,y in list(elves):
        x_min, x_max = min(x_min, x), max(x_max, x)
        y_min, y_max = min(y_min, y), max(y_max, y)

    return ((x_min, x_max+1), (y_min, y_max+1))

def tests():
    print("--- tests done ----")

if __name__ == "__main__":
    tests()
    solve(fileinput.input(sys.argv[1]))
