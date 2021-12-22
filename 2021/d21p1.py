#!/usr/bin/python3

import collections
import fileinput
import functools
import itertools
import math
import re
import sys

def solve(inp):
    pos = [int(l.strip().split(":")[1])-1 for l in inp if l.strip() != ""]
    dice = itertools.cycle(range(1,101))
    score = [ 0 for p in pos ]

    player = 0
    num_rolls = 0
    while max(score) < 1000:
        rolls = [next(dice) for i in range(3)]
        num_rolls += 3
        newpos = (pos[player] + sum(rolls)) % 10
        print("player", player+1, "rolled ", rolls, "total", sum(rolls), "moving from ", pos[player]+1, "to ", newpos+1)
        pos[player] = newpos

        score[player] += pos[player]+1
        print("scores: ", score)
        player = (player + 1) % len(score)

    print(num_rolls * min(score))

if __name__ == "__main__":
    solve(fileinput.input(sys.argv[1]))
